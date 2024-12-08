use crate::utils;
use regex;


pub fn execute() -> u32 {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> u32 {
    let input = utils::read_input(
        utils::get_input_path(&utils::file_stem(file!()).as_str(), input_filename).as_str(),
    )
    .expect("Failed to read input");

    calculate_result(&input.as_str())
}

fn calculate_result(input: &str) -> u32 {
    let reg = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Failed to create regex from input");
    let mut result = 0;
    for cap in reg.captures_iter(input){
        let a: i32 = cap[1].parse().expect("Failed to parse number");
        let b: i32 = cap[2].parse().to_owned().expect("Failed to parse number");
        result += a * b;
    }
    result as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_result(){
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(calculate_result(input), 161);
    }
}