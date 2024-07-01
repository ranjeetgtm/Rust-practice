fn main() {
    println!("Reference!");

    let mut x = 68;

   {
    let z = &mut x;
    *z += 1;
    println!("The value of z is - {}", z);
   }

    println!("The value of x is - {}", x);
}
