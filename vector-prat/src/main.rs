fn main() {
    println!("Vector Practice!");

    let mut xyz = vec![1,2,3,4,5,6];
    println!("{:?}", xyz);
    xyz.push(9);
    println!("{:?}", xyz);
    xyz.push(10);
    println!("{:?}", xyz);
    println!("{:?}", xyz[3]);
    xyz.push(100);

    let mut Vec1: Vec<i32> = Vec::new();
    Vec1.push(32);
    println!("{:?}", Vec1);

}
