extern crate csv;

use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;

const INPUT_NEURON_REGISTER_STRING: [&str; crate::settings::INPUT_NEURONS as usize] = 
    ["input_functions::always_true", "input_functions::always_false", 
    "input_functions::random", "input_functions::population_density", 
    "input_functions::population_density", "input_functions::population_size",
    "input_functions::age", "input_functions::time", "input_functions::x", 
    "input_functions::y", "input_functions::distance_nn", "input_functions::angle_nn",
    "input_functions::distance_nearest_boarder", "input_functions::distance_north_south",
    "input_functions::distance_west_east", "input_functions::blocked_angle",
    "input_functions::blocked_around"/* , "input_functions::average_letter", 
    "input_functions::mode_letter", "input_functions::length_letter"*/];

const OUTPUT_NEURON_REGISTER_STRING: [&str; crate::settings::OUTPUT_NEURONS as usize] =[ "output_functions::turn_left",
"output_functions::turn_right",
"output_functions::move_fw",
"output_functions::move_left",
"output_functions::move_right",
"output_functions::pos_x",
"output_functions::neg_x",
"output_functions::pos_y",
"output_functions::neg_y",
"output_functions::place_barrier_block",
"output_functions::mutate"];


fn write_to_csv(data: &Vec<[u32; 5]>) {
    let _ = fs::remove_file(format!("{}/network.csv", crate::settings::OUTPUT_FOLDER));
    // from chat.openai.com
    // Open the file, creating it if it doesn't exist, and emptying it if it does
    let mut file = File::create("output/network.csv").expect("Unable to create file");

    // Write the data to the CSV file
    writeln!(file, "Source,Target,weight").expect("Unable to write data to file");
    for row in data {
        let source = match row[0] {
          0 => {format!("{}", INPUT_NEURON_REGISTER_STRING[row[1] as usize])},
          _=>{format!("inner_neuron{}_{}", row[0] , row[1])}
        };
        let inner_layer2 = crate::settings::INNER_LAYERS - 1;

        let target = match row[2] as usize{
            crate::settings::INNER_LAYERS => {format!("{}", OUTPUT_NEURON_REGISTER_STRING[row[3] as usize])}
            _ => {format!("inner_neuron{}_{}", row[2]+1, row[3])},
            

        };
        writeln!(file, "{:?},{:?},{:.4}", source, target, row[4] as f32 / 2_i32.pow(15) as f32).expect("Unable to write data to file");
    }
}


pub fn main(data: &Vec<[u32; 5]>){
    write_to_csv(data);
    let output = Command::new("python")
        .args(&["src/tools/plot network/plot network.py"])
        .output()
        .expect("Failed to execute Python script");
    println!("{:?}", output);
}