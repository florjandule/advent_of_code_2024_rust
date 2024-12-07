use crate::utils;

type Level = Vec<u32>;

#[derive(PartialEq)]
enum LevelDirection {
    NotSet,
    Increasing,
    Decreasing,
}

pub fn execute() -> u32 {
    execute_with_input("input.txt")
}

fn execute_with_input(input_filename: &str) -> u32 {
    let input = utils::read_input(
        utils::get_input_path(&utils::file_stem(file!()).as_str(), input_filename).as_str(),
    )
    .unwrap();

    let lines: Vec<&str> = input.lines().collect();

    calculate_result(&lines)
}

fn calculate_result(lines: &Vec<&str>) -> u32 {
    let mut result: u32 = 0;
    lines.iter().for_each(|line| {
        let level = parse_line(line);
        if is_level_safe(&level) {
            result += 1;
        }
    });

    result
}

fn parse_line(line: &str) -> Level {
    line.split_whitespace()
        .map(|x| x.parse::<u32>().expect("Failed to parse number."))
        .collect()
}

fn is_level_safe(level: &Level) -> bool {
    let mut direction = LevelDirection::NotSet;
    for i in 1..level.len() {
        if (level[i] as i64 - level[i - 1] as i64).abs() > 3
            || (level[i] as i64 - level[i - 1] as i64).abs() < 1
        {
            return false;
        }
        if level[i] > level[i - 1] {
            if direction == LevelDirection::Decreasing {
                return false;
            }
            direction = LevelDirection::Increasing;
        } else if level[i] < level[i - 1] {
            if direction == LevelDirection::Increasing {
                return false;
            }
            direction = LevelDirection::Decreasing;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("7 6 4 2 1"), vec![7, 6, 4, 2, 1]);
        assert_eq!(parse_line("1 2 7 8 9"), vec![1, 2, 7, 8, 9]);
        assert_eq!(parse_line("9 7 6 2 1"), vec![9, 7, 6, 2, 1]);
        assert_eq!(parse_line("1 3 2 4 5"), vec![1, 3, 2, 4, 5]);
        assert_eq!(parse_line("8 6 4 4 1"), vec![8, 6, 4, 4, 1]);
        assert_eq!(parse_line("1 3 6 7 9"), vec![1, 3, 6, 7, 9]);
    }

    #[test]
    fn test_is_level_safe() {
        assert_eq!(is_level_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_level_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_level_safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_level_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_level_safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_level_safe(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_calculate_result() {
        let input = r"7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(calculate_result(&lines), 2);
    }
}
