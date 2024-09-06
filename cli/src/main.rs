use std::env;
use colored::Colorize;
use commom;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_args(&args);
    
    let result = commom::add(10, 10);
    println!("{}", result);
}

fn parse_args(args: &Vec<String>) {
    if args.len() < 2 {
        help_content();
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

// TODO: Make a better help mensage for the users
fn help_content() {
    println!("Help content comming soon!");
    println!("{} - Compile and link.\n{} - Run the bin.",
            "Build".yellow(),
            "Run".yellow())
}
