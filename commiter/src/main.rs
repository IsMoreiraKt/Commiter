fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap().expect("Failed to read input");
    input.trim().to_string()
}
