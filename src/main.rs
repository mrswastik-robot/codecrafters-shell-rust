#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {

    let builtin_commands = ["exit", "echo","type", "cd", "pwd", "export", "unset", "env", "source", "history"];

    let path_env = std::env::var("PATH").unwrap();

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
            ["echo", args @ ..] => println!("{}", args.join(" ")),
            ["type", args @ ..] => {
                let command = args.join(" ");
                if builtin_commands.contains(&command.as_str())
                {
                    println!("{} is a shell builtin", command);
                } else {
                    let mut found = false;
                    for path in path_env.split(":") {
                        let full_path = format!("{}/{}", path, command);
                        if std::path::Path::new(&full_path).exists() {
                            println!("{} is {}", command, full_path);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("{}: not found", command);
                    }
                }
            }

            _ => println!("{}: command not found", command),
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
