/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bytes they take in memory)
Floats: f32, f64
Booleans: bool (true/false)
Chars: char
Tuples: tuple
Arrays: array
*/

// Rust is a statically typed language, which means that it must now the types of all variables at compile time, however,
// the compiler can usually infer what type we want to use based on the value and how we use it

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 45454545454545454;

    // find max size
    println!("Max size i32: {}", std::i32::MAX);
    println!("Max size i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Get Boolean from expression
    let is_greater = 10 < 5;
    let a1 = "ab";
    let face = "\u{1F600}";
    println!("{:?}", (is_greater, a1, face));
}
