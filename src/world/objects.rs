#![allow(dead_code)]

use super::ObjectTrait;

type WorldType = super::World;

// impl of ObjectTrait for every Object
impl ObjectTrait for Bot{fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}}
impl ObjectTrait for BarrierBlock{fn pos(&self)->(super::Dow, super::Dow) {(self.x, self.y)}}

// Bot 
#[derive(Debug)]
pub struct Bot{
    /*
    This struct provides Information about the Bot e.g. genes, pos,...
    */

    // coordinates; i32
    x: super::Dow,
    y: super::Dow,
    
    // angle 
    angle: u8,

    // genome; hex -> view concept
    genome_length: u8,
    genome: [u32; super::GENOME_LENGTH],
}

impl Bot {
    fn new(w: &WorldType) -> (){
    }
    fn inherit(x:super::Dow, y:super::Dow, parents: (Bot, Bot)) -> (){}
}


// BarrierBlock
#[derive(Debug)]
pub struct BarrierBlock{    /*
    This struct provides Information about the BarrierBlock( position)
    */

    // coordinates; i32
    x: super::Dow,
    y: super::Dow,
}
