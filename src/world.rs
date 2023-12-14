use std::{process::Output, fmt::write};

use rand::Rng;
use rayon::prelude::*;

pub mod objects;
pub mod neurons;
pub mod criteria;

// constants
use crate::settings::*;


pub enum ObjectsEnum{
    Bot(*const objects::Bot),
    BarrierBlock(*const objects::BarrierBlock)
}

// trait for all Objects
pub trait ObjectTrait{
    // pos fn for every object
    fn pos(&self)->(Dow, Dow);
    fn kind(&self) -> ObjectsEnum;
}

impl std::fmt::Debug  for dyn ObjectTrait{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "{:?}", self.pos())
    }
}



#[derive(Debug)]
pub struct World{
    //setting dimension of the world; (u8, u8)
    dim: (Dow, Dow),

    //number of bots
    n_of_bots: u16,

    // selection criteria can be found in criteria.rs
    selection_criteria: criteria::Criteria,

    // generation of the world
    generation: usize,
    time: u64, // time overall; 
    age_of_gen: u16,
    bots_alive: u16, 

    // holding of the bots and blocks etc
    pub bot_vec: Vec<objects::Bot>,
    barrier_block_vec: Vec<objects::BarrierBlock>,

    // grid with coordinates of object
    grid: Vec<Vec<objects::Block>>,

    pub neuron_lib: Vec<&'static usize>

    //
    // maybe add a vec of all generations
    //


}

// impl for thread sharing
unsafe impl Sync for World {}

impl std::fmt::Display for World{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();
        text.push_str(&format!("Bots: {}\n", self.n_of_bots));
        text.push_str(&format!("Dim: {:?}\n", self.dim));

        write!(f, "{}", text)
    }
}

impl World{
    pub fn new(dim: (Dow, Dow), n_of_bots: u16, selection_criteria: criteria::Criteria) -> Self {

        // checking input
        if dim.0 == Dow::MAX || dim.1 == Dow::MAX{panic!("dim.0/dim.1 must be smaller than Dow::Max; buffer needed")}
        if dim.0 as usize * (dim.1 as usize) < n_of_bots as usize{
            panic!("number of objects must be smaller than dim.0*dim.1")}

        // the neuron lib is a library whitch is used for the creation of the genes
            let mut neuron_lib: Vec<&usize> = Vec::new();
            neuron_lib.push(&(INPUT_NEURONS as usize));

            for _ in 0..INNER_LAYERS{
                neuron_lib.push(&INNER_NEURONS);
            }
            neuron_lib.push(&(OUTPUT_NEURONS as usize));
        //

        // the bot vec contains every bot
            let mut bot_vec: Vec<objects::Bot> = vec![];
            for i in 0..n_of_bots{
                bot_vec.push(objects::Bot::new(neurons::create_genome(&neuron_lib)));
            }
        //

        // the grid is a 2d vec with Blocks in it
        let mut grid = Vec::new();
        for y in 0..dim.1{
            let mut row = Vec::new();
            for x in 0..dim.0{
                row.push(objects::Block::new(None, x, y));
            }
            grid.push(row);
        }

        World { dim,
                n_of_bots,
                selection_criteria,
                generation: 0,
                time: 0,
                age_of_gen: 0,
                bots_alive: 0,
                bot_vec,
                barrier_block_vec: vec![],
                grid,
                neuron_lib,}
    }

    pub fn spawn_barrier_blocks(&mut self, barrier_blocks_pos: Vec<(Dow, Dow)>){
        // this function adds the barrier blocks

        // check input
        if self.n_of_bots as usize + barrier_blocks_pos.len() + self.barrier_block_vec.len() > self.dim.0 as usize * self.dim.1 as usize{
            panic!("number of objects must be smaller than dim.0*dim.1")
        }

        for coord in barrier_blocks_pos.into_iter() {
            let index = self.barrier_block_vec.len();

            self.barrier_block_vec.push(objects::BarrierBlock::new(coord.0, coord.1)); // create new barrier block

            // create the raw pointer witch is passed to the Block on the coordinate
            let raw_pointer: *const dyn ObjectTrait = &self.barrier_block_vec[index]; 
            self.grid[coord.1 as usize][coord.0 as usize].edit_guest(Some(raw_pointer));
            
        }
    }

    pub fn spawn_bots(&mut self){
        let mut rng = rand::thread_rng();


        for bot in self.bot_vec.iter_mut(){

            // gen coords and check validaty
            let coords = loop{
                let x = rng.gen_range(0..self.dim.0) as usize;
                let y = rng.gen_range(0..self.dim.1) as usize;

                // check coords
                match self.grid[y][x].guest{
                    None => {break (x, y);}
                    Some(_) =>{continue;}
                }
            };

            bot.spawn(coords.0 as Dow, coords.1 as Dow);

            // create raw pointer
            let raw_pointer: *const dyn ObjectTrait = bot;
            // add the raw pointer to the grid
            self.grid[coords.1][coords.0].edit_guest(Some(raw_pointer));
        }

        self.bots_alive = self.n_of_bots;

    }

    pub fn calculate_step(&mut self){
        // for every bot in self.bot_vec 
        // the function bot.neurons_to_comute is called
        // this returns a Vec of vecs(one per bot) of vecs(one per neccesery gene)
        // the neurons are sorted per layer
        let input_neurons: Vec<Vec<Vec<[f64; 5]>>> = self.bot_vec.par_iter()// the process is computed in parrallel with .par_iter() method
        .map(|bot: &objects::Bot| bot.calculate_input(/*make &self immutable*/&*self))
        // collect the outputs of all bots in a Vec<Vec<[f64; 2]>>
        .collect::<Vec<_>>();

        // pass to calculate.rs
        // todo: create fn in calculate.rs
        let mut output: Vec<Vec<usize>>  = vec![];
        if !crate::settings::GPU{
            // returns a vec of vec(bot) of output neurons
            output = input_neurons.par_iter().
            map(|bot| crate::calculate::calc_step(bot)).collect::<Vec<_>>();
            println!("{:?}", output);
        }
        
        //  pass to bot.react(vec<usize>)
        // copy bot vec
        let mut bot_vec_copy = self.bot_vec.clone();

        // edit bot vec
        for (index, bot) in bot_vec_copy.iter_mut().enumerate() {
            bot.react(self, &output[index]);
        }

        // replace bot vec with edited vec
        self.bot_vec = bot_vec_copy;
        self.age_of_gen += 1;

        
        
    }

    fn select(&mut self){
        let selected_bot_vec = self.selection_criteria.select(self);
        let mut new_bot_vec: Vec<objects::Bot> = vec![];

        for i in 0..self.n_of_bots{
            let mut new_bot = objects::Bot::clone_(unsafe{&*selected_bot_vec[i as usize%selected_bot_vec.len()]}, & self.neuron_lib);
            new_bot_vec.push(new_bot);
        }

        self.bot_vec = new_bot_vec;

    }

    pub fn calculate_generation(&mut self){
        for _ in 0..crate::settings::GENERATION_STEPS{
            self.calculate_step();
        }
        self.age_of_gen = 0;
        self.generation += 1;

        self.select();

        self.spawn_bots();

    }

    pub fn store_world(&self, dir: String){
        
    }
}
