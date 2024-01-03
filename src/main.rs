#![allow(dead_code, unused_variables, unused_imports)]
mod world;
mod calculate;
mod settings;
mod tools;
mod tests;

use world::neurons::GeneTrait;
use world::criteria;

pub static mut GENOME_LENGTH: usize = 16;

fn main(){
    /* 
    let p = "/home/julianheer/output/killing".to_string();
    let mut settings_ = settings::Settings::use_template((100,100), 300, 150);
    let criteria_ = criteria::Criteria::Area([(0,0), (5,100)]);

    settings_.killing_enabled = true;

    let mut main_world: world::World = world::World::new(settings_, criteria_, "killing_and_storing".to_string());
    
    main_world.spawn_bots();


    for i in 0..100{
        main_world.calculate_generation();
        println!("{}", i)
    }
    */
    match tools::save::save("killing_and_storing", "/mnt/c/Users/julia/desktop/killing_a_s.evolan"){
        Ok(_) => {println!("saved! ")},
        Err(e) => {println!("{e}"); panic!("{e}")}
    }

    /* 
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
    */



}
