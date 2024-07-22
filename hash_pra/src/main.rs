
use std::{collections::HashMap, hash::Hash};  // way to import packages.




//declaring new hashmap as a marks variable 
fn main() {
    println!("Hashmap Practice!");
    
    let mut marks: HashMap<&str, i32> = HashMap::new();

    // adding values to the hashmap
    marks.insert("Rust", 60);
    marks.insert("Java", 10);
    marks.insert("Python", 90);
    marks.insert("Javascript", 89);
    marks.insert("Golang", 80);
    marks.insert("Science", 100);
    

    println!("{:?}",marks);

    // find the length
    println!("How many subjects you have studied - {}", marks.len());

    // lets match the value
    match marks.get("Science") {
        Some(marks) => println!("You got {} for the science", marks),
        None => println!("You dit not studied this subject")
    }

    // Remove the value
    marks.remove("Java");
    println!("Subject remains {}, Remaining subjects are - {:?}",marks.len(),marks);

    // loop through hashmap
    for (subject, mark) in &marks {
        println!("for {} you got {} marks", subject,mark);

        // check the value
        println!("Did you study c++ {}", marks.contains_key("c++"));
    }      
}           
