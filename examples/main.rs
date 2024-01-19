use evolan::*;
use evolan::world::criteria;

pub static mut GENOME_LENGTH: usize = 16;

fn main(){

    let p = r"C:\Users\julia\Desktop\evolan_sims\killing_a_s";
    let mut settings_ = settings::Settings::use_template((100,100), 300, 150);
    let criteria_ = criteria::Criteria::Area([(0,0), (5,100)]);

    settings_.killing_enabled = true;

    let mut main_world: world::World = world::World::new(settings_, criteria_, "killing_and_storing".to_string());

    main_world.spawn_bots();


    for i in 0..100{
        main_world.calculate_generation();
        println!("{}", i)
    }

    match tools::save::save("killing_and_storing", p){
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
