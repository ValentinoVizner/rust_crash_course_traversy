// Functions - Used t ostore bloc of code for re-use

pub fn run() {
    greeting("Hi", "Marija");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}
