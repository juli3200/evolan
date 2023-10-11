use std::mem;

#[derive(Debug, Clone)]
pub enum OutputNeurons{
    // zero is backwards 1 is forwards
    Move(bool),
    // angle witch it must be turned
    Turn(u16)

}

pub fn create_gene(){
    //todo: create gene
}


