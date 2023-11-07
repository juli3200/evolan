use super::super::{objects::Bot, World};
use crate::settings::*;

use rand::Rng;

// make every neuron to a  function
/* 
pub enum InputNeurons{
    PopulationDensity(f32), 
    PopulationSize(u16),
    // age of bot
    Age(u16), 
    // time of the whole world
    Time(u64),
    // x coord
    X(super::Dow), 
    // y coord
    Y(super::Dow),
    // own angle
    Angle(u8),

    // angle nearest neighbour
    AngleNN(u8),
    // Distance nearest neighbour
    DistanceNN(super::Dow),

    // distance to nearest boarder
    DistanceNearestBoarder(u32),
    // relation between northh south -> north 0; south -> 1
    DistanceNorthSouth(super::Dow),
    // relation between west east
    DistanceWestEast(super::Dow),

    // if block in an angle hosts a guest u8 -> angle
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






