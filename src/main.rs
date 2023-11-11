#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
use world::{objects, neurons, criteria};




fn main(){
    let main_world: world::World = world::World::new((254, 254), 100);

    let barrier_blocks = vec![()];
    
}
