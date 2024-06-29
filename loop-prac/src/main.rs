fn main() {
    println!("loop-prac!");

    first();

    second();

    third();

    fourth();
}

//types of loops in rust
//loops
//while
//for

fn first() {

    let mut x = 0;
    loop {
        x += 1;
        println!("x = {}",x);
         
         if x == 5 {
            println!("it's done brother");
            break;
         }
    }
}

//ex of while loop
fn second() {
    let number = 0;
     
     while number != 0 {
        println!("{}",number);
     }

     println!("hey");
}

fn third() {
    let a = [10,20,30,40,50,60];

    let mut index = 0;

    while index < 6 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

//for loop

fn fourth() {
   for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("x is {}", x);
    }
    


}