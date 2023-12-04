mod day01_trebuchet;
mod utils;

use crate::utils::Solution;
use std::collections::HashMap;
use std::fs::read_to_string;

use crate::day01_trebuchet::Day01;

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn main() {
    let mut function_map = HashMap::new();
    function_map.insert("day01", &Day01 as &dyn Solution);

    let lines = read_lines("./src/day01_trebuchet/input.txt");
    println!("{}", function_map.get("day01").unwrap().solve(&lines));
}
