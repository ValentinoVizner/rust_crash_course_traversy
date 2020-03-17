// Vectors - Resizable Vectors

use std::mem;

macro_rules! vec_of_strings {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

pub fn run() {
    let mut numbers: Vec<u8> = vec![0, 1, 2, 3];

    println!("{:?}", numbers);

    // GEt single value
    println!("Single value: {}", numbers[0]);

    // We can make it MUTatable
    let mut words = vec_of_strings!["Ja", "sam", "mala", "pa", "sam", "pala"];

    words[5] = std::string::String::from("srala");
    println!("{:?}", words);

    // Add on to vector
    numbers.push(5);

    // Pop off last number
    numbers.pop();

    // Get arra length
    println!("Vector length: {}", words.len());

    // Vectors are stac allocated
    println!("Vector occupies {} bytes", mem::align_of_val(&numbers));

    // Slice Vector
    let slice = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 10;
    }
    println!("Number: {:?}", numbers);
}
