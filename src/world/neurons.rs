use rand::{Rng, seq::SliceRandom};

pub mod input_functions;
mod output_functions;
// the neuron register is used to convert genes to real values
 
use crate::world::{World, objects::Bot};

pub const INPUT_NEURON_REGISTER: [fn(&Bot, &World) -> f64; crate::settings::INPUT_NEURONS as usize] = 
    [input_functions::always_true, input_functions::always_false, 
    input_functions::random, input_functions::population_density, 
    input_functions::population_density, input_functions::population_size,
    input_functions::age, input_functions::time, input_functions::x, 
    input_functions::y, input_functions::distance_nn, input_functions::angle_nn,
    input_functions::distance_nearest_boarder, input_functions::distance_north_south,
    input_functions::distance_west_east, input_functions::blocked_angle,
    input_functions::blocked_around /*, input_functions::average_letter, 
    input_functions::mode_letter, input_functions::length_letter*/];

 /*[&fn(&Bot, &World) -> f64; crate::settings::OUTPUT_NEURONS as usize]*/
pub const OUTPUT_NEURON_REGISTER:  [fn(&mut Bot, &mut World); crate::settings::OUTPUT_NEURONS as usize] =
[ output_functions::turn_left,
output_functions::turn_right,
output_functions::move_fw,
output_functions::move_left,
output_functions::move_right,
output_functions::pos_x,
output_functions::neg_x,
output_functions::pos_y,
output_functions::neg_y,
output_functions::place_barrier_block,
output_functions::mutate];
 
 

 // ✅
fn create_gene(lib: &Vec<&usize>) -> u32{
    /*
    :return: one gene with 2-type_bits, 5 id bits, 2 type bits, 5 id bits and 18 weight bits in hex format
    eg: 0x10328899

    the first neuron can either be an input or an inner neuron ;
     0-> input, 1 -> first hidden layer; 2 -> second and 3 -> third
    the second neuron can either be an inner layer or an output neuron
     0->1. hidden layer, 1 -> second hidden layer, super::INNER_LAYERS -> OUTPUT_NEURON

    
    */

    // number generator
    let mut rng = rand::thread_rng();
    
    // neuron bits; 14 bits
    // create type and id for first neuron
    let type_1 = rng.gen_range(0..=(super::INNER_LAYERS)) as u32; // 0 is Input, 1 is first inner layer etc...
    let id_1 = rng.gen_range(0..*lib[type_1 as usize]) as u32;

    // create type and id for second neuron
    // if bakwards connections enabled random else must be bigger then type 1
    let start_val = match crate::settings::BACKWARDS_ENABLED{
        true => 0,
        false => type_1
    };
    let type_2 = rng.gen_range(start_val as usize..=(super::INNER_LAYERS)) as u32; // 0 is for 1. layer as described above
    let id_2 = rng.gen_range(0..(*lib[type_2 as usize + 1])) as u32;

    // weight bits; 18 bits
    let weight = rng.gen_range(0..2u32.pow(18)); // 18 bits long number; is converted to a float between +-4

    let gene = ((type_1 << 30) | (id_1 << 25) | (type_2 << 23) | (id_2 << 18) | weight) as u32;

    gene

}

// ✅
pub fn create_genome(neuron_lib: &Vec<&usize>) -> [u32; super::GENOME_LENGTH]{
    let mut gene: [u32; super::GENOME_LENGTH] = [0u32; super::GENOME_LENGTH];
    for g in gene.iter_mut(){
        *g = create_gene(neuron_lib);
    }

    let gene = gene;
    gene
}

// trait for u32 and Vec<char> for decode gene
pub trait GeneTrait{
    fn decode_gene(&self) -> [u32; 5];
}

impl GeneTrait for u32{
    // ✅
    fn decode_gene(&self) -> [u32; 5]{
        let type_1_mask:u32 = 0b11 << 30;
        let id_1_mask: u32 = 0b11111 << 25;
        let type_2_mask: u32 = 0b11 << 23;
        let id_2_mask: u32= 0b11111 << 18;
        let weight_mask: u32 = 0b111111111111111111;
    
        // Extract values using bit masking and shifting
        let type_1 = (*self & type_1_mask) >> 30;
        let id_1 = (*self & id_1_mask) >> 25;
        let type_2 = (*self  & type_2_mask) >> 23;
        let id_2 = (*self & id_2_mask) >> 18;
        let weight = *self & weight_mask;

        [type_1, id_1, type_2, id_2, weight]
    }
}

impl GeneTrait for Vec<char> {
    // ✅
    fn decode_gene(&self) -> [u32; 5] {
        // convert the vec<char> in a string
        let string_gene: String = self.into_iter().collect();
        // convert the hex string in a u32 and perform the .decode_gene methode
        u32::from_str_radix(&string_gene, 16).expect("REASON").decode_gene()

    }
}

// ✅
pub fn valid_gene(gene: Vec<char>, neuron_lib: &Vec<&usize>)-> bool{
    // checks if mutated gene is valid

    // decode Vec<char> gene 
    let decoded_gene: [u32; 5] = gene.decode_gene();

    // check
    if (decoded_gene[0]as usize) > super::INNER_LAYERS{return false;}
    else if decoded_gene[1] as usize >= *neuron_lib[decoded_gene[0] as usize]{return false;}

    else if (decoded_gene[2]as usize) > super::INNER_LAYERS{return false;}
    else if decoded_gene[3] as usize >= *neuron_lib[(decoded_gene[2] +1 )as usize]{return false;}

    else{true}
}


// ✅
pub fn mutate(genome: &mut[u32; super::GENOME_LENGTH], neuron_lib: &Vec<&usize>){
    // mutation

    let hex_letters = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    let mut c1 = 0;
    // c1 is the counter of the outer for loop
    for gene in genome.iter_mut(){

        let mut rng = rand::thread_rng();
        
        println!("{:08X}", gene);

        let mut hex_gene: Vec<char> = format!("{:08X}", gene).chars().collect(); // convert u32 in hex string
        
        let og_gene = hex_gene.clone();

        // go through every letter and change it by chance
        // c2 is the counter of the inner for loop
        let mut c2 = 0;
        for letter in hex_gene.iter_mut(){

            match rng.gen_bool(super::MUTATION_RATE) { 
                // if mutation occurs, the loop searches for a new random valid gene 
                // the validaty is checked with the neurons::valid_gene fn
                // if valid the new_letter is assigned to the *letter
                true => *letter =   
                    loop{// choose a random char from the hex_letters array
                        let new_letter = *hex_letters.choose(&mut rng).unwrap();
                        // the new_gene is a copy of the gene
                        let mut new_gene = og_gene.clone();
                        new_gene[c2] = new_letter; // the new letter is changed and checked
                        
                        match valid_gene(new_gene, neuron_lib){
                            true=>break new_letter,
                            false=> continue
                        }
                    }, 
                false => {}
            }
            c2+=1;    
        }
        c1+=1;
        // end of for

        // convert the new gene in a u32

        // convert the vec<char> in a string
        let string_gene: String = hex_gene.into_iter().collect();
        // convert the hex string in a u32 and perform the .decode_gene methode
        *gene = u32::from_str_radix(&string_gene, 16).expect("REASON");
    }
}



