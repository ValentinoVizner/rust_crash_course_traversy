use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Vizzi";
    println!("Args: {:?}", args);

    // Input in terminal: cargo run
    // Output in terminal: Args: ["target/debug/rust_crash_course_traversy"]

    //Input in terminal: cargo run hello world
    //Output in terminal: Args: ["target/debug/rust_crash_course_traversy", "hello", "world"]

    if command == "hello" {
        println!("Hi {}, how are you?", name)
    }

    let args_multi: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}", args_multi[0]);

    // The rest of the arguments that are passed to command line parameters
    // Call program like this e.g.:
    // $ cargo run hello world
    println!(
        "I got {:?} arguments: {:?}.",
        args_multi.len(),
        &args_multi[1..]
    );
}
