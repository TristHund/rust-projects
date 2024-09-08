use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    let num1: f64 = tokens[0].parse().expect("Not a valid number");
    let operator = tokens[1];
    let num2: f64 = tokens[2].parse().expect("Not a valid number");

    println!("{}", calculate(operator, num1, num2)); 
}

fn calculate(op: &str, num1: f64, num2: f64) -> f64 {
    match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Unknown operator"),
    }
}

