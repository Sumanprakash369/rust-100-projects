use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice: u32 = choice.trim().parse().unwrap();

    println!("Enter temperature value:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let temp: f64 = input.trim().parse().unwrap();

    let result = if choice == 1 {
        // C → F
        temp * 9.0 / 5.0 + 32.0
    } else {
        // F → C
        (temp - 32.0) * 5.0 / 9.0
    };

    println!("Converted temperature: {:.2}", result);
}
