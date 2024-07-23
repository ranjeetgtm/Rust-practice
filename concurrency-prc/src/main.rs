
// importing some packages
use std::thread;
use std::time::Duration;



fn main() {
    println!("Concurrency in rust!");
    // create a new thread to run paralley with main function
    thread::spawn( || {
        for i in 1..10 {
            println!("The numbers are {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //code run from main function
    for i in 1..5 {
        println!("The numbers are {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}
