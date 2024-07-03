
enum CarTypes {
    Hatchback,
    Sedan,
    Suv,
    Muv,

}

fn printCars(car:CarTypes) {
    match car {
        CarTypes::Hatchback => {
            println!("Small car in a segment");
        }
    
    CarTypes::Sedan => {
        println!("Luxury car in a segment");
    }
    
    CarTypes::Suv => {
        println!("Sports utility based vechile");
    }

    CarTypes::Muv => {
        println!("Multi utility based vechile");
    }


    }
   
}


fn main() {
    println!("Enum prat!");

    let x = printCars(CarTypes::Suv);
    printCars(CarTypes::Muv);

}
