#[derive(Debug)]
enum GenderCat {
    Male,
    Female,
    TransGender
}

#[derive(Debug)]
struct Person {
    name:String,
    email:String,
    age:i32,
    gender:GenderCat
}

fn main() {
    println!("Enum advance pratice!");

    let person1 = Person {
        name:String::from("aayush chalise"),
        email:String::from("aayushchalise8@gmail.com"),
        age:21,
        gender:GenderCat::Male
    };

    println!("{:?}",person1);

    let result = cal(4);
    println!("{:?}",result);
}


// option enum

/*
enum option <T> {
    Some(T)
    None
}
*/ 

fn cal(no: i32) -> Option<bool> {
    if no % 2 == 0 {
        Some(true)
    } else {
        None
    }
}