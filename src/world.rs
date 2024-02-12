use std::{process::Output, fmt::{write, format}, fs, ptr};

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
    Bot(*mut objects::Bot),
    BarrierBlock,
    Empty
}

impl Kind{
    pub fn type_(&self) -> Self{
        let p: *mut Bot = ptr::null_mut();
        match *self {
            Self::Bot(_) => Self::Bot(p),
            Self::BarrierBlock => Self::BarrierBlock,
            Self::Empty => Self::Empty
        }
    }
}

#[derive(Debug)]
pub struct World{
    pub settings_: settings::Settings,

    // selection criteria can be found in criteria.rs
    pub selection_criteria: criteria::Criteria,

    // output path
    pub name: String,

    // generation of the world
    pub generation: usize,
    time: u64, // time overall; 
    age_of_gen: u16,
    pub bots_alive: u16, 
    killed_bots: Vec<*mut objects::Bot>,

    // holding of the bots and blocks etc
    pub bot_vec: Vec<objects::Bot>,

    pub bot_register: Vec<Option<*mut objects::Bot>>,

    // vec of all clusters
    pub cluster_vec: Vec<cluster::Cluster>,

    // vec of all bots that are ready to form a cluster
    // id is saved in the vec
    cluster_ready_vec: Vec<*mut objects::Bot>,

    barrier_block_vec: Vec<objects::BarrierBlock>,

    // grid with coordinates of object
    pub grid: Vec<Vec<objects::Block>>,

    pub neuron_lib: Vec<usize>,

    pub grid_store: Vec<Vec<Vec<Kind>>>

    //
    // maybe add a vec of all generations
    //


}

