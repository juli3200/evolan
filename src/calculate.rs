
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

fn modified_sigmoid(x:  f64) -> f64 {
    2.0 * sigmoid(x) -1.0
}


pub fn calc_step(input_neurons: &Vec<Vec<[f64; 5]>>) -> Vec<usize>{

    // output vec is a vec of every layer where calculation results are stored
    let mut output_vec: Vec<Vec<Option<f64>>> = vec![];
    // inner layers
    for _ in 0..crate::settings::INNER_LAYERS{
        output_vec.push(vec![None; crate::settings::INNER_NEURONS])
    } // end for
    // output layer
    output_vec.push(vec![None; crate::settings::OUTPUT_NEURONS as usize]);

    // 
    // calculate neurons
    // iterate over every connection
    // calculate value and store it in the output_vec
    for layer in 0..input_neurons.len() {
        // Iterate over connections in the current layer
        for conn in 0..input_neurons[layer].len() {
            // pointer to the current conn
            let this_conn = &input_neurons[layer][conn];

            // Calculate the new value
            let calc_val: f64;


            match layer{
                // if input neuron the input value is taken and multiplied by the weight
                0 => {calc_val= this_conn[0] *
                (this_conn[4]/crate::settings::WEIGHT_DIVISION - crate::settings::WEIGHT_SUBTRACTION);},

                // if inner neuron the val stored in the output_vec is taken and multiplied by the weight
                // if no val stored => continue
                1..=crate::settings::INNER_LAYERS => {
                    // check if inner neuron is empty=> None or Some()
                    // if None => continue
                    // it's only Some if a input neuron has a connection to the inner neuron
                    match output_vec[layer ][this_conn[1] as usize] {
                        Some(val) => {
                            calc_val = val * // val times weight
                                (this_conn[4] / crate::settings::WEIGHT_DIVISION - crate::settings::WEIGHT_SUBTRACTION);},
                        None => continue }
                    }
                // shouldn't happen
                _ => {panic!("LAYER NOT FOUND")}
            
            } // end match
            
            // add calc val to the target neuron
            // if no val stored Some(calc_val) is stored

            match output_vec[this_conn[2] as usize -1]/*layer*/[this_conn[3] as usize]/*neuron*/ {
                Some(val) => output_vec[this_conn[2] as usize  -1][this_conn[3] as usize] = Some(val+ calc_val),
                None => output_vec[this_conn[2] as usize  -1][this_conn[3] as usize] = Some(calc_val)
            }


            
        } // end inner for (connections)

        // activation:
        //      a modified sigmoid function is used: 2*sigmoid(x) -1
        //      to get vals between -1 and 1

        if layer < input_neurons.len()-1{
            for neuron in output_vec[layer+1].iter_mut() {
                // if neuron is some(val) the val is compressed in the modified sigmoid function
                match *neuron {
                    Some(val) => {*neuron = Some(modified_sigmoid(val))}
                    None => continue
                }
            }
        }
    } // end outer for (layers)

    let mut activated_outputs = vec![];
    println!("{:?}", output_vec.last().unwrap());

    // access output layer from the output_vec
    for (index, output_neuron) in output_vec[output_vec.len()-1].iter().enumerate(){
        match output_neuron {
            Some(val) => {if *val >= 0.0{activated_outputs.push(index); println!("{}", index);}}
            None => continue
        }
    }


    activated_outputs

}

pub fn calc_step_gpu(input_neurons: &Vec<Vec<[f64; 5]>>){

}