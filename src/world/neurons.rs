use rand::Rng;

pub const INPUT_NEURONS: u8 = 3; // number of input neurons
#[derive(Debug, Clone)]
pub enum InputNeurons{
    Random(f32),
    Density(f32),
    Age(u16),


}


pub const OUTPUT_NEURONS: u8 = 2; // number of output neurons
#[derive(Debug, Clone)]
pub enum OutputNeurons{
    // zero is backwards 1 is forwards
    Move(bool),
    // angle witch it must be turned
    Turn(u16)

}

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
    let type_2 = rng.gen_range(0..=(super::INNER_LAYERS)) as u32; // 0 is for 1. layer as described above
    let id_2 = rng.gen_range(0..(*lib[type_1 as usize]+ 1)) as u32;

    // weight bits; 18 bits
    let weight = rng.gen_range(0..2u32.pow(18)); // 18 bits long number; is converted to a float between +-4

    let gene = ((type_1 << 30) | (id_1 << 25) | (type_2 << 23) | (id_2 << 18) | weight) as u32;

    gene

}


pub fn create_genome() -> [u32; super::GENOME_LENGTH]{
    // the neuron lib is a library whitch is used for the creation of the genes
    let mut neuron_lib: Vec<&usize> = Vec::new();
    neuron_lib.push(&(INPUT_NEURONS as usize));

    for _ in 0..super::INNER_LAYERS{
        neuron_lib.push(&super::INNER_NEURONS);
    }
    neuron_lib.push(&(OUTPUT_NEURONS as usize));

    let mut gene: [u32; super::GENOME_LENGTH] = [0u32; super::GENOME_LENGTH];
    for g in gene.iter_mut(){
        *g = create_gene(&neuron_lib);
    }

    let gene = gene;
    gene
}


pub fn valid_gene(gene: Vec<char>)-> bool{
    ///
    ///
    /// 
    /// 
    /// 
    /// 
    /// 
    /// todo: check if gene is valid 
    /// used in objects::inherit (mutation)
    /// create possibility to disable mixed genes from two parents(one parent/clone)
    /// next create spawn function
    /// 
    /// 
    /// 
    /// 
    /// 
}



