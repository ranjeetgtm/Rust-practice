fn main() {
    println!("owernship!");

    first();
}


fn first(){

    let x = 90; //----8bit
    println!("{}",x);

    let y = x; 
    println!("{}",y);

    //----------------------------

    let a = String::from("aayush chalise");
    let b = a.clone();

    println!("{}---{}", a,b);
}