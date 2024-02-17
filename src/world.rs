use std::{rc::Rc, process::Output, fmt::{write, format}, fs, ptr};
use std::cell::RefCell;

use rand::Rng;
use rayon::{prelude::*, iter::Empty};
use serde::Serialize;

pub mod objects;
pub mod neurons;
pub mod criteria;
pub mod cluster;

mod world_fns;
use world_fns::*;

// constants
use crate::{settings::{self, *}, tools};

use self::objects::Bot;

#[derive(Debug, Clone,  Copy)]
pub enum Kind{
    Bot,
    BarrierBlock,
    Empty
}


#[derive(Debug)]
pub struct World{
    pub settings_: Settings,

    // selection criteria can be found in criteria.rs
    pub selection_criteria: criteria::Criteria,

    // output path
    pub name: String,

    // generation of the world
    pub generation: usize,
    time: u64, // time overall; 
    age_of_gen: u16,
    pub bots_alive: u16, 
    killed_bots: Vec<Rc<RefCell<Bot>>>,

    // holding of the bots and blocks etc
    pub bot_vec: Vec<Bot>,

    pub bot_register: Vec<Option<RefCell<Bot>>>,

    // vec of all clusters
    pub cluster_vec: Vec<cluster::Cluster>,

    // vec of all bots that are ready to form a cluster
    // id is saved in the vec
    cluster_ready_vec: Vec<RefCell<Bot>>,

    barrier_block_vec: Vec<objects::BarrierBlock>,

    // grid with coordinates of object
    pub grid: Vec<Vec<objects::Block>>,

    pub neuron_lib: Vec<usize>,

    pub grid_store: Vec<Vec<Vec<Kind>>>

    //
    // maybe add a vec of all generations
    //


}

