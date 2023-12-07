mod day01_trebuchet;
mod day02_cube_conundrum;
mod day03_gear_ratios;
mod day04_scratchcards;
mod day05_fertilizer;
mod day06_wait_for_it;
mod day07_camel_cards;

mod utils;

use crate::utils::Solution;
use inquire::Select;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn main() {
    let solutions_map: HashMap<&str, &dyn Solution> = [
        ("day01_1", &day01_trebuchet::Day01_1 as &dyn Solution),
        ("day01_2", &day01_trebuchet::Day01_2 as &dyn Solution),
        ("day02_1", &day02_cube_conundrum::Day02_1 as &dyn Solution),
        ("day02_2", &day02_cube_conundrum::Day02_2 as &dyn Solution),
        ("day03_1", &day03_gear_ratios::Day03_1 as &dyn Solution),
        ("day03_2", &day03_gear_ratios::Day03_2 as &dyn Solution),
        ("day04_1", &day04_scratchcards::Day04_1 as &dyn Solution),
        ("day04_2", &day04_scratchcards::Day04_2 as &dyn Solution),
        ("day05_1", &day05_fertilizer::Day05_1 as &dyn Solution),
        ("day05_2", &day05_fertilizer::Day05_2 as &dyn Solution),
        ("day06_1", &day06_wait_for_it::Day06_1 as &dyn Solution),
        ("day06_2", &day06_wait_for_it::Day06_2 as &dyn Solution),
        ("day07_1", &day07_camel_cards::Day07_1 as &dyn Solution),
    ]
    .iter()
    .cloned()
    .collect();
    let mut select_box = Select::new(
        "Select the problem:",
        solutions_map.keys().sorted().collect(),
    );
    select_box.starting_cursor = solutions_map.len() - 1;
    let problem = select_box.prompt();
    let solution = solutions_map.get(problem.unwrap()).unwrap();
    let lines = read_lines(format!("./src/{}/input.txt", solution.name()).as_str());
    println!("{}", solution.solve(&lines));
}
