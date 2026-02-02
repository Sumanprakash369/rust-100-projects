// Standard input/output ke liye io module import
use std::io;

fn main() {
    // Program ka title
    println!("=== Simple Calculator ===");
    println!("Example input: 10 + 5");
    println!("Supported operators: +  -  *  /");

    // User input store karne ke liye mutable String
    // String heap par hoti hai (ownership matters)
    let mut input = String::new();

    // stdin se ek poori line read karo
    // &mut input => mutable borrow (ownership transfer nahi)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // trim(): newline hataata hai
    // split_whitespace(): spaces ke base par todta hai
    // collect(): Vec<&str> banata hai (string slices, ownership nahi)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    // Expected format: number operator number
    // Agar 3 parts nahi mile to error
    if parts.len() != 3 {
        println!("Invalid format. Use: <number> <operator> <number>");
        return; // early exit
    }

    // First number parse karo (string → f64)
    let left: f64 = parts[0]
        .parse()
        .expect("Left operand must be a number");

    // Operator ko &str ke roop me rakho (borrowed)
    let operator: &str = parts[1];

    // Second number parse karo
    let right: f64 = parts[2]
        .parse()
        .expect("Right operand must be a number");

    // match expression: Rust ka safe switch
    // Ye ek expression hai → value return karta hai
    let result: f64 = match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => {
            // Division by zero check
            if right == 0.0 {
                println!("Error: Division by zero");
                return;
            }
            left / right
        }
        _ => {
            println!("Unsupported operator");
            return;
        }
    };

    // Final result print
    println!("Result = {}", result);
}
