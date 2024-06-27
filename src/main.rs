#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{self, Command};

// Color constants inspired by Catppuccin Mocha theme
const RESET: &str = "\x1b[0m";
const BG_COLOR: &str = "\x1b[48;2;30;30;46m";  // Crust (dark background)
const FG_COLOR: &str = "\x1b[38;2;205;214;244m";  // Text
const PROMPT_COLOR: &str = "\x1b[38;2;243;139;168m";  // Red for prompt
const OUTPUT_COLOR: &str = "\x1b[38;2;137;180;250m";  // Blue for command output
const ERROR_COLOR: &str = "\x1b[38;2;235;160;172m";  // Peach for errors

const TRASH_ASCII: &str = r#"
 _____  ____    __    ____  _   _ 
|_   _||  _ \  / _\  / ___|| | | |
  | |  | |_) |/ _ \  \___ \| |_| |
  | |  |  _ </ ___ \  ___) |  _  |
  |_|  |_| \_\_/  \_\|____/|_| |_|
"#;

const TRASH_ICON: &str = "[ゴミ]";

fn print_styled(text: &str, style: &str) {
    print!("{}{}{}{}", BG_COLOR, style, text, RESET);
    io::stdout().flush().unwrap();
}

fn println_styled(text: &str, style: &str) {
    println!("{}{}{}{}", BG_COLOR, style, text, RESET);
}

fn main() {


    // Set the background color for the entire terminal
    print!("{}", BG_COLOR);
    io::stdout().flush().unwrap();

    // Display ASCII art for "Trash" when the shell starts
    println_styled(TRASH_ASCII, PROMPT_COLOR);
    println_styled("Welcome to Trash Shell!", FG_COLOR);

    let builtin_commands = ["exit", "echo", "type", "cd", "pwd", "export", "unset", "env", "source", "history"];

    let path_env = std::env::var("PATH").unwrap();

    loop {
        // Print the prompt with a trash icon
        print_styled(&format!("{} -> ", TRASH_ICON), PROMPT_COLOR);

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
                    println_styled(&format!("Invalid exit code: {}", code), ERROR_COLOR);
                }
            }
            ["exit"] => process::exit(0),
            ["echo", args @ ..] => println_styled(&args.join(" "), OUTPUT_COLOR),
            ["type", args @ ..] => {
                let command = args.join(" ");
                if builtin_commands.contains(&command.as_str()) {
                    println_styled(&format!("{} is a shell builtin", command), OUTPUT_COLOR);
                } else {
                    let mut found = false;
                    for path in path_env.split(":") {
                        let full_path = format!("{}/{}", path, command);
                        if std::path::Path::new(&full_path).exists() {
                            println_styled(&format!("{} is {}", command, full_path), OUTPUT_COLOR);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println_styled(&format!("{}: not found", command), ERROR_COLOR);
                    }
                }
            }
            ["cd"] | ["cd", "~"] => {
                if let Some(home_dir) = std::env::var("HOME").ok() {
                    if let Err(e) = std::env::set_current_dir(&home_dir) {
                        println_styled(&format!("cd: {}: {}", home_dir, e), ERROR_COLOR);
                    }
                } else {
                    println_styled("cd: HOME environment variable not set.", ERROR_COLOR);
                }
            }
            ["cd", dir] => {
                if let Err(_) = std::env::set_current_dir(dir) {
                    println_styled(&format!("{}: No such file or directory", dir), ERROR_COLOR);
                }
            }
            ["pwd"] => {
                if let Ok(path) = std::env::current_dir() {
                    println_styled(&path.display().to_string(), OUTPUT_COLOR);
                }
            }
            _ => {
                let command = tokens[0];
                let args = &tokens[1..];
                match Command::new(command).args(args).output() {
                    Ok(output) => {
                        print_styled(&String::from_utf8_lossy(&output.stdout), OUTPUT_COLOR);
                        print_styled(&String::from_utf8_lossy(&output.stderr), ERROR_COLOR);
                    }
                    Err(_) => {
                        println_styled(&format!("{}: command not found", command), ERROR_COLOR);
                    }
                }
            }
        }
    }
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}