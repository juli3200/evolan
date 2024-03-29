use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::Dow;
use crate::world::neurons;

use super::{World, Kind};
use super::objects::Bot;

#[derive(Debug, Clone)]
pub enum Criteria{
    // survive by beeing in certain Area
    Area([(super::Dow, super::Dow); 2]),
    // survive in circle (x, y), r
    Circle((super::Dow, super::Dow), super::Dow),
    // survive if you built a cluster
    Cluster,
    // survive if you are ready to build a cluster  
    ClusterReady,

    None,
}

fn cluster_fn(world: &mut World) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];
    let mut surviers_grid = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];

    for bot in world.bot_vec.iter_mut(){
        if bot.cluster.is_some(){
            selected_bot_vec.push(bot.genome.clone());
            surviers_grid[bot.y as usize][bot.x as usize] = Kind::Bot(bot.id);
        }
    }

    // selected bot vec is returned to the world select fn
    (selected_bot_vec, surviers_grid)
}

fn cluster_ready_fn(world: &mut World) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];
    let mut surviers_grid = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];

    for bot in world.bot_vec.iter_mut(){
        if bot.build_cluster{
            selected_bot_vec.push(bot.genome.clone());
            surviers_grid[bot.y as usize][bot.x as usize] = Kind::Bot(bot.id);
        }
    }

    // selected bot vec is returned to the world select fn
    (selected_bot_vec, surviers_grid)
}

fn circle_fn(world: &mut World, coords: &(Dow, Dow), r:&Dow) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
    let coords = (coords.0 as i64, coords.1 as i64);
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];
    // creating image of bots who survived
    let mut surviers_grid = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];
    let r = *r as f64;

    // calculate the distance of every bot and check if its lower then radius
    for bot in world.bot_vec.iter_mut(){
        if (((coords.0 - bot.x as i64).pow(2) +
        (coords.1 - bot.y as i64).pow(2)) as f64).sqrt() < r{
            selected_bot_vec.push(bot.genome.clone());
            surviers_grid[bot.y as usize][bot.x as usize] = Kind::Bot(bot.id);
        }
    }

    // selected bot vec is returned to the world select fn
    (selected_bot_vec, surviers_grid)

}

fn area_fn(world: &mut World, coords: &[(Dow, Dow); 2]) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];

    // creating image of bots who survived
    let mut surviers_grid = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];

    // create the selected bot_vec
    // iterate over the bots and check if they are in area

    for bot in world.bot_vec.iter_mut(){
        if /*y coord*/(bot.y >= coords[0].1 && bot.y < coords[1].1) &&  (bot.x >= coords[0].0 && bot.x < coords[1].0){
            selected_bot_vec.push(bot.genome.clone());
            surviers_grid[bot.y as usize][bot.x as usize] = Kind::Bot(bot.id);
        }
    }


    // selected bot vec is returned to the world select fn
    (selected_bot_vec, surviers_grid)

}

fn none(world: &mut World) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
    let mut selected_bot_vec: Vec<[u32; crate::settings::GENOME_LENGTH]> = vec![];
    let mut surviers_grid = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];

    for bot in world.bot_vec.iter_mut(){
        selected_bot_vec.push(bot.genome.clone());
        surviers_grid[bot.y as usize][bot.x as usize] = Kind::Bot(bot.id);
    }

    // selected bot vec is returned to the world select fn
    (selected_bot_vec, surviers_grid)
}

impl Criteria{
    // selcect survived bots based on criteris
    pub fn select(&self, world: &mut World) -> (Vec<[u32; crate::settings::GENOME_LENGTH]>, Vec<Vec<Kind>>){
        match self {
            Self::Area(coords) => {area_fn(world, coords)},
            Self::Circle(coords, r) => {circle_fn(world, coords, r)}
            Self::Cluster => {cluster_fn(world)}
            Self::ClusterReady => {cluster_ready_fn(world)},
            Self::None => {none(world)}
        }

    }

}