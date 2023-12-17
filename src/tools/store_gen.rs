use serde::Serialize;
use serde_json::to_string;
use std::io::Write;
use std::fs::File;


#[derive(Debug, Clone, Serialize)]
pub enum Kind{
    Bot,
    BarrierBlock,
    Empty
}

pub fn store_step(world: &crate::world::World) -> Vec<Vec<Kind>>{
    let mut picture_vec = vec![vec![Kind::Empty; world.dim.0 as usize]; world.dim.1 as usize];
    for (y,row) in world.grid.iter().enumerate(){
        for (x, block) in row.iter().enumerate(){
            match block.guest{
                // if genome is returned must be bot
                Some(b) => {
                    match unsafe {&*b}.genome() {
                        None => picture_vec[y][x] = Kind::BarrierBlock,
                        Some(_) => picture_vec[y][x] = Kind::Bot
                    }
                }
                None => {picture_vec[y][x] = Kind::Empty}
            }
        }
    }

    return picture_vec;

}


pub fn store_generation(world: &crate::world::World){
    let json_string = to_string(&world.grid_store).unwrap();

    let path = format!("{}/generations/{}.json", world.path, world.generation);

    let mut file = File::create(path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

}

