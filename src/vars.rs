// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust block-scoped language

pub fn run() {
    let name = "Vizzi";
    let mut age = 26;
    println!("My name is {} and I am {}", name, age);
    // In Rust this is invalid:
    // age = 27; but we can add mutations (mut)
    age = 27;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: u32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Vizzi", 26);
    println!("My name is {} and I am {}", my_name, my_age);
}
