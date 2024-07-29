
extern crate regex;

use regex::Regex;

fn main() {
    println!("Regex example!");

    //ex 1
    let re = Regex::new(r"\w{6}").unwrap();

    let myname = "Aayush";

    println!("Found the correct: - {}", re.is_match(myname));

    //ex2

    let re1 = Regex::new(r"\w{6}").unwrap();
    match re1.captures(myname) {
        Some(caps) => println!("Found the correct match - {}",caps.get(0).unwrap().as_str()),
        None => println!("Could not found the match")
    }
}
