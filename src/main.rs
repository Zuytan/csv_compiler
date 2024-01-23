use std::io::stdin;

use input_file_reader::InputFileReader;

use crate::formatter::{raw_formatter::RawFormatter, Formatter};

pub mod error;
pub mod event_data;
pub mod formatter;
pub mod input_file;
pub mod input_file_reader;

fn main() {
    println!("Hello, I am Jean-Roger and I will compile the CSV files you put in the input_files directory.");
    println!(
        "Please select the compilation you want to use (you can put multiple choice if you want) :"
    );
    println!("1. Raw compilation (just compile all files in one)");
    println!("Any other touch will cancel the compilation");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Did not enter a correct string. Bye bye.");
    if let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }
    if let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }
    let mut formatters = Vec::new();
    if !buf.contains("1") {
        return println!("No valid input found, bye bye.");
    }
    println!("Your input seems valid, wait a few seconds...");
    let reader = match InputFileReader::try_from("./input_files".to_string()) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e.reason());
            return;
        }
    };
    if buf.contains("1") {
        formatters.push(Box::new(RawFormatter::new(reader.files())));
    }

    for formatter in formatters {
        formatter.compile();
    }
    println!("Compilation is done, another excellent job made by Jean-Roger B-)");
}
