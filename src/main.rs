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

    println!("\n{:?}",main_world.bot_vec[0].neurons_to_compute());
    println!("{:?}", main_world.bot_vec[0].calculate_input(&main_world));

    //main_world.bot_vec[0].draw_graph();

    main_world.calculate_step()

}
