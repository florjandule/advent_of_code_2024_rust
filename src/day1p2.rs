use crate::input_filepath;
use crate::utils;

pub fn execute() -> u32 {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> u32 {
    let input = utils::read_input(input_filepath!(input_filename).as_str()).unwrap();

    let lines: Vec<&str> = input.lines().collect();

    // Reserve space for the vectors.
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    extract_from_lines(&mut left_list, &mut right_list, &lines);
    calculate_result(&left_list, &right_list)
}

fn extract_from_lines(left_list: &mut Vec<u32>, right_list: &mut Vec<u32>, lines: &Vec<&str>) {
    for line in lines.iter() {
        extract_from_line(left_list, right_list, line);
    }
}

fn extract_from_line(left_list: &mut Vec<u32>, right_list: &mut Vec<u32>, line: &str) {
    let (left, right) = utils::extract_two_numbers_from_line(line);
    left_list.push(left);
    right_list.push(right);
}

fn calculate_result(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    // Reserve space for the vectors.
    let mut result: Vec<u32> = Vec::new();
    for k in left_list.iter() {
        result.push(right_list.iter().filter(|&n| *n == *k).sum());
    }
    result.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_from_line() {
        let mut left_list: Vec<u32> = Vec::new();
        let mut right_list: Vec<u32> = Vec::new();
        let line = "1   2";
        extract_from_line(&mut left_list, &mut right_list, line);
        assert_eq!(left_list.len(), 1);
        assert_eq!(right_list.len(), 1);
        assert_eq!(
            left_list
                .get(0)
                .expect("Failed to get the first element from left_list"),
            &1
        );
        assert_eq!(
            right_list
                .get(0)
                .expect("Failed to get the first element from right_list"),
            &2
        );
    }

    #[test]
    fn test_calculate_result() {
        let left_list = vec![3, 4, 2, 1, 3, 3];
        let right_list = vec![4, 3, 5, 3, 9, 3];
        let result = calculate_result(&left_list, &right_list);
        assert_eq!(result, 31);
    }
}
