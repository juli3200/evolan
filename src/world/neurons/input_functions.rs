use super::super::{objects::Bot, World};
use crate::settings::*;

use rand::Rng;
use std::collections::HashMap;


// The Neurons are represented with functions
// every function returns a f64
// bool conversion: false -> -1, true -> 1


// every function takes &world / &bot as argument even if it isn't used
pub fn always_true(bot: &Bot, world: &World) -> f64{1.0}
pub fn always_false(bot: &Bot, world: &World) -> f64{-1.0}

// random input (-1 or 1)
pub fn random(bot: &Bot, world: &World) -> f64{
    let mut rng = rand::thread_rng();
    match rng.gen_bool(0.5) {
        true => {1.0}
        false => {-1.0},
    }
}

pub fn population_density(bot: &Bot, world: &World) -> f64{
    // calculates denity in a certain area

    // eg. density size = 10; start at -5 and go to +5
    // go through the grid and check if it host a guest
    // density counts any Blocks 
    let mut n_blocks = 0;
    for y in bot.y as u32-(DENSITY_SIZE/2)..bot.x as u32 + DENSITY_SIZE/2{
        // check if y is lower than 0 or bigger as dim[1]
        if y > world.dim.1 as u32{continue;}
        for x in bot.x as u32-(DENSITY_SIZE/2)..bot.x as u32+DENSITY_SIZE/2{
            // check if x is lower than 0 or bigger as dim[0]
            if x > world.dim.0 as u32{continue;}
            match world.grid[y as usize][x as usize].guest {
                // if it hosts guets n_blocks += 1
                Some(_) => {n_blocks+=1;}
                None => {continue;}
            }
        }
    }

    // return ratio 
    (n_blocks / DENSITY_SIZE^2) as f64 
}

// how many bots are alive
pub fn population_size(bot: &Bot, world: &World) -> f64{world.bots_alive as f64}

// every age is identical; stored in world
pub fn age(bot: &Bot, world: &World) -> f64{world.age_of_gen as f64}

//time of the world
pub fn time(bot: &Bot, world: &World) -> f64{world.time as f64}

// x coord
pub fn x(bot: &Bot, world: &World) -> f64{bot.x as f64}

// y coord
pub fn y(bot: &Bot, world: &World) -> f64{bot.y as f64}

// angle of bot
pub fn angle(bot: &Bot, world: &World) -> f64{bot.angle as f64}

// private fn used for all nn functions
fn nearest_neighbour(bot: &Bot, world: &World) -> (usize, usize){
    
    for i in 1..world.dim.0.min(world.dim.1){
        // ask
        //
        // todo
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
    (((coords_nn.0 as i64 - bot.x as i64)^2 + 
    (coords_nn.1 as i64- bot.y as i64)^2) as f64).sqrt()
}

pub fn angle_nn(bot: &Bot, world: &World) -> f64{
    let coords_nn = nearest_neighbour(bot, world);
    // calc the ratio of the triangle between the points
    let ratio: f64 = (coords_nn.0 as f64 - bot.x as f64) / (coords_nn.1 as f64 - bot.y as f64);
    // return the arctangens
    ratio.atan()
}

pub fn distance_nearest_boarder(bot: &Bot, world: &World) -> f64{
    // create a vec and evaluate the min
    let lv = *vec![bot.x, bot.y, world.dim.0-bot.x, world.dim.1-bot.y].iter().min().unwrap();
    lv as f64
}

// relation between northh south -> north 0; south -> 1
pub fn distance_north_south(bot: &Bot, world: &World) -> f64{
    // should not divide by zero
    bot.y as f64/(world.dim.1 as f64 -bot.y as f64)
}

// relation between west east
pub fn distance_west_east(bot: &Bot, world: &World) -> f64{
    bot.x as f64/(world.dim.0 as f64 -bot.x as f64)
}

// 0 fw; 1 left 2 bw 3 right -1 none
pub fn blocked_angle(bot: &Bot, world: &World) -> f64{
    // every coordinate whitch needs to be checked
    let coords = vec![(bot.x+1, bot.y), (bot.x, bot.y+1), 
    (bot.x-1, bot.y), (bot.x-1, bot.y)];

    // go through them and check if guet is some; first is returned
    let mut c: f64 = 0.0;
    for coord in coords.into_iter(){
        match world.grid[coord.1 as usize][coord.0 as usize].guest {
            Some(_) => {return c}
            None => {c+=1.0;}
        }
    }
    -1_f64
}


// if block in an angle hosts a guest
pub fn blocked_around(bot: &Bot, world: &World) -> f64{
    match blocked_angle(bot, world) as i64{
        0..=3 => {1f64}
        -1 => {-1f64}
        _ => {-1.0}
    }
}

// sums  up all letters and returns ceiled average
pub fn average_letter(bot: &Bot, world: &World) -> f64{
    let letters = &world.grid[bot.y as usize][bot.x as usize].letters;
    let sum = world.grid[bot.y as usize][bot.x as usize].letters.iter().sum::<u8>() as f64;
    sum / letters.len() as f64
}

// most common letter
pub fn mode_letter(bot: &Bot, world: &World) -> f64{
    // assign value to let letters
    let letters  = &world.grid[bot.y as usize][bot.x as usize].letters;
    // code from https://github.com/konsumer/learningrust/blob/fc959be93054d49142b14f27f294787c103ddb51/ap_chapter8_homework/src/main.rs#L18-L52
    let mut map = HashMap::new();
    let mut current_high_count = u8::MIN;
    let mut current_high_val = 0;
    for n in letters {
        let count = map.entry(n).or_insert(0);
        *count += 1;
        if count > &mut current_high_count {
            current_high_count = *count;
            current_high_val = n.clone();
        }
    }
    current_high_val as f64
}

pub fn length_letter(bot: &Bot, world: &World) -> f64{
    world.grid[bot.y as usize][bot.x as usize].letters.len() as f64
}

