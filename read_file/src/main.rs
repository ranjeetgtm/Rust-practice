
use std::fs::File;
use std::io::prelude::*;



fn main() {
    // open the file
    let mut file = File::open("../intro.txt").expect( "Not able to find file");

    // read the file 
    let mut content = String::new();

    // process the file for reading
    file.read_to_string(&mut content).expect("Not able to read the file");
    println!("File contains this data: - \n {}", content);
}
