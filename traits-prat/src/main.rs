// trait Printable{
//     fn print(&self);
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Printable for Rectangle {
//     fn print(&self) {
//         println!("Rectangle: {} * {}", self.width, self.height);
//     }
// }

use std::f64;


trait HasArea {
    fn area(&self) -> f64;
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
       
    }

}

fn print_area <T:HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
    println!("Traits Practice!");

    // let rect1 = Rectangle {width: 18, height: 11};
    // rect1.print();
    let sqr1 = Square {
        x: 12.0,
        y: 12.0,
        side: 10.0
    };

    print_area(sqr1);




}