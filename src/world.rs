use std::{process::Output, fmt::write};

use rand::Rng;
use rayon::{prelude::*, iter::Empty};
use serde::Serialize;

pub mod objects;
pub mod neurons;
pub mod criteria;

// constants
use crate::{settings::*, tools};

use self::objects::Bot;

#[derive(Debug, Clone, Serialize,  Copy)]
pub enum Kind{
    Bot(u16),
    BarrierBlock,
    Empty
}




#[derive(Debug)]
pub struct World{
    //setting dimension of the world; (u8, u8)
    pub dim: (Dow, Dow),

    //number of bots
    n_of_bots: u16,

    // selection criteria can be found in criteria.rs
    pub selection_criteria: criteria::Criteria,

    // output path
    pub path: String,

    // generation of the world
    pub generation: usize,
    time: u64, // time overall; 
    age_of_gen: u16,
    bots_alive: u16, 

    // holding of the bots and blocks etc
    pub bot_vec: Vec<objects::Bot>,
    barrier_block_vec: Vec<objects::BarrierBlock>,

    // grid with coordinates of object
    pub grid: Vec<Vec<objects::Block>>,

    pub neuron_lib: Vec<&'static usize>,

    pub grid_store: Vec<Vec<Vec<Kind>>>

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
    pub fn new(dim: (Dow, Dow), n_of_bots: u16, selection_criteria: criteria::Criteria, path: String) -> Self {

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
            for i in 0..n_of_bots {
                bot_vec.push(objects::Bot::new(neurons::create_genome(&neuron_lib), i));
            }
        //

        // the grid is a 2d vec with Blocks in it
        let mut grid = Vec::new();
        for y in 0..dim.1{
            let mut row = Vec::new();
            for x in 0..dim.0{
                row.push(objects::Block::new(Kind::Empty, x, y));
            }
            grid.push(row);
        }

        World { dim,
                n_of_bots,
                selection_criteria,
                path,
                generation: 0,
                time: 0,
                age_of_gen: 0,
                bots_alive: 0,
                bot_vec,
                barrier_block_vec: vec![],
                grid,
                neuron_lib,
                grid_store: vec![], }
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
            self.grid[coord.1 as usize][coord.0 as usize].edit_guest(Kind::BarrierBlock);
            
        }
    }

    pub fn spawn_bots(&mut self){
        let mut rng = rand::thread_rng();


        for (i, bot) in self.bot_vec.iter_mut().enumerate(){

            // gen coords and check validaty
            let coords = loop{
                let x = rng.gen_range(0..self.dim.0) as usize;
                let y = rng.gen_range(0..self.dim.1) as usize;

                // check coords
                match self.grid[y][x].guest{
                    Kind::Empty => {break (x, y);}
                    _ =>{continue;}
                }
            };

            bot.spawn(coords.0 as Dow, coords.1 as Dow);
            // add the raw pointer to the grid
            self.grid[coords.1][coords.0].edit_guest(Kind::Bot(i as u16));
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
        

        self.grid_store.push(tools::store_gen::store_step(&*self));
        
        
    }

    fn select(&mut self){

        let (selected_bot_vec, survivars_grid) = self.selection_criteria.select(self);

        self.grid_store.push(survivars_grid);

        let mut new_bot_vec: Vec<objects::Bot> = vec![];

        if selected_bot_vec.len() == 0{

            for i in 0..self.n_of_bots{
                new_bot_vec.push(objects::Bot::new(neurons::create_genome(&self.neuron_lib), i));
            }

        }

        else{
            
            for i in 0..self.n_of_bots{
                let b = selected_bot_vec[i as usize%selected_bot_vec.len()];
                let b2 = selected_bot_vec[(i+1) as usize%selected_bot_vec.len()];
                

                let new_bot = match INHERIT{
                    true => objects::Bot::inherit((&b, &b2), &self.neuron_lib, i),
                
                    false => objects::Bot::clone_(&b, &self.neuron_lib, i),
                };
               
                new_bot_vec.push(new_bot);
            }
            
        }
        self.bot_vec = new_bot_vec;


        // resetting self.grid
        for row in self.grid.iter_mut(){
            for block in row{
                match block.guest {
                    Kind::Bot(_) => block.guest = Kind::Empty,
                    Kind::Empty => block.guest = Kind::Empty,
                    Kind::BarrierBlock => block.guest = Kind::BarrierBlock
                }
            }
        }
        self.spawn_bots();

    }

    pub fn calculate_generation(&mut self){
        for _ in 0..crate::settings::GENERATION_STEPS{
            self.calculate_step();
        }
   
        self.select();

        self.age_of_gen = 0;
        self.generation += 1;

        tools::store_gen::store_generation(&*self);
        self.grid_store = vec![];

        self.spawn_bots();

    }

}
