struct User {
    name: String,
    company: String,
    age: u32
}

struct Home {
    house_no: i32,
    house_name: String,
    you_lived: bool
}
fn main() {
    let mut U1 = User {
        name: String::from("Aayush Chalise"),
        company: String::from("Linux Server"),
        age: 69


    };

    let UH1 = Home {
        house_no:102,
        house_name: String::from("Chalise Niwas"),
        you_lived: true

    };

    U1.age = 21;

    println!("My name is {} & My age is {} I'am working in {}.", U1.name, U1.age, U1.company);
    println!("My house no is {} & House Name is {}, OfCourse I lived here {}", UH1.house_no, UH1.house_name, UH1.you_lived)

}
