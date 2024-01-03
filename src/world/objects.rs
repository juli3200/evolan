use std::path::Path;

use rand::Rng;

use super::{neurons::GeneTrait, Kind};
use crate::{tools, settings::{self, GENOME_LENGTH, Settings}};


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
    pub genome: [u32; GENOME_LENGTH],

    pub id: u16,
    

}

impl  Bot  {
    // the new function creates the Bot without any information except the genome
    // this is because the grid and &world is not known
    // ✅
    pub fn new(genome: [u32; GENOME_LENGTH], id: u16) -> Self{
        Bot { x: super::Dow::MAX, 
              y: super::Dow::MAX, 
              angle: 0, 
              genome, 
              id
              }
    }

    // with the inherit function it's not neccesary to call the new function
    // the genome is provided using the 
    // todo: check
    // update
    pub fn inherit(parents: (&[u32; GENOME_LENGTH], &[u32; GENOME_LENGTH]), neuron_lib: &Vec<usize>,  id: u16, settings_: &Settings) -> Self{
        let mut rng = rand::thread_rng();

        // create a genome with zeros
        let mut genome: [u32; GENOME_LENGTH] = [0u32; GENOME_LENGTH];

        
        // filling raw genome with random value of parents
        let mut c = 0; // couter c
        for gene in genome.iter_mut(){
            match rng.gen_bool(0.5){
                true=> *gene = parents.0[c as usize],
                false => *gene = parents.1[c as usize],
            }
            c+= 1
        }

        if settings_.mutation_enabled{
            // call the neurons::mutate fn to mutate the genome
            super::neurons::mutate(&mut genome, neuron_lib, settings_);
        }

        Self::new(genome, id)


    }

    // new bot is created as a clone of the old one with mutation
    pub fn clone_(parent_genome: &[u32; GENOME_LENGTH],  neuron_lib: &Vec<usize>, id: u16, settings_: &Settings) -> Self{
        let mut genome = parent_genome.clone();
        assert_eq!(genome, *parent_genome);
        
        if settings_.mutation_enabled{
            super::neurons::mutate(&mut genome, neuron_lib, settings_);
        }

        Self::new(genome, id)
    }

    // the spawn function adds further information(coordinates) & is called after the World::new() in the World::spawn
    // for the spawn function either the new or the inherit function have already had to be called 
    // ✅
    pub fn spawn(& mut self, x:super::Dow, y:super::Dow){
        self.x = x;
        self.y = y;
    }

    pub fn neurons_to_compute(&self, settings_: &Settings) -> Vec<Vec<[f64;5]>>{
        // this function filters all unused neurons out and sorts them
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
        let mut decoded_genome = vec![vec![]; settings_.inner_layers+1];
        for gene in self.genome{
            let a =gene.decode_gene();
            let mut fa = [0.0; 5];
            fa[0] = 0.0;
            fa[1] = a[1] as f64;fa[2] = a[2] as f64 + 1.0;fa[3] = a[3] as f64;fa[4] = a[4] as f64;
            decoded_genome[a[0] as usize].push(fa);

            
        }
         
        let computed_neurons: Vec<_> = vec![1];
        /* 
        let mut layer_c = 0;
        for layer in decoded_genome.iter(){
            let mut connection_c = 0;
            for connection in layer{
                connection_c+=1;
            }
            layer_c+=1;
        }*/
        return decoded_genome
    }

    pub fn calculate_input(&self, world: &super::World)-> Vec<Vec<[f64; 5]>>{
        let mut calc_input_vec = self.neurons_to_compute(&world.settings_);
        for neuron in calc_input_vec[0].iter_mut(){
            neuron[0] = super::neurons::INPUT_NEURON_REGISTER[neuron[1] as usize](self, world);
        }
        

        calc_input_vec


    }

    pub fn react(&mut self, world: &mut super::World, output: &Vec<usize>){
        for neuron in output{
            super::neurons::OUTPUT_NEURON_REGISTER[*neuron](self, world);
        }
    }

    // ✅
    pub fn draw_graph(&self, world: &super::World, path: &str){
        let decoded_genes: Vec<[u32; 5]> = self.genome.map(|gene| gene.decode_gene()).to_vec();
        tools::plot_network::plot(&decoded_genes, world.settings_.inner_layers, path);
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
    // e.g a bot can be a guest in the Block

    pub guest: super::Kind,
    
    // coordinates; i32
    x: super::Dow,
    y: super::Dow,

    // received letter
    pub letters: Vec<u8>,

    // more can be added later

}


impl Block{
    pub fn new(guest: Kind, x: super::Dow, y: super::Dow)-> Self{
        Block {guest, x, y, letters: vec![]}
    }

    pub fn edit_guest(&mut self, guest: Kind){
        self.guest = guest;
    }
}