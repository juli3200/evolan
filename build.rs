fn main() {
    /*
    let mut outputs: Vec<Output> = vec![];
    
    // Compile the CUDA source file using nvcc
    outputs.push(Command::new("make")
        .args(&["makefile"])
        .output()
        .expect("Failed to compile with nvcc"));

    for o in outputs.into_iter(){
        if !o.status.success() {
            panic!("Failed to compile live_view.cu with nvcc:\n{}", String::from_utf8_lossy(&o.stdout));
        }
    }*/

    /*
    println!("cargo:rustc-link-search=native=lib");
    println!("cargo:rustc-link-lib=static=render_img");
    */


}
