
extern crate rand;

use rand::Rng;

fn main() {
    println!("Random number!");

    let random_no = rand::thread_rng().gen_range(1..100);
    println!("Random number is : - {}", random_no);
}
