#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();
        let tokens = tokenize(command);

        match tokens.as_slice() {
            ["exit", code] => {
                if let Ok(exit_code) = code.parse::<i32>() {
                    process::exit(exit_code);
                } else {
                    println!("Invalid exit code: {}", code);
                }
            }
            ["exit"] => process::exit(0),
            
            _ => println!("{}: command not found", command),
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
