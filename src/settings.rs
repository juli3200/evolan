pub const OUTPUT_FOLDER: &str = "output";
// Dimension_of_world; type of dimension val; if it is higher than 255 change to u16
pub type Dow = u8;

// number of neurons (edit only if neuron is added)
pub const INPUT_NEURONS: u8 = 17; // number of input neurons; max 32
pub const OUTPUT_NEURONS: u8 = 12; // number of output neurons; MAX 32

// general settings
pub const GENOME_LENGTH: usize = 16; // length of genomes
pub const INNER_LAYERS: usize = 1; // max val 3; because of gene generation (more bites assigned to the index bits)
pub const OUTPUT_LAYER: usize = INNER_LAYERS + 1;
pub const INNER_NEURONS: usize = 1; // inner neurons per inner layer
pub const MUTATION_ENABLED: bool = false; // used for performance 
pub const MUTATION_RATE: f64 = 0.0001; // mutation rate of the genes (one hexadecimal letter will be changed)
pub const GENERATION_STEPS: u16 = 60; // how many steps the bots take until selection occurs
pub const COMM_RADIUS: Dow = 10;
pub const INHERIT:bool = true; // activates sexual reproduction

// neurons settings
pub const KILLING_ENABLED: bool = true;
pub const NEURONAL_MUTATION_ENABLED: bool = false; // is the mutation neuron active
pub const NEURONAL_MUTATION_RATE: f64 = 0.0005;
pub const SEARCH_AREA: u32 = 20; // how big the area is for the calculation for the density or neaerst neighbour; side length
pub const BARRIER_BLOCK_BLOCKADE: f64 = 0.0; // chance of placing a barrier block when neuron is fired

// performance
pub const GPU: bool = false;
pub const BACKWARDS_ENABLED: bool = false; // backwards connections enabled?
pub const WEIGHT_DIVISION: f64 = 2_i64.pow(15) as f64;
pub const WEIGHT_SUBTRACTION: f64 = 4.0;