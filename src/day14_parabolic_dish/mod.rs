use std::collections::HashMap;

use crate::utils::Solution;
use itertools::Itertools;
pub struct Day14_1;
pub struct Day14_2;

fn parse(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect_vec()).collect_vec()
}

fn tilt_north(grids: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut d = vec![vec![0isize; grids[0].len()]; grids.len()];
    let mut ret = vec![vec!['.'; grids[0].len()]; grids.len()];
    for (j, &c) in grids[0].iter().enumerate() {
        d[0][j] = match c {
            'O' => 0,
            '.' => -1,
            '#' => 0,
            _ => panic!("invalid char"),
        };
        ret[0][j] = grids[0][j];
    }

    for (i, row) in (1..).zip(grids[1..].iter()) {
        for (j, &c) in row.iter().enumerate() {
            d[i][j] = match c {
                'O' => {
                    ret[(d[i - 1][j] + 1) as usize][j] = 'O';
                    d[i - 1][j] + 1
                }
                '.' => d[i - 1][j],
                '#' => {
                    ret[i][j] = '#';
                    i as isize
                }
                _ => panic!("invalid char"),
            }
        }
    }
    ret
}

fn rotate_90_cw(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            ret[x][grid.len() - y - 1] = c;
        }
    }
    ret
}

fn cycle(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut grid1 = grid.to_owned();
    for _ in 0..4 {
        grid1 = tilt_north(&grid1);
        // print_grid(&grid);
        // println!("");
        grid1 = rotate_90_cw(&grid1);
    }
    grid1
}

fn score(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&x| x == 'O').count() * (grid.len() - i))
        .sum::<usize>()
}

fn print_grid(grid: &[Vec<char>]) {
    for v in grid.iter() {
        println!("{}", v.iter().collect::<String>());
    }
}

impl Solution for Day14_1 {
    fn name(&self) -> &str {
        "day14_parabolic_dish"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = parse(lines);
        let tilted_grid = tilt_north(&grid);
        print_grid(&tilted_grid);
        score(&grid).to_string()
    }
}

impl Solution for Day14_2 {
    fn name(&self) -> &str {
        "day14_parabolic_dish"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut grid = parse(lines);
        let mut h = HashMap::new();
        for i in 0..1_000_000_000 {
            grid = cycle(&grid);
            let hashable: String = grid.iter().flatten().collect::<String>();
            if let std::collections::hash_map::Entry::Vacant(e) = h.entry(hashable.clone()) {
                e.insert(i);
            } else {
                let first_instance = h.get(&hashable).unwrap();
                let cycle_len = i - first_instance;
                let more_iterations: i32 = (1_000_000_000 - first_instance - 1) % cycle_len;

                println!("{first_instance} -> {i} | {cycle_len} | {more_iterations}");
                for _ in 0..more_iterations {
                    grid = cycle(&grid);
                }
                return score(&grid).to_string();
            }
        }
        panic!("Solution not found.")
    }
}
