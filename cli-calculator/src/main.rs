use regex::Regex;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let re = Regex::new(r"(\d+\.?\d*)\s*([\+\-\*/])\s*(\d+\.?\d*)").unwrap();

    if let Some(captures) = re.captures(&input) {
        let num1: f64 = captures[1].parse().unwrap();
        let operator = &captures[2];
        let num2: f64 = captures[3].parse().unwrap();

        println!("{}", calculate(operator, num1, num2));
    } else {
        println!("Invalid input.");
    }
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
