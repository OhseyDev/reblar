mod cli;
mod md;

use std::env;
use std::vec;

fn main() {
    let mut args: vec::Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide arguments to the program");
        return;
    }
    let command = args[1].to_lowercase().chars().take(1).next().unwrap();
    args.remove(1);
    args.remove(0);
    let result = cli::process_command(command, &args);
    if result.is_ok() {
        println!("All good!");
    } else {
        eprintln!("ERROR: {}", result.err().unwrap());
    }
}
