
// Dimension_of_world; type of dimension val; if it is higher than 255 change to u16
pub type Dow = u8;

// number of neurons (edit only if neuron is added)
pub const INPUT_NEURONS: u8 = 17; // number of input neurons; max 32
pub const OUTPUT_NEURONS: u8 = 12; // number of output neurons; MAX 32

pub const WEIGHT_DIVISION: f64 = 2_i64.pow(15) as f64;
pub const WEIGHT_SUBTRACTION: f64 = 4.0;

pub const GENOME_LENGTH: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Settings{
    // dimension
    pub dim: (Dow, Dow),
    pub n_of_bots: u16,
    // Length of genomes
    pub genome_length: usize, // not supported yet
    // Max val 3; because of gene generation (more bites assigned to the index bits)
    pub inner_layers: usize,
    // Index of output layer
    pub output_layer: usize,
    // Inner neurons per inner layer
    pub inner_neurons: usize,
    // Mutation enabled for performance
    pub mutation_enabled: bool,
    // Mutation rate of the genes (one hexadecimal letter will be changed)
    pub mutation_rate: f64,
    // How many steps the bots take until selection occurs
    pub generation_steps: u16,
    // Communication radius
    pub comm_radius: Dow,
    // Activates sexual reproduction
    pub inherit: bool,

    // Killing neurons enabled?
    pub killing_enabled: bool,
    // Is the mutation neuron active?
    pub neuronal_mutation_enabled: bool,
    // Mutation rate for neurons
    pub neuronal_mutation_rate: f64,
    // Size of the area for the calculation for the density or nearest neighbor; side length
    pub search_area: u32,
    // Chance of placing a barrier block when neuron is fired
    pub barrier_block_blockade: f64,

    // Performance flag for GPU
    pub gpu: bool,
    // Backwards connections enabled?
    pub backwards_enabled: bool,

}

impl Settings{
    pub fn use_template(dim:(Dow, Dow), n_of_bots: u16, generation_steps: u16) -> Self{
        let inner_layers = 1;
        Settings{
            dim,
            n_of_bots,
            genome_length: 16,
            inner_layers,
            output_layer : inner_layers + 1,
            inner_neurons : 2,
            mutation_enabled : true,
            mutation_rate : 0.001,
            generation_steps,
            comm_radius : 10,
            inherit : true,
            
            killing_enabled : false,
            neuronal_mutation_enabled : true,
            neuronal_mutation_rate : 0.0005,
            search_area : 20,
            barrier_block_blockade : 0.0,
            
            gpu : false,
            backwards_enabled: false}
    }
}
