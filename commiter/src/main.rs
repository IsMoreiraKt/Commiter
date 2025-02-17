use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use chrono::{NaiveDate, Duration};



const NUM_THREADS: usize = 4;


fn main() {
    let start_date = read_date_input("Enter start date (YYYY-MM-DD): ");
    let end_date = read_date_input("Enter end date (YYYY-MM-DD): ");
    let commits_per_day: usize = read_number_input("Enter number of commits per day: ");
    let repo = read_input("Enter repository path: ");

    assert!(start_date < end_date, "Start date must be before end date");
    assert!(commits_per_day > 0, "Commits per day must be greater than 0");
    assert!(!repo.is_empty(), "Repository path must not be empty");
    assert!(Path::new(&repo).exists(), "Repository path does not exist");

    let repo = Arc::new(repo);
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = Vec::with_capacity(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        let repo_clone = Arc::clone(&repo);
        let rx_clone = Arc::clone(&rx);

        let handle = thread::spawn(move || {
            let rx = rx_clone.lock().expect("Failed to lock receiver");

            while let Ok(current_date_str) = rx.recv() {
                for i in 1..=commits_per_day {
                    let commit_message = format!("Commit {} on {}", i, current_date_str);
                    let output = Command::new("git")
                        .arg("commit")
                        .arg("--allow-empty")
                        .arg("--date")
                        .arg(&current_date_str)
                        .arg("-m")
                        .arg(&commit_message)
                        .current_dir(&**repo_clone)
                        .output()
                        .expect("Failed to execute git command");

                    if !output.status.success() {
                        eprintln!("Failed to commit: {}", String::from_utf8_lossy(&output.stderr));
                        return;
                    } else {
                        println!("Commit completed: {}", commit_message);
                    }
                }
            }
        });

        handles.push(handle);
    }

    let mut current_date = start_date;

    while current_date <= end_date {
        let current_date_str = current_date.format("%Y-%m-%d").to_string();
        tx.send(current_date_str).expect("Failed to send task to thread");
        current_date += Duration::days(1);
    }

    drop(tx);

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}


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
