use std::collections::HashMap;

//mod utils;
use crate::utils::Solution;
use std::cmp::max;
pub struct Day02_1;
pub struct Day02_2;

fn parse_draw(draw: &str) -> HashMap<&str, u32> {
    // 1 red, 2 green, 6 blue -> {"red": 1, "green": 2, "blue": 6}
    return draw
        .trim()
        .split(", ")
        .map(|cstr| cstr.split_once(' ').unwrap())
        .map(|(n_str, c_str)| {
            (
                c_str,
                n_str.parse::<u32>().unwrap_or_else(|_| {
                    println!("{} | {} | {}", draw, n_str, c_str);
                    panic!("lol");
                }),
            )
        })
        .collect();
}

fn check_game(line: &str, condition: &HashMap<&str, u32>) -> bool {
    let (_, line) = line.split_once(':').unwrap();
    line.split(';').map(parse_draw).all(|marbles| {
        marbles
            .iter()
            .all(|(key, val)| condition.contains_key(key) && condition.get(key).unwrap() >= val)
    })
}

fn fewest_cubes(line: &str) -> u32 {
    let (_, line) = line.split_once(':').unwrap();
    let mut min_cubes: HashMap<&str, u32> = HashMap::new();
    for draw in line.split(';').map(parse_draw) {
        for (color, count) in draw.iter() {
            min_cubes.insert(
                color,
                max(min_cubes.get(color).unwrap_or(&0), count).to_owned(),
            );
        }
    }
    return min_cubes.values().product();
}

impl Solution for Day02_1 {
    fn name(&self) -> &str {
        "day02_cube_conundrum"
    }
    fn solve(&self, lines: &[String]) -> String {
        let condition: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect();
        return lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                if check_game(line, &condition) {
                    i + 1
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string();
    }
}

impl Solution for Day02_2 {
    fn name(&self) -> &str {
        "day02_cube_conundrum"
    }
    fn solve(&self, lines: &[String]) -> String {
        return lines
            .iter()
            .map(|s| fewest_cubes(s.as_str()))
            .sum::<u32>()
            .to_string();
    }
}
