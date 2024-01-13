use std::process::Command;
fn main() {
    
    // Compile the CUDA source file using nvcc
    let output = Command::new("nvcc")
        .args(&["--lib", "src/live_view/live_view.cu", "-o", "lib/live_view.lib"])
        .output()
        .expect("Failed to compile live_view.cu with nvcc");

    if !output.status.success() {
        panic!("Failed to compile live_view.cu with nvcc:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    // Tell cargo to tell rustc to link the system live_view library.
    println!("cargo:rustc-link-lib=live_view");
    // Specify the path to the library
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/lib");


}
