// Dimension_of_world; type of dimension val; if it is higher than 255 change to u16
pub type Dow = u8;

// number of neurons (edit only if neuron is added)
pub const INPUT_NEURONS: u8 = 19; // number of input neurons; max 32
pub const OUTPUT_NEURONS: u8 = 12; // number of output neurons; MAX 32

// general settings
pub const GENOME_LENGTH: usize = 16; // lenght of genomes
pub const INNER_LAYERS: usize = 1; // max val 3; because of gene generation (more bites assigned to the index bits)
pub const INNER_NEURONS: usize = 2; // inner neurons per inner layer
pub const MUTATION_ENABLED: bool = true; // used for performance 
pub const MUTATION_RATE: f64 = 0.001; // mutation rate of the genes (one hexadecimal letter will be changed)

// neurons settings
pub const KILLING_ENABLED: bool = false;
pub const NEURONAL_MUTATION_ENABLED: bool = true; // is the mutation neuron active
pub const NEURONAL_MUTATION_RATE: f64 = 0.001;
pub const DENSITY_SIZE: u32 = 100; // how big the area is for the calculation for the density

