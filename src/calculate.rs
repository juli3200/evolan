use std::os::raw::c_char;

#[link(name = "calc_network")]
extern "C"{
    pub fn calculate(a: *const c_char);
}
// calculates every output neuron of every bot
pub fn calc_step(input_neurons: &Vec<Vec<[f64; 2]>>){

}

pub fn calc_step_gpu(input_neurons: &Vec<Vec<[f64; 2]>>){

}