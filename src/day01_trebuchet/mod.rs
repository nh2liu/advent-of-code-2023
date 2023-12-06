use std::collections::HashMap;

//mod utils;
use crate::utils::Solution;

pub struct Day01_1;
pub struct Day01_2;

fn solve_line(line: &str) -> u32 {
    let mut first_char = Option::None;
    let mut last_char = Option::None;

    for c in line.chars() {
        if c.is_ascii_digit() {
            if first_char.is_none() {
                first_char = Some(c);
            }
            last_char = Some(c);
        }
    }
    let first_digit = first_char.unwrap().to_digit(10).unwrap();
    let last_digit = last_char.unwrap().to_digit(10).unwrap();
    first_digit * 10 + last_digit
}

impl Solution for Day01_1 {
    fn name(&self) -> &str {
        "day01_trebuchet"
    }

    fn solve(&self, lines: &[String]) -> String {
        return lines
            .iter()
            .map(|s| solve_line(s.as_str()))
            .sum::<u32>()
            .to_string();
    }
}

fn solve_line2(line: &str) -> u32 {
    let value_map: HashMap<&str, u32> = vec![
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();
    let (first_digit, _) = value_map
        .iter()
        .map(|(s, v)| (v, line.find(s)))
        .filter(|(_, index)| index.is_some())
        .min_by_key(|(_, index)| index.unwrap())
        .unwrap();
    let (last_digit, _) = value_map
        .iter()
        .map(|(s, v)| (v, line.rfind(s)))
        .filter(|(_, index)| index.is_some())
        .max_by_key(|(_, index)| index.unwrap())
        .unwrap();
    first_digit * 10 + last_digit
}

impl Solution for Day01_2 {
    fn name(&self) -> &str {
        "day01_trebuchet"
    }

    fn solve(&self, lines: &[String]) -> String {
        return lines
            .iter()
            .map(|s| solve_line2(s.as_str()))
            .sum::<u32>()
            .to_string();
    }
}
