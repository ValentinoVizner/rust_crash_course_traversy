// Arrays - Fixed list where elements are the same data types (different from Python)

use std::mem;

pub fn run() {
    let numbers: [i8; 4] = [0, 1, 2, 3];

    println!("{:?}", numbers);

    // GEt single value
    println!("Single value: {}", numbers[0]);

    // We can make it MUTatable
    let mut words = ["Ja", "sam", "mala", "pa", "sam", "pala"];

    words[5] = "srala";
    println!("{:?}", words);

    // Get arra length
    println!("Array length: {}", words.len());

    // Arrays are stac allocated
    println!("Array occupies {} bytes", mem::align_of_val(&numbers));

    // Slice array
    let slice = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
