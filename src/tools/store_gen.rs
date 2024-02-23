use serde_json::to_string;
use std::io::Write;
use std::fs::File;
use crate::settings::{self, Dow};
use crate::world::Kind;



pub fn store_step(world: &crate::world::World) -> Vec<Vec<Kind>>{
    // Represents a 2D vector of Kind values used for storing pictures.
    let mut picture_vec = vec![vec![Kind::Empty; world.settings_.dim.0 as usize]; world.settings_.dim.1 as usize];
    for (y,row) in world.grid.iter().enumerate(){
        for (x, block) in row.iter().enumerate(){
            picture_vec[y][x] = block.guest.clone();
        }
    }

    return picture_vec;

}

fn restructure_grid(world: &crate::world::World) -> Vec<Vec<Vec<(Dow, Dow)>>>{
    // Restructures the grid into a vector of generations, each containing a vector of bots and a vector of barriers.
    let mut list_of_gens: Vec<Vec<Vec<(Dow, Dow)>>> = vec![];

    // Iterate over all steps
    for step in 0..world.grid_store.len(){
        
        // create a vector of bots and barriers 
        let mut bots: Vec<(Dow, Dow)> = vec![];
        let mut barriers: Vec<(Dow, Dow)> = vec![];
        let mut clusters: Vec<(Dow, Dow)> = vec![];
        
        for (y, row) in world.grid_store[step].iter().enumerate(){
            for (x, block) in row.iter().enumerate(){
                match block{
                    Kind::Bot(_) => bots.push((x as Dow, y as Dow)),
                    Kind::BarrierBlock => barriers.push((x as Dow, y as Dow)),
                    Kind::Cluster(_) => clusters.push((x as Dow, y as Dow)),
                    _ => {}
                }
            }
        }

        list_of_gens.push(vec![bots, barriers, clusters]);
        
    }

    list_of_gens
    
}

pub fn store_generation(world: &crate::world::World){
    let json_string = to_string(&restructure_grid(world)).unwrap();
    let path = format!(".cache/worlds/{}/generations/{}.json", world.name, world.generation);

    let mut file = File::create(path).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

}

