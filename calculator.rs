use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num1: i32 = input.trim().parse().unwrap();

    input.clear();
    println!("Enter the second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let num2: i32 = input.trim().parse().unwrap();

    println!("Sum: {}", num1 + num2);
    println!("Difference: {}", num1 - num2);
    println!("Product: {}", num1 * num2);
    println!("Quotient: {}", num1 / num2);
}
