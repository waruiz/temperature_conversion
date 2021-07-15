use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius.");
    println!("What is the Fahrenheit temperature?");

    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Invalid Fahrenheit.");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let celsius = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("It is {} degrees Celsius", celsius);
}
