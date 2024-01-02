use crate::GENOME_LENGTH as GL;
use crate::tools::plot_network;
use crate::world::{Kind, World, objects::Bot, objects::Block, neurons};
use crate::settings::*;
use rand::Rng;


// 0 = 0, 1 = 90, 2 = 180, 3= 270
pub fn turn_left(bot: &mut Bot, world: &mut World){
    bot.angle += 1;
    bot.angle %= 4;
}

// 0 = 0, 1 = 90, 2 = 180, 3= 270
pub fn turn_right(bot: &mut Bot, world: &mut World){
    bot.angle += 3;
    bot.angle %= 4;
}

fn check_block(world: &mut World, new_coords: &(isize, isize)) -> bool{
    if (new_coords.0 >= 0 &&  new_coords.0 < world.settings_.dim.0 as isize) && (new_coords.1 >= 0 && new_coords.1 < world.settings_.dim.1 as isize){
        return match world.grid[new_coords.1 as usize][new_coords.0 as usize].guest {
            Kind::Empty => true, 
            _ => false
        }
    }
    else {
        false
    }
}

fn edit_grid(world: &mut World, bot: &mut Bot, new_coords: (isize, isize), old_coords: (Dow, Dow)){
    world.grid[new_coords.1 as usize][new_coords.0 as usize].guest = 
    world.grid[old_coords.1 as usize][old_coords.0 as usize].guest.clone();
    world.grid[old_coords.1 as usize][old_coords.0 as usize].guest = Kind::Empty;
    bot.x = new_coords.0 as Dow;
    bot.y = new_coords.1 as Dow;

}

pub fn move_fw(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x.clone() as isize, bot.y.clone() as isize);
    match bot.angle {
        0 => {new_coords.0 = if new_coords.0 < world.settings_.dim.0 as isize {new_coords.0 + 1}
        else{new_coords.0};},
        1 => {new_coords.1 = if new_coords.1 > 0 {new_coords.1 - 1}
        else{new_coords.1};},
        2 => {new_coords.0 = if new_coords.0 > 0 {new_coords.0- 1}
        else{new_coords.0};},
        3 => {new_coords.1 = if new_coords.1 < world.settings_.dim.1 as isize{new_coords.1 + 1}
        else{new_coords.1};},
        _ => {panic!("Not found, move right")}
    }
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}

pub fn move_left(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x.clone() as isize, bot.y.clone() as isize);
    match bot.angle {
        3 => {new_coords.0 = if new_coords.0 < world.settings_.dim.0 as isize {new_coords.0 + 1}
        else{new_coords.0};},
        0 => {new_coords.1 = if new_coords.1 > 0 {new_coords.1 - 1}
        else{new_coords.1};},
        1 => {new_coords.0 = if new_coords.0 > 0 {new_coords.0- 1}
        else{new_coords.0};},
        2 => {new_coords.1 = if new_coords.1 < world.settings_.dim.1 as isize{new_coords.1 + 1}
        else{new_coords.1};},
        _ => {panic!("Not found, move right")}
    }
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}

pub fn move_right(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x.clone() as isize, bot.y.clone() as isize);
    match bot.angle {
        1 => {new_coords.0 = if new_coords.0 < world.settings_.dim.0 as isize {new_coords.0 + 1}
        else{new_coords.0};},
        2 => {new_coords.1 = if new_coords.1 > 0 {new_coords.1 - 1}
        else{new_coords.1};},
        3 => {new_coords.0 = if new_coords.0 > 0 {new_coords.0- 1}
        else{new_coords.0};},
        0 => {new_coords.1 = if new_coords.1 < world.settings_.dim.1 as isize{new_coords.1 + 1}
        else{new_coords.1};},
        _ => {panic!("Not found, move right")}
    }
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}

pub fn pos_x(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x as isize, bot.y as isize);
    new_coords.0 += 1;
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}

pub fn neg_x(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x as isize, bot.y as isize);
    new_coords.0 -= 1;
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}
pub fn pos_y(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x as isize, bot.y as isize);
    new_coords.1 += 1;
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}
pub fn neg_y(bot: &mut Bot, world: &mut World){
    let mut new_coords = (bot.x as isize, bot.y as isize);
    new_coords.1 -= 1;
    if check_block(world, &new_coords){
        edit_grid(world, bot, new_coords, (bot.x.clone(), bot.y.clone()));
    }
}

// places barrier blok behind
pub fn place_barrier_block(bot: &mut Bot, world: &mut World){
    
    let mut rng = rand::thread_rng();
    if !rng.gen_bool(world.settings_.barrier_block_blockade){return;}

    let mut new_coords = (bot.x.clone() as isize, bot.y.clone() as isize);
    match bot.angle {
        2 => {new_coords.0 = if new_coords.0 < world.settings_.dim.0 as isize {new_coords.0 + 1}
        else{new_coords.0};},
        3 => {new_coords.1 = if new_coords.1 > 0 {new_coords.1 - 1}
        else{new_coords.1};},
        0 => {new_coords.0 = if new_coords.0 > 0 {new_coords.0- 1}
        else{new_coords.0};},
        1 => {new_coords.1 = if new_coords.1 < world.settings_.dim.1 as isize{new_coords.1 + 1}
        else{new_coords.1};},
        _ => {panic!("Not found, move right")}
    }
    if check_block(world, &new_coords){
        world.spawn_barrier_blocks(vec![(new_coords.0 as Dow, new_coords.1 as Dow)]);
    }   
}

pub fn mutate(bot: &mut Bot, world: &mut World){
    let mut rng = rand::thread_rng();
    
    if world.settings_.neuronal_mutation_enabled&&
    rng.gen_bool(world.settings_.neuronal_mutation_rate){
        neurons::mutate(&mut bot.genome, &world.neuron_lib, &world.settings_);
    }
}

// modify??

pub fn kill(bot: &mut Bot,world: &mut World){
    if world.settings_.killing_enabled{
        let mut new_coords = (bot.x.clone() as isize, bot.y.clone() as isize);
        match bot.angle {
            0 => {new_coords.0 = if new_coords.0 + 1 < world.settings_.dim.0 as isize {new_coords.0 + 1}
            else{new_coords.0};},
            1 => {new_coords.1 = if new_coords.1 -1 > 0 {new_coords.1 - 1}
            else{new_coords.1};},
            2 => {new_coords.0 = if new_coords.0 -1 > 0 {new_coords.0- 1}
            else{new_coords.0};},
            3 => {new_coords.1 = if new_coords.1 + 1 < world.settings_.dim.1 as isize{new_coords.1 + 1}
            else{new_coords.1};},
            _ => {panic!("Not found, kill")}
        }
        match world.grid[new_coords.1 as usize][new_coords.0 as usize].guest {
            Kind::Bot(id) =>{
                world.bots_alive-=1;
                world.killed_bots.push(id);},
            _ => {}
        }
    }
}

// comm??!!