use std::fs::OpenOptions;
use std::io::{self, Read};

pub fn read_input(file_path: &str) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn file_stem(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .expect("Failed to get the filename without extension")
        .to_str()
        .expect("Failed to convert the filename to a string")
        .to_string()
}

pub fn get_input_path(challenge_name: &str, input_filename: &str) -> String {
    std::env::current_dir()
        .expect("Failed to get the current directory")
        .join("inputs")
        .join(challenge_name)
        .join(input_filename)
        .display()
        .to_string()
}

pub fn extract_two_numbers_from_line(input: &str) -> (u32, u32) {
    let split: Vec<&str> = input.split("   ").collect();
    let first = split[0]
        .parse::<u32>()
        .expect("Failed to parse the first number");
    let second = split[1]
        .parse::<u32>()
        .expect("Failed to parse the second number");
    (first, second)
}
