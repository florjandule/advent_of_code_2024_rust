use crate::utils;
use regex;

type ResultValue = u32;

pub fn execute() -> ResultValue {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> ResultValue {
    let input = utils::read_input(
        utils::get_input_path(&utils::file_stem(file!()).as_str(), input_filename).as_str(),
    )
    .expect("Failed to read input");

    calculate_result(&input.as_str())
}

fn calculate_result(input: &str) -> ResultValue {
    let reg = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)")
        .expect("Failed to create regex");

    let mut result: ResultValue = 0;
    let mut enabled: bool = true;
    for cap in reg.captures_iter(input) {
        if cap[0].contains("do()") {
            enabled = true;
            continue;
        } else if cap[0].eq("don't()") {
            enabled = false;
            continue;
        }
        if !enabled {
            continue;
        }

        let a: ResultValue = cap[1].parse().expect("Failed to parse number");
        let b: ResultValue = cap[2].parse().expect("Failed to parse number");
        result += a * b;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_result() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(calculate_result(input), 48);
    }
}
