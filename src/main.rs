mod day01_trebuchet;
mod day02_cube_conundrum;
mod day03_gear_ratios;
mod day04_scratchcards;
mod day05_fertilizer;
mod day06_wait_for_it;
mod day07_camel_cards;
mod day08_haunted_wasteland;
mod day09_mirage_maintenance;
mod day10_pipe_maze;
mod day11_cosmic_expansion;
mod day12_hot_springs;
mod day13_point_of_incidence;
mod day14_parabolic_dish;
mod day15_lens_library;
mod day16_lava_floor;

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
        ("day07_2", &day07_camel_cards::Day07_2 as &dyn Solution),
        (
            "day08_1",
            &day08_haunted_wasteland::Day08_1 as &dyn Solution,
        ),
        (
            "day08_2",
            &day08_haunted_wasteland::Day08_2 as &dyn Solution,
        ),
        (
            "day09_1",
            &day09_mirage_maintenance::Day09_1 as &dyn Solution,
        ),
        (
            "day09_2",
            &day09_mirage_maintenance::Day09_2 as &dyn Solution,
        ),
        ("day10_1", &day10_pipe_maze::Day10_1 as &dyn Solution),
        ("day10_2", &day10_pipe_maze::Day10_2 as &dyn Solution),
        ("day11_1", &day11_cosmic_expansion::Day11_1 as &dyn Solution),
        ("day11_2", &day11_cosmic_expansion::Day11_2 as &dyn Solution),
        ("day12_1", &day12_hot_springs::Day12_1 as &dyn Solution),
        ("day12_2", &day12_hot_springs::Day12_2 as &dyn Solution),
        (
            "day13_1",
            &day13_point_of_incidence::Day13_1 as &dyn Solution,
        ),
        (
            "day13_2",
            &day13_point_of_incidence::Day13_2 as &dyn Solution,
        ),
        ("day14_1", &day14_parabolic_dish::Day14_1 as &dyn Solution),
        ("day14_2", &day14_parabolic_dish::Day14_2 as &dyn Solution),
        ("day15_1", &day15_lens_library::Day15_1 as &dyn Solution),
        ("day15_2", &day15_lens_library::Day15_2 as &dyn Solution),
        ("day16_1", &day16_lava_floor::Day16_1 as &dyn Solution),
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
