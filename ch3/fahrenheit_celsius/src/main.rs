use std::io;

fn main() {
    println!("Input temperature in fahrenheit:");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Not a number");

    let celsius = (fahrenheit - 32.0) / 1.8;

    println!("That's {} celsius", celsius);
}
