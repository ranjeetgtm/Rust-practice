use std::fs::File;
use std::io::Write;

fn main() {
    println!("Let's write into file");

    // Create a new file
    let mut my_file = File::create("output.txt").expect("Could not create file");
    
    // Write into file
    my_file.write_all(b"Welcome \n").expect("Not able to write into file");

    // Append more data
    my_file.write_all(b"Hey, I am aayush").expect("Not able to write");
}