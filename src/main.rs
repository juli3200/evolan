#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
mod tools;

use world::{objects, neurons, criteria};
use neurons::GeneTrait;

use std::ffi::{CString, CStr};

fn main(){
    let p = "C://users//julia//desktop//test_moving_area".to_string();
    let mut main_world: world::World = world::World::new((80, 80),200 , criteria::Criteria::Area([(0,0), (5, 80)]), p);
    
    main_world.spawn_bots();


    for i in 0..500{
        main_world.calculate_generation();
        println!("{}", i)
    }
    main_world.selection_criteria = criteria::Criteria::Area([(0,0), (5, 40)]);
    for i in 0..200{
        main_world.calculate_generation();
        println!("{}", i+500)
    }
    main_world.selection_criteria = criteria::Criteria::Area([(0,0), (80, 10)]);
    for i in 0..200{
        main_world.calculate_generation();
        println!("{}", i+700)
    }
    main_world.selection_criteria = criteria::Criteria::Area([(70, 0), (80, 80)]);
    for i in 0..200{
        main_world.calculate_generation();
        println!("{}", i+900)
    }
    //main_world.bot_vec[0].draw_graph();



}
