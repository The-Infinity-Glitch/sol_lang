use core;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
}

fn parse_args(args: &Vec<String>) {
    if args.len() < 2 {
        core::show_help_content("cli", env!("CARGO_PKG_DESCRIPTION"));
    } else {
        for arg in args {
            if arg.chars().nth(0).unwrap() == '-' {
                println!("{}: argument.", arg);
            } else {
                println!("{}: source file.", arg);
            }
        }
    }
}
