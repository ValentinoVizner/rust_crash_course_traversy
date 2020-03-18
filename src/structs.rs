// Structs - Used to create custom data types

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorTuple(u8, u8, u8);
pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", color.red, color.green, color.blue);

    color.red = 200;
    println!("Color: {} {} {}", color.red, color.green, color.blue);

    let mut color_tuple = ColorTuple(255, 255, 255);
    println!(
        "Color: {} {} {}",
        color_tuple.0, color_tuple.1, color_tuple.2
    );
    color_tuple.0 = 0;
    println!(
        "Color: {} {} {}",
        color_tuple.0, color_tuple.1, color_tuple.2
    );
}
