fn main() {
    println!("Data type in rust!");

    //scaler type-----------type where we store single value
    //integer, string, boolean, floating, char

    //length (8bit, 16, 32, 64, 128 arch) singed(i)-i8, i16 or unsinged(u)-u8, u16,

    let no = 9;
    println!("{}", no);
    
    let is_male = true;
    println!("{}", is_male);

    let char = "xyz";
    println!("{}", char);

    let dec = 9.5;
    println!("{}", dec);



//compound type ------type where we store multiple data at a time
// arrays, tuples, dictionary

//tuples

let tup = (32,42,56);
println!("{:?}", tup);

}
