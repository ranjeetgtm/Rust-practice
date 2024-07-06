fn main() {
    println!("Tuple Struct!");

    struct User (u8,u8,u8);

    let mut user1 = User(5,19,69);

    println!("{},{},{}",user1.0,user1.1,user1.2);

    user1.1 = 23;

    println!("{},{},{}",user1.0,user1.1,user1.2);


    struct UserDetails(String,i16,bool);

    let user2 = UserDetails(String::from("aayush"),21,true);
    println!("Name is {}, Age of user {}, and Is he male ? {}",user2.0,user2.1,user2.2);


}


