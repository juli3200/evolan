use crate::settings::Dow;
use crate::world::neurons;

use super::World;
use super::objects::Bot;

#[derive(Debug)]
pub enum Criteria{
    // survive by beeing in certain Area
    Area([(super::Dow, super::Dow); 2])
}

    

fn area_fn(world: &World, coords: &[(Dow, Dow); 2]) -> Vec<[u32; crate::settings::GENOME_LENGTH]>{
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];

    // create the selected bot_vec
    // iterate over the bots and check if they are in area

    for bot in world.bot_vec.iter(){
        if /*y coord*/(bot.y >= coords[0].1 && bot.y < coords[1].1) &&  (bot.x >= coords[0].0 && bot.x < coords[1].0){
            selected_bot_vec.push(bot.genome.clone());
        }
    }

    

    // selected bot vec is returned to the world select fn
    selected_bot_vec

}

impl Criteria{
    // selcect survived bots based on criteris
    pub fn select(&self, world: &World) -> Vec<[u32; crate::settings::GENOME_LENGTH]>{
        match self {
            Self::Area(coords) => {area_fn(world, coords)},

        }

    }

}