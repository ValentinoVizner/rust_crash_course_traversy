pub fn run() {
    // Print Console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Vaba", "Kutina");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Vaba", "Kutina", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Vizi",
        activity = "Basketball"
    );

    // Placeholder traits
    println!(
        "{0} Decimal is Binary: {0:b}, Hex: {0:x}, Octal: {0:o},",
        10
    );

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
