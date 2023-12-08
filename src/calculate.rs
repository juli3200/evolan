
/*
extern "C"{
    fn calculate(a: i32);
}
*/
// calculates every output neuron of one bot

use rand::seq::index;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn moodified_sigmoid(x:  f64) -> f64 {
    2.0 * sigmoid(x) -1.0
}


pub fn calc_step(input_neurons: &Vec<Vec<[f64; 5]>>) -> Vec<usize>{

    // create a "dictionary" so i can index the neurons 
    // add the output layer
    let dic_vec: Vec<Vec<f64>> = {
        let mut c = 0;
        let mut a: Vec<Vec<f64>> = vec![];
        for layer in input_neurons.iter(){
            a.push(vec![]);
            for conn in layer{
                a[c].push(conn[3]);
            }
            c+=1;
        }
        a
    };
    ///
    /// 
    /// add dic_vec
    /// add output layer
    /// 
    /// 
    /// 
    /// 
    let mut output_vec = vec![0.0; crate::settings::OUTPUT_NEURONS as usize];

    let mut mut_bot_vec = input_neurons.clone();
    // calculate neurons
    // Iterate mutably over each layer in mut_bot_vec
    for layer in 0..input_neurons.len() {
        // Iterate over connections in the current layer
        for conn in 0..input_neurons[layer].len() {
            // Calculate the new value
            let calc_val: f64 = mut_bot_vec[layer][conn][0] * 
                (mut_bot_vec[layer][conn][4]/crate::settings::WEIGHT_DIVISION - crate::settings::WEIGHT_SUBTRACTION);
            
            // check if target neuron is a output neuron or inner neuron
            match input_neurons[layer][conn][2] as usize{
                // if inner neuron
                1..=crate::settings::INNER_LAYERS =>{
                        // get the neuron id from the dict
                        let neuron_index: usize;
                        //                                                 target layer                                         target neuron
                        if let Some(index) = dic_vec[input_neurons[layer][conn][2] as usize].iter().position(|&x| x == input_neurons[layer][conn][3]) {
                            neuron_index = index as usize;
                        } else {
                            panic!("Neuron not found")
                        }
                        // Update the value in mut_bot_vec using the indices from conn
                    
                        mut_bot_vec[input_neurons[layer][conn][2] as usize]
                        [neuron_index][0] += calc_val;

                    }
                // if output neuron
                crate::settings::OUTPUT_LAYER => {
                    output_vec[input_neurons[layer][conn][3] as usize] += calc_val;
                }
                _ => {panic!("layer not found!")}
        }
            
        }

        // activation:
        //      a moodified sigmoid function is used: 2*sigmoid(x) -1
        //      to get vals between -1 and 1

        if layer < input_neurons.len()-1{
            for conn in 0..input_neurons[layer].len() {
                mut_bot_vec[layer+1][conn][0] = sigmoid(mut_bot_vec[layer][conn][0]);
            }
        }
    }



    output_vec

}

pub fn calc_step_gpu(input_neurons: &Vec<Vec<[f64; 5]>>){

}