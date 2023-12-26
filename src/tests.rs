#![cfg(test)]

#[test]
fn test_storing(){
    use crate::tools::save;
    let input = &String::from(r"C:\Users\julia\Desktop\test_moving_area2\generations");
    let output = &String::from(r"C:\Users\julia\Desktop\test_moving_area2\test.evolan");
    match save::save(input, output){
        Ok(_) => {},
        Err(e) => {panic!("{e}")}
    } 
}

#[test]
fn test_extracting(){
    use crate::tools::load;
    let output = &String::from(r"C:\Users\julia\Desktop\test_moving_area2\generations2");
    let input = &String::from(r"C:\Users\julia\Desktop\test_moving_area2\test.evolan");
    match load::load_into_folder(input, output){
        Ok(_) => {},
        Err(e) => {panic!("{e}")}
    } 
}