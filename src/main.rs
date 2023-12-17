#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
mod tools;

use world::{objects, neurons, criteria};
use neurons::GeneTrait;

use std::ffi::{CString, CStr};

fn main(){
    let mut main_world: world::World = world::World::new((2, 2),1 , criteria::Criteria::Area([(0,0), (2, 2)]), "C://users//julia//desktop//testp".to_string());
    
    main_world.spawn_bots();


    for i in 0..2{
        main_world.calculate_generation();
        println!("{}", neurons::valid_gene::<u32>(main_world.bot_vec[0].genome[0], &main_world.neuron_lib));
    }

    //main_world.bot_vec[0].draw_graph();



}
