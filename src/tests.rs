#![cfg(test)]

#[test]
fn test_storing(){
    use crate::tools::save;
    let input = &String::from("/home/julianheer/Desktop/test_for_storing/o");
    let output = &String::from("/home/julianheer/Desktop/test_for_storing/test.evolan");
    match save::save(input, output){
        Ok(_) => {},
        Err(e) => {panic!("{e}")}
    } 
}