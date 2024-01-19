use std::os::raw::c_int;
#[link(name = "render_img")]
extern {
    pub fn main() -> c_int;
    pub fn test() -> c_int;
}

fn t(){
    println!("")
}