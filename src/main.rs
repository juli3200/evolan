#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
mod tools;
use world::{objects, neurons, criteria};
use neurons::GeneTrait;



fn main(){
    let mut main_world: world::World = world::World::new((10, 10), 2);

    main_world.spawn_bots();
    println!("{:?}", main_world.bot_vec);


    let char_gene = vec!['0', 'C', '3', '1', '5', 'A', 'E' ,'6'];

    println!("{:?}", main_world.neuron_lib);

    println!("{:?}", 344448949.decode_gene());
    main_world.bot_vec[0].draw_graph();
}
