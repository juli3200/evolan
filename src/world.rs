pub mod objects;

const GENOME_LENGTH: usize = 16;
type Dow = u8;

// trait for all Objects
trait ObjectTrait{
    fn pos(&self)->(Dow, Dow);
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

