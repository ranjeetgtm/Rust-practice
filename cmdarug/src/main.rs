

use std::{env, fmt::Arguments}; 

fn main() {
    println!("How to pass args in CMD");

    let args: Vec<String> = env::args().collect();

    // println!("{}",args[1]);

    for arguments in args.iter(){
        println!("{}",arguments);
    }
}
