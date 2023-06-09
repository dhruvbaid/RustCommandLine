mod cli;
use std::io::{self, Write};

fn main() {
    loop {
        // Command Line prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Receive user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Split input by ' ' character, collect() result into a vector
        let input_vec: Vec<&str> = input.trim().split(' ').collect();

        // Handle different cases for this input by calling functions defined
        // in cli.rs
        match input_vec[0] {
            "" => {},
            "cat" => {
                cli::cat(input_vec[1..].to_vec());
            },
            "echo" => {
                println!("{}", &cli::echo(input_vec[1..].to_vec()));
            },
            "ls" => {
                println!("{}", &cli::ls());
            }
            "quit" => {break;},
            _ => {break;},
        }
    }
}
