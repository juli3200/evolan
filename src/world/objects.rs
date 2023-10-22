#![allow(dead_code)]

use std::env::consts::FAMILY;

use rand::Rng;
use super::ObjectTrait;

type WorldType = super::World;

// impl of ObjectTrait for every Object
impl ObjectTrait for Bot{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
}
impl ObjectTrait for BarrierBlock{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
}
impl ObjectTrait for Block {
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
    x: super::Dow,
    y: super::Dow,
    
    // angle 
    angle: u8,

    // genome; hex -> view concept
    genome: [u32; super::GENOME_LENGTH],
    // output; vec of outputs from the output enum 
    output: Vec<super::neurons::OutputNeurons>,
}

impl Bot {
    // the new function creates the Bot without any information except the genome
    // this is because the grid and &world is not known
    pub fn new(genome: [u32; super::GENOME_LENGTH]) -> Self{
        Bot { x: super::Dow::MAX, 
              y: super::Dow::MAX, 
              angle: 0, 
              genome: genome, 
              output: vec![]}
    }

    // with the inherit function it's not neccesary to call the new function
    // the genome is provided using the 
    pub fn inherit(parents: (Bot, Bot)) -> (){
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
            // mutation
            for gene in genome.iter_mut(){
                
                let mut hex_gene: Vec<char> = format!("{:X}", gene).chars().collect(); // convert u32 in hex string

                // go through every letter and change it by chance
                let mut c = 0;
                for letter in hex_gene.iter_mut(){
                    match rng.gen_bool(super::MUTATION_RATE) { 
                        // if mutation occurs, the loop searches for a new random valid gene 
                        // the validaty is checked with the neurons::valid_gene fn
                        // if valid the new_letter is assigned to the *letter
                        true => *letter =   
                            loop{let new_letter = std::char::from_u32(rng.gen_range(0..16u32)).unwrap();
                                // the new_gene is a copy of the gene
                                let mut new_gene = hex_gene.clone();
                                new_gene[c] = new_letter; // the new letter is changed and checked
                                match super::neurons::valid_gene(new_gene){
                                    true=>break new_letter,
                                    false=> continue
                                }
                            }, 
                        false => {}
                    }
                    c+=1;
                }
            }
        }


    }

    // the spawn function adds further information(coordinates) & is called after the World::new() in the World::spawn
    // for the spawn function either the new or the inherit function have already had to be called 
    pub fn spawn(& mut self, world: &super::World)-> (){}

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

    guest: Option<[usize; 2]>,

    // coordinates; i32
    x: super::Dow,
    y: super::Dow

    // more can be added later

}


impl Block{
    pub fn new(x: super::Dow, y: super::Dow)-> Self{
        Block {guest: None, x: x, y: y }
    }
}