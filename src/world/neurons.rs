use rand::Rng;


pub const OUTPUT_NEURONS: u8 = 2; // number of output neurons
#[derive(Debug, Clone)]
pub enum OutputNeurons{
    // zero is backwards 1 is forwards
    Move(bool),
    // angle witch it must be turned
    Turn(u16)

}

pub const INPUT_NEURONS: u8 = 3; // number of input neurons
#[derive(Debug, Clone)]
pub enum InputNeurons{
    Random(f32),
    Density(f32),
    Age(u16),


}

fn create_gene() -> u32{
    /*
    :return: one gene with 2-type_bits, 5 id bits, 2 type bits, 5 id bits and 18 weight bits in hex format
    eg: 0x10328899

    the first neuron can either be an input or an inner neuron ;
     0-> input, 1 -> first hidden layer; 2 -> second and 3 -> third
    the second neuron can either be an inner layer or an output neuron
     0->1. hidden layer...

    there can't be an index error because of lookup of the neuron_lib
    the neuron lib can be edited from the grid class
     */

    // number generator
    let mut rng = rand::thread_rng();

    // create numbers for type 1 and id 1

    let type_1 = rng.gen_range(0..(super::INNER_LAYERS+1)) as u32;
    let id_1 = rng.gen_range(0..Bot::neuron_lib[type_1 as usize].len()) as u32;

    let type_2 = rng.gen_range(0, Bot::neuron_lib.len() - 1) as u32;
    let id_2 = rng.gen_range(0, Bot::neuron_lib[(type_2 + 1) as usize].len()) as u32;




    let weight = rng.gen_range(0, 2u32.pow(18));

    let gene = ((type_1 << 23) | (id_1 << 18) | (type_2 << 13) | (id_2 << 8) | weight) as u32;

}


pub fn create_genome() -> [u32; super::GENOME_LENGTH]{

    let mut gene: [u32; super::GENOME_LENGTH] = [0u32; super::GENOME_LENGTH];
    for g in gene.iter_mut(){
        *g = create_gene();
    }

    let gene = gene;
    gene
}


