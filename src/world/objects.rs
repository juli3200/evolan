use std::vec;

use rand::Rng;
use crate::calculate;

use super::ObjectTrait;

// impl of ObjectTrait for every Object
impl ObjectTrait for Bot{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
}
impl ObjectTrait for BarrierBlock{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
}


// Bot 
#[derive(Debug, Clone)]
pub struct Bot{
    /*
    This struct provides Information about the Bot e.g. genes, pos,...
    */

    // coordinates; i32
    // default of the coords are super::Dow::MAX; this coordinate is treated as None
    pub x: super::Dow,
    pub y: super::Dow,
    
    // angle 
    pub angle: u8,

    // genome; hex -> view concept
    pub genome: [u32; super::GENOME_LENGTH],
    
    pub input: Vec<Vec<[usize; 2]>>

}

impl Bot {
    // the new function creates the Bot without any information except the genome
    // this is because the grid and &world is not known
    pub fn new(genome: [u32; super::GENOME_LENGTH]) -> Self{
        Bot { x: super::Dow::MAX, 
              y: super::Dow::MAX, 
              angle: 0, 
              genome: genome, 
              input: vec![vec![]]
              }
    }

    // with the inherit function it's not neccesary to call the new function
    // the genome is provided using the 
    pub fn inherit(parents: (Bot, Bot), neuron_lib: &Vec<&usize>) -> Self{
        let mut rng = rand::thread_rng();

        // create a genome with zeros
        let mut genome: [u32; super::GENOME_LENGTH] = [0u32; super::GENOME_LENGTH];
        
        // filling raw genome with random value of parents
        let mut c = 0; // couter c
        for gene in genome.iter_mut(){
            match rng.gen_bool(0.5){
                true=> *gene = parents.0.genome[c as usize],
                false => *gene = parents.1.genome[c as usize],
            }
            c+= 1
        }

        if super::MUTATION_ENABLED{
            // call the neurons::mutate fn to mutate the genome
            super::neurons::mutate(&mut genome, neuron_lib);
        }

        Self::new(genome)


    }

    // the spawn function adds further information(coordinates) & is called after the World::new() in the World::spawn
    // for the spawn function either the new or the inherit function have already had to be called 
    pub fn spawn(& mut self, x:super::Dow, y:super::Dow){
        self.x = x;
        self.y = y;
        self.input = self.neurons_to_compute()
    }

    fn neurons_to_compute(&self) -> Vec<Vec<[usize; 2]>>{
        /*
        ///
        /// 
        ///  
        /// 
        /// 
        /// 
        /// 
        /// 
        
         */
        // continue here

        let mut a = Vec::new();
        a.push(Vec::new());
        a[0].push([18, 18]);
        a
    }

    pub fn calculate_input(&self, world: &super::World)-> Vec<[f64; 2]>{
        let mut calc_input_vec = vec![];
        for neuron in self.input[0].iter(){
            let calculated_input: f64 = super::neurons::INPUT_NEURON_REGISTER[neuron[0]](self, world);
            calc_input_vec.push([calculated_input, neuron[1] as f64])
        }
        
        calc_input_vec


    }

}


// BarrierBlock
#[derive(Debug, Clone, Copy)]
pub struct BarrierBlock{    /*
    This struct provides Information about the BarrierBlock( position)
    */

    // coordinates; i32
    x: super::Dow,
    y: super::Dow,
}


impl BarrierBlock{
    pub fn new(x:super::Dow, y:super::Dow)-> Self{
        BarrierBlock{x, y}
    }
}

#[derive(Debug, Clone)]
pub struct Block{
    // this block contains information about the guest of the block
    // e.g a bot han be a guest in the Block

    pub guest: Option<*const/*add a mut if needed*/ dyn ObjectTrait>,
    
    // coordinates; i32
    x: super::Dow,
    y: super::Dow,

    // received letter
    pub letters: Vec<u8>,

    // more can be added later

}


impl Block{
    pub fn new(guest: Option<*const/*add a mut if needed*/ dyn ObjectTrait>, x: super::Dow, y: super::Dow)-> Self{
        Block {guest, x, y, letters: vec![]}
    }

    pub fn edit_guest(&mut self, guest: Option<*const/*add a mut if needed*/dyn ObjectTrait>){
        match guest{
            None  => {self.guest = None;}
            Some(raw_pointer) => {
                self.guest = Some(raw_pointer)}
        }
    }
}