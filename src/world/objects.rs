use std::vec;

use rand::Rng;
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
    angle: u8,

    // genome; hex -> view concept
    genome: [u32; super::GENOME_LENGTH],
    // output; vec of outputs from the output enum 

}

impl Bot {
    // the new function creates the Bot without any information except the genome
    // this is because the grid and &world is not known
    pub fn new(genome: [u32; super::GENOME_LENGTH]) -> Self{
        Bot { x: super::Dow::MAX, 
              y: super::Dow::MAX, 
              angle: 0, 
              genome: genome, }
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
            super::neurons::mutate(&mut genome, neuron_lib)
        }

        Self::new(genome)


    }

    // the spawn function adds further information(coordinates) & is called after the World::new() in the World::spawn
    // for the spawn function either the new or the inherit function have already had to be called 
    pub fn spawn(& mut self, x:super::Dow, y:super::Dow){
        self.x = x;
        self.y = y;
    }

    pub fn neurons_to_comute() -> Vec<(Box<dyn super::NeuronTrait>, Box<dyn super::NeuronTrait>)>{
        vec![(Box::new(super::neurons::InputNeurons::AlwaysFalse), Box::new(super::neurons::InputNeurons::AlwaysFalse))]

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
    y: super::Dow

    // more can be added later

}


impl Block{
    pub fn new(guest: Option<*const/*add a mut if needed*/ dyn ObjectTrait>, x: super::Dow, y: super::Dow)-> Self{
        Block {guest, x, y }
    }

    pub fn edit_guest(&mut self, guest: Option<*const/*add a mut if needed*/dyn ObjectTrait>){
        match guest{
            None  => {self.guest = None;}
            Some(raw_pointer) => {
                self.guest = Some(raw_pointer)}
        }
    }
}