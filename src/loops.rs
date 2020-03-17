// Loops - Used to iterate until a conditon is met
pub fn run() {
    let mut count = 0;
    /*
        // Infinite loop
        loop {
            count += 1;
            println!("Number of iterations: {}", count);

            if count == 20 {
                break;
            }
        }
    */
    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", count)
        }

        // Increment
        count += 1;
    }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", x)
        }
    }
}
