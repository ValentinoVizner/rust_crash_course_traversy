// Conditionals - Used to check the conditons of something and act upon it

pub fn run() {
    let age = 22;
    let check_id = true;
    let knows_person = true;

    // If/Else
    if age >= 21 && check_id || knows_person {
        println!("Bartender: What would you like to drink?");
    } else if age <= 21 && check_id {
        println!("Bartender: Sorry, you are too young to drink.");
    } else {
        println!("Bartender: I'll need see your ID");
    }

    // Shorthand If
    let is_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_age);
}
