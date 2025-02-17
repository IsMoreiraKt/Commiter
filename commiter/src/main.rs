use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use chrono::{NaiveDate, Duration};




fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


fn read_date_input(prompt: &str) -> NaiveDate {
    loop {
        let input = read_input(prompt);
        match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            Ok(date) => return date,
            Err(_) => println!("Invalid format! Use YYYY-MM-DD."),
        }
    }
}


fn read_number_input(prompt: &str) -> usize {
    loop {
        let input = read_input(prompt);
        match input.trim().parse::<usize>() {
            Ok(num) if num > 0 => return num,
            _ => println!("Invalid number! Enter a value greater than zero."),
        }
    }
}
