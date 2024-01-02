use serde::Serialize;
use serde_json::to_string;
use std::io::Write;
use std::fs::File;
use crate::world::Kind;



pub fn store_step(world: &crate::world::World) -> Vec<Vec<Kind>>{
    let mut picture_vec = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];
    for (y,row) in world.grid.iter().enumerate(){
        for (x, block) in row.iter().enumerate(){
            picture_vec[y][x] = block.guest;
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

