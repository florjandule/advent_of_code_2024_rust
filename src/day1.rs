use crate::utils;

pub fn execute() -> u32 {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> u32 {
    let input = utils::read_input(
        utils::get_input_path(&utils::file_stem(file!()).as_str(), input_filename).as_str(),
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
        let (left, right) = utils::extract_two_numbers_from_line(line);
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

fn calculate_distance(left_numbers: &Vec<u32>, right_numbers: &Vec<u32>) -> u32 {
    let mut distance = 0;
    for i in 0..left_numbers.len() {
        distance += (left_numbers[i] as i64 - right_numbers[i] as i64).abs() as u32;
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_input() {
        let input = "1   2";
        let (left, right) = utils::extract_two_numbers_from_line(input);
        assert_eq!(left, 1);
        assert_eq!(right, 2);
    }

    #[test]
    fn test_calculate_distance() {
        let left_numbers = vec![1, 2, 3];
        let right_numbers = vec![4, 5, 6];
        let distance = calculate_distance(&left_numbers, &right_numbers);
        assert_eq!(distance, 9);
    }

    #[test]
    fn test_execute_with_input() {
        let result = execute_with_input("test_input.txt");
        assert_eq!(result, 11);
    }
}
