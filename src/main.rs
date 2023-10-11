#![allow(dead_code, unused_variables, unused_imports)]
mod world;
use world::{objects, neurons};




fn main(){
    println!("Hello, world!");
    let v = vec![objects::Bot::new([0u32; world::GENOME_LENGTH]), objects::Bot::new([1u32; world::GENOME_LENGTH]), objects::Bot::new([2u32; world::GENOME_LENGTH])];
    for b in v.iter(){
        println!("{:?}", b)
    }
}
