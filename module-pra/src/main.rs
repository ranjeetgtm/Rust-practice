
// we use "use" keyword for importing any module and public function from different files
use rand;

//getting functions from another files
mod lib; 

// normal module
mod my_module {
     pub fn personal() {
        println!("Hello I am aayush");
    }
    }

    // nested module

    mod my_mod2 {
        pub mod movie {
            pub mod english {
                pub fn play(name:String) {
                    println!("Playing this english movie- {}",name);
                }
            }

            pub mod hindi {
                pub fn play1(name:String) {
                    println!("Playing this hindi movie- {}",name);
                }
            }
        }
    }

    use my_mod2::movie::english::play;
    use my_mod2::movie::hindi::play1;
fn main() {
    println!("Module Practice!");

    my_module::personal();

    lib::print_my_message();
    lib::print_my_message2();

    // let do some nested

    play("Titanic".to_string());
    play1("Kabhi Khusi Kabhi Gam".to_string());

} 
