use crate::settings::KILLING_ENABLED;

use super::{World, ObjectTrait, ObjectsEnum};
use super::objects::Bot;

#[derive(Debug)]
pub enum Criteria{
    // survive by beeing in certain Area
    Area([(super::Dow, super::Dow); 2])
}

    

fn area_fn(world: &World, coords: &[(u8, u8); 2]) -> Vec<*const Bot>{
    let mut selected_bot_vec: Vec<*const Bot> = vec![];

    // create the selected bot_vec
    // iterate over the rectangle in the grid
    for y in coords[0].1..coords[1].1{
        for x in coords[0].0..coords[1].0{

            // matchh the guest of the current coordinate
            match world.grid[y as usize][x as usize].guest{
                // if guest is Some check if the raw pointer isnt null
                Some(block) => {
                    if !block.is_null(){

                        // dereference raw pointer and convert to reference
                        let reference = unsafe{&*block};
                        // match reference and check if Bot 
                        match reference.kind() {
                            ObjectsEnum::Bot(bot) => selected_bot_vec.push(bot),
                            ObjectsEnum::BarrierBlock(_) => continue
                        }
                    }
                    else {println!("error in area"); continue;}
                }, 
                None => continue
            }
        }
    }

    // selected bot vec is returned to the world select fn
    selected_bot_vec

}

impl Criteria{
    // selcect survived bots based on criteris
    pub fn select(&self, world: &World) -> Vec<*const Bot>{
        match self {
            Self::Area(coords) => {area_fn(world, coords)},

        }
    }

}