// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own a string data
pub fn run() {
    let mut world = String::from("World"); // String => Growable, heap-allocated data structure
    let hi = "Hello ";

    // Now there are 2 methods push and STR::push (string push)

    // Get Length
    println!("Length: {}", hi.len());

    // Push is only for one letter (char)
    //hi.push("W");

    // Push string
    world.push_str(" from the other side");

    // Capacity in bytes
    println!("Capacity: {}", world.capacity());

    // Check if empty
    println!("Is Empty: {}", world.is_empty());

    // Contains
    println!("Contains: {}", world.contains("World"));

    // Replace
    println!("Replace: {}", world.replace("World", "There"));

    // Loop through string by whitespace
    for word in world.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push_str("Valentino");
    s.push_str("Vizner");

    // Assertion testing
    assert_eq!(15, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hi, world));
}
