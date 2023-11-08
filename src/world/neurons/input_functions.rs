use std::ops::RangeBounds;

use super::super::{objects::Bot, World};
use crate::settings::*;

use rand::Rng;

// make every neuron to a  function
/* 

    
    BlockedAround(u8),

    // received communication hex letter pointer to the array in the guest block
    ReComm(*const [u8]),
    // length of the array
    LenghtComm(usize),
    // most common letter
    MostCommonLetter(u8),

}
*/
/*
    // angle is turned +90 or -90
    TurnRight,
    TurnLeft,

    // zero is backwards 1 is forwards
    MoveStraight(bool),
    // left or right movement
    MoveSideways(bool),
    // move in x_direction; 1 positive x, -1 negative
    MoveX(bool),
    // move in y direction
    MoveY(bool),
    // move in rnd deirection
    MoveRandom(u8),

    // send letter
    SendComm(u8),

    // can live for a specific time; really high value to be fired
    PlaceBarrierBlock,

    // mutation and modification
    // these Neurons need an extrem high value to be fired
    Mutate,
    // modification
    Modify,

    // kill neuron can be deactivated
    // kill bot in front
    Kill */

// The Neurons are represented with functions


// every function takes &world / &bot as argument even if it isn't used
pub fn always_true(bot: &Bot, world: &World) -> bool{true}
pub fn always_false(bot: &Bot, world: &World) -> bool{false}

// random input (0 or 1)
pub fn random(bot: &Bot, world: &World) -> bool{
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5) as bool
}

pub fn population_density(bot: &Bot, world: &World) -> f64{
    // calculates denity in a certain area

    // eg. density size = 10; start at -5 and go to +5
    // go through the grid and check if it host a guest
    // density counts any Blocks 
    let mut n_blocks = 0;
    for y in bot.y-(DENSITY_SIZE/2)..bot.x + DENSITY_SIZE/2{
        // check if y is lower than 0 or bigger as dim[1]
        if y< 0 || y > world.dim[1]{continue;}
        for x in bot.x-(DENSITY_SIZE/2)..bot.x+DENSITY_SIZE/2{
            // check if x is lower than 0 or bigger as dim[0]
            if x < 0 || x > world.dim[0]{continue;}
            match world.grid[y as usize][x as usize].guest {
                // if it hosts guets n_blocks += 1
                Some(_) => {n_blocks+=1;}
                None => {continue;}
            }
        }
    }

    // return ratio 
    n_blocks / DENSITY_SIZE**2
}

// how many bots are alive
pub fn population_size(bot: &Bot, world: &World) -> u16{world.bots_alive}

// every age is identical; stored in world
pub fn age(bot: &Bot, world: &World) -> u16{world.age_of_gen}

//time of the world
pub fn time(bot: &Bot, world: &World) -> u64{world.time}

// x coord
pub fn x(bot: &Bot, world: &World) -> crate::settings::Dow{bot.x}

// y coord
pub fn y(bot: &Bot, world: &World) -> crate::settings::Dow{bot.y}

// angle of bot
pub fn angle(bot: &Bot, world: &World) -> u8{bot.angle}

// private fn used for all nn functions
fn nearest_neighbour(bot: &Bot, world: &World) -> (Dow, Dow){
    
    for i in 1..world.dim[0].min(world.dim[1]){
        // ask
        //
        //
        //
        //
        //
        
        
    }
    (0, 0)
}

// distance to nearest neighbour
pub fn distance_nn(bot: &Bot, world: &World) -> f64{
    // get the coords of the nn
    let coords_nn = nearest_neighbour(bot, world);

    // calculate and return the distance
    ((coords_nn[0] - bot.x)**2 + (coords_nn[1] - bot.y) as f64).sprt()
}

pub fn angle_nn(bot: &Bot, world: &World) -> u8{
    let coords_nn = nearest_neighbour(bot, world);
    // calc the ratio of the triangle between the points
    let ratio: f64 = (coords_nn[0] - bot.x) / (coords_nn[1] - bot.y);
    // return the arctangens
    ratio.atan()
}

pub fn distance_nearest_boarder(bot: &Bot, world: &World) -> Dow{
    // create a vec and evaluate the min
    vec![bot.x, bot.y, world.dim[0]-bot.x, world.dim[1]-bot.y].iter()
    .min().unwrap()
}

// relation between northh south -> north 0; south -> 1
pub fn distance_north_south(bot: &Bot, world: &World) -> f64{
    bot.y/(world.dim[1]-bot.y)
}

// relation between west east
pub fn distance_west_east(bot: &Bot, world: &World) -> f64{
    bot.x/(world.dim[0]-bot.x)
}

// if block in an angle hosts a guest
pub fn blocked_around(bot: &Bot, world: &World) -> bool{
    // every coordinate whitch needs to be checked
    let coords = vec![(bot.x, bot.y-1), (bot.x, bot.y+1), 
    (bot.x-1, bot.y), (bot.x+1, bot.y)];

    for coord in coords.into_iter(){
        match world.grid[coord[1] as usize][coord[0] as usize] {
            Some(_) => {true}
            None => {continue;}
        }
    }
    false
}
