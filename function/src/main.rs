fn main() {
    println!("Hey!");

    first_fn();
    
    //passing single parameter
    second_fn(42); // Call second_fn with a parameter

    //passing multiple parameter
    third_fn(42,'S');

    //expression ex
    ex();
}

//simple function
fn first_fn() {
    println!("This is a simple function");
}

//pass single parameter 
fn second_fn(x: i32) { // Add parameter to function definition
    println!("The value of x is - {}", x);
}

// multiple params
fn third_fn(x: i32, y: char) {
    println!("The value of x is - {x} and the value of y is - {y}");
}

// expression
fn ex () {
    let y = {
        let x = 7;
        x * 2
    };

    println!("value of y is - {}",y);
}
