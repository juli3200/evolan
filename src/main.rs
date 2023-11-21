#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
mod tools;
use world::{objects, neurons, criteria};
use neurons::GeneTrait;

use std::ffi::{CString, CStr};

fn main(){
    let mut main_world: world::World = world::World::new((10, 10), 2);

    main_world.spawn_bots();
    println!("{:?}", main_world.bot_vec);


    let char_gene = vec!['4', '0', '9', 'F', 'A', 'F', '6' ,'3'];
    

    println!("{:?}\n", neurons::valid_gene(char_gene, &main_world.neuron_lib));
    
    println!("{:?}", main_world.neuron_lib);

    println!("{:08X}",main_world.bot_vec[0].genome[0]);
    neurons::mutate(&mut main_world.bot_vec[0].genome, &main_world.neuron_lib);
    println!("{:08X}",main_world.bot_vec[0].genome[0]);
    //main_world.bot_vec[0].draw_graph();

    let c_string = CString::new("hello aajshkajshkjas").expect("CString::new failed");
    unsafe { calculate::calculate(c_string.as_ptr()) };
}
