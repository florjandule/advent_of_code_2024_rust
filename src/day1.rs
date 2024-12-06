use crate::utils;

pub fn execute() -> u32 {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> u32 {
    let input = utils::read_input(
        get_input_path(
            utils::get_filename_without_extension(file!()).as_str(),
            input_filename,
        )
        .as_str(),
    )
    .unwrap();

    let lines: Vec<&str> = input.lines().collect();

    // Split the input into two vectors.
    // Each line is split into two numbers.
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    let len = lines.len();
    left_numbers.reserve_exact(len);
    right_numbers.reserve_exact(len);

    for line in lines.iter() {
        let (left, right) = split_input(line);
        left_numbers.push(left);
        right_numbers.push(right);
    }

    // Sort the vectors in ascending order.
    left_numbers.sort();
    right_numbers.sort();

    // Calculate the distance between the two vectors.
    let distance = calculate_distance(&left_numbers, &right_numbers);
    println!("Result: {}", distance);
    distance
}

fn get_input_path(challenge_name: &str, input_filename: &str) -> String {
    std::env::current_dir()
        .expect("Failed to get the current directory")
        .join("inputs")
        .join(challenge_name)
        .join(input_filename)
        .display()
        .to_string()
}

fn split_input(input: &str) -> (u32, u32) {
    let split: Vec<&str> = input.split("   ").collect();
    let first = split[0]
        .parse::<u32>()
        .expect("Failed to parse the first number");
    let second = split[1]
        .parse::<u32>()
        .expect("Failed to parse the second number");
    (first, second)
}

fn calculate_distance(left_numbers: &Vec<u32>, right_numbers: &Vec<u32>) -> u32 {
    let mut distance = 0;
    for i in 0..left_numbers.len() {
        distance += (left_numbers[i] as i64 - right_numbers[i] as i64).abs() as u32;
    }
    distance
}
