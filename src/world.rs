pub mod objects;
pub mod neurons;

// lenght of genomes
pub const GENOME_LENGTH: usize = 16;
pub const INNER_LAYERS: usize = 1;
pub const INNER_NEURONS: usize = 2; // inner neurons per inner layer


// Dimension_of_world; type of dimension val; if it is higher than 255 change to u16
pub type Dow = u8;

// trait for all Objects
pub trait ObjectTrait{
    // pos fn for every object
    fn pos(&self)->(Dow, Dow);
    fn spawn(& mut self, world: &World);
}

impl std::fmt::Debug for dyn ObjectTrait{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        write!(f, "{:?}", self.pos())
    }
}



#[derive(Debug)]
pub struct World{
    //setting dimension of the world; (u8, u8)
    dim: (Dow, Dow),

    //number of bots and blocks etc...
    n_of_bots: u16,
    n_of_barrier_blocks: u16,

    // holding of the bots and blocks etc
    bot_vec: Vec<objects::Bot>,
    barrier_block_vec: Vec<objects::BarrierBlock>,

    // grid with pointer to objects
    grid: Vec<Box<dyn ObjectTrait>>

}

impl World{
    pub fn new(dim: (Dow, Dow), n_of_bots: u16, barrier_blocks_pos: Vec<(Dow, Dow)>) {
        {
            // all variables get out of scope 
            let n_of_bots_usize: usize = n_of_bots as usize;
            // checking input
            if dim.0 == Dow::MAX || dim.1 == Dow::MAX{panic!("dim.0/dim.1 must be smaller than Dow::Max; buffer needed")}
            if dim.0 as usize * (dim.1 as usize) < (n_of_bots_usize+barrier_blocks_pos.len()){
                panic!("number of objects must be smaller than dim.0*dim.1")}
        }

        // barrier_blocks_pos is a vector of every barrier_block
        let mut bot_vec: Vec<objects::Bot> = vec![];
        for i in 0..n_of_bots{
            bot_vec.push(objects::Bot::new(neurons::create_genome()));
        }
        let mut barrier_block_vec: Vec<objects::BarrierBlock>= vec![];
        

        /* 
        World { dim: dim, 
                n_of_bots: n_of_bots, 
                n_of_barrier_blocks: barrier_blocks_pos.len() as u16, 
                bot_vec: bot_vec, 
                barrier_block_vec: (), 
                grid: () }
        */
    }
}
