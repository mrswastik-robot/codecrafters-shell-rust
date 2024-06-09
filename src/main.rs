#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};

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

            //handling the cd and pwd commands

            ["cd"] | ["cd" , "~"] => {
                if let Some(home_dir) = std::env::var("HOME").ok(){
                    if let Err(e) = std::env::set_current_dir(&home_dir){
                        println!("cd: {}: {}", home_dir, e);
                    }
                } else {
                    println!("cd: HOME enivornment variable not set.")
                }
            }

            ["cd", dir] => {                                    //it's the standard case when there is some directory after the cd command either valid or invalid
                if let Err(_) = std::env::set_current_dir(dir){
                    println!("{}: No such file or directory", dir);
                }
            }

            ["pwd"] => {
                if let Ok(path) = std::env::current_dir(){
                    println!("{}", path.display());
                }
            }

            // _ => println!("{}: command not found", command),  earlier it was like this but now we have to have the functionality to execute the command as an external command if it is not a built-in command 
            _ => {
                let command = tokens[0];
                let args = &tokens[1..];
                match Command::new(command).args(args).output() {
                    Ok(output) => {
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    }
                    Err(_) => {
                        println!("{}: command not found", command);
                    }
                }
            }

        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
