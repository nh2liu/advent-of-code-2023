//mod utils;
use crate::utils::Solution;

pub struct Day01;

fn solve_line(line: &String) -> u32 {
    let mut first_char = Option::None;
    let mut last_char = Option::None;

    for c in line.chars() {
        if c.is_digit(10) {
            if first_char.is_none() {
                first_char = Some(c);
            }
            last_char = Some(c);
        }
    }
    let first_digit = first_char.unwrap().to_digit(10).unwrap();
    let last_digit = last_char.unwrap().to_digit(10).unwrap();
    return first_digit * 10 + last_digit;
}

impl Solution for Day01 {
    fn solve(&self, lines: &Vec<String>) -> String {
        return lines.iter().map(solve_line).sum::<u32>().to_string();
    }
}
