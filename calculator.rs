use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Enter two numbers:");

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let num1: f64 = input1.trim().parse().unwrap_or(0.0);

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let num2: f64 = input2.trim().parse().unwrap_or(0.0);

    println!("Select an operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice: u32 = choice.trim().parse().unwrap_or(0);

    match choice {
        1 => println!("Result: {}", num1 + num2),
        2 => println!("Result: {}", num1 - num2),
        3 => println!("Result: {}", num1 * num2),
        4 => {
            if num2 != 0.0 {
                println!("Result: {}", num1 / num2);
            } else {
                println!("Division by zero is not allowed");
            }
        }
        _ => println!("Invalid choice"),
    }
}
