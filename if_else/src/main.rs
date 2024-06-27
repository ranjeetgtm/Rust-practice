fn main() {
    println!("if, if-else!");

    first();

    second();

    third();
}
// ----- single line comments
/*
multiple line comments
*/
fn first() {
    let no = 4;

    if no > 5 {
        println!("The condition is ture");
     } else {
        let abc = 68+1;
        println!("{abc}");
        println!("The condition is false");
    }
}

fn second() {
    let no = 3;

    if no / 6 == 0 {
        println!("Number is divisible by 6");
    }else if no / 8 == 0{
        println!("Number is divisible by 8");
    }else if no / 10 == 0{
        println!("This no is out of syllabus");
    }

 }

 fn third () {
    let condition = false;
    let number = if condition {5} else {0};

    println!("The value of number is {}",number);
 }