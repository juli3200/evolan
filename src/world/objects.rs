#![allow(dead_code)]

use super::ObjectTrait;

type WorldType = super::World;

// impl of ObjectTrait for every Object
impl ObjectTrait for Bot{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
    fn spawn(& mut self, world: &super::World)-> (){}
}
impl ObjectTrait for BarrierBlock{
    fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}
    fn spawn(& mut self, world: &super::World) {
        
    }

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
    pub fn inherit(parents: (Bot, Bot)) -> (){}

    // the spawn function is declared in the trait objects
    // the spawn function adds further information(coordinates) & is called after the World::new() in the World::spawn


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
    fn new(){

    }
}
