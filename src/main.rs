mod day01_trebuchet;
mod day02_cube_conundrum;

mod utils;

use crate::utils::Solution;
use std::collections::HashMap;
use std::fs::read_to_string;

use crate::day01_trebuchet::Day01;
use crate::day02_cube_conundrum::Day02;

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn main() {
    let solutions_map: HashMap<&str, &dyn Solution> = [
        ("day01", &Day01 as &dyn Solution),
        ("day02", &Day02 as &dyn Solution),
    ]
    .iter()
    .cloned()
    .collect();
    let solution = solutions_map.get("day02").unwrap();
    let lines = read_lines(format!("./src/{}/input.txt", solution.name()).as_str());
    println!("{}", solution.solve(&lines));
}
