fn main() {
    println!("Hello, world!");

    first_fn();
    second_fn(42); // Call second_fn with a parameter
}

//simple function
fn first_fn() {
    println!("This is a simple function");
}

//pass single parameter 
fn second_fn(x: i32) { // Add parameter to function definition
    println!("This is a function with a single parameter: {}", x);
}
