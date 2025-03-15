use std::io;

fn main() {
    println!("Simple Rust Calculator");
    
    // Input the first number
    let num1 = get_number("Enter the first number: ");
    
    // Input the operator
    let op = get_operator();
    
    // Input the second number
    let mut num2 = get_number("Enter the second number: ");

    // Handle division by zero
    if op == '/' && num2 == 0.0 {
        loop {
            println!("Cannot divide by zero! Enter another number:");
            num2 = get_number("Enter the second number: ");
            if num2 != 0.0 {
                break;
            }
        }
    }

    // Calculate the result
    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => unreachable!(),
    };

    println!("Result: {} {} {} = {}", num1, op, num2, result);
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number!"),
        };
    }
}

fn get_operator() -> char {
    loop {
        println!("Enter an operator (+, -, *, /):");
        
        let mut op = String::new();
        io::stdin()
            .read_line(&mut op)
            .expect("Failed to read input");
        
        let op = op.trim();
        
        if op.len() != 1 {
            println!("Enter exactly one operator character");
            continue;
        }
        
        match op.chars().next().unwrap() {
            '+' | '-' | '*' | '/' => return op.chars().next().unwrap(),
            _ => println!("Invalid operator! Use +, -, *, or /"),
        }
    }
}