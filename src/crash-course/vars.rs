// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Vinicius";
    let mut age = 30;
    println!("My name is {} and I am {} years old", name, age);
    age = 29;
    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Vinicius", 29);
    println!("{} is {}", my_name, my_age);
}
