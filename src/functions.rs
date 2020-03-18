// Functions - Used t ostore bloc of code for re-use

pub fn run() {
    greeting("Hi", "Marija");
    add(5, 5);

    // Bind function values to Variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    // cool thing we can do in Closure and what we can't do in functions (because it's block-scoped):
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(5, 5));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
