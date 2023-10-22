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
    fn decode_gene(&self) -> [u32; 5] {
        // convert the vec<char> in a string
        let string_gene: String = self.into_iter().collect();
        // convert the hex string in a u32 and perform the .decode_gene methode
        u32::from_str_radix(&string_gene, 16).expect("REASON").decode_gene()

    }
}


pub fn valid_gene(gene: Vec<char>, neuron_lib: &Vec<&usize>)-> bool{
    // checks if mutated gene is valid

    // decode Vec<char> gene 
    let decoded_gene: [u32; 5] = gene.decode_gene();

    // check
    if (decoded_gene[0]as usize) > super::INNER_LAYERS{return false;}
    if decoded_gene[1] as usize >= *neuron_lib[decoded_gene[0] as usize]{return false;}

    if (decoded_gene[2]as usize) > super::INNER_LAYERS{return false;}
    if decoded_gene[3] as usize >= *neuron_lib[decoded_gene[2] as usize]{return false;}

    true
}



