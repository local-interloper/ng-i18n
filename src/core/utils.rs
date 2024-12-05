use std::error::Error;
use colored::Colorize;

pub fn print_error(error: &Box<dyn Error>) {
    println!("{}", "Execution failed!".red());
    println!("{} {}", "ERROR:".red(), error);
}