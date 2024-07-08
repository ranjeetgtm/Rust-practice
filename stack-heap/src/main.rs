fn main() {
    println!("Stack and Heap Memory!");

    stack_mem_ex();

    Heap_mem_ex();
}


//stack memory

fn stack_mem_ex() {
    let x = 6;
    let y = 9;

    let sum = add(x,y);
    println!("The sum of x {} and y {} is - {}",x,y,sum);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// Heap Memory

fn Heap_mem_ex() {
    let mut v = Vec::new(); //dynamically allocated memory for vector v

    v.push(1);
    v.push(22);
    v.push(45);
    v.push(78);
    v.push(322);

    println!("{:?}", v);

}