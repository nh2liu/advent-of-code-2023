use std::collections::HashSet;

use crate::utils::Solution;

pub struct Day03_1;
pub struct Day03_2;

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn get_adjacent_cells(m: usize, n: usize, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    return deltas
        .iter()
        .map(|(dy, dx)| (m as i32 + dy, n as i32 + dx))
        .filter(|(y, x)| -> bool {
            grid.len() as i32 > *y && *y >= 0 && grid[0].len() as i32 > *x && *x >= 0
        })
        .map(|(y, x)| (y as usize, x as usize))
        .collect();
}

fn parse_num_at(m: usize, n: usize, grid: &[Vec<char>]) -> u32 {
    let mut num = 0;
    for i in n..grid[0].len() {
        let c = grid[m][i];
        if !c.is_ascii_digit() {
            break;
        }
        num = num * 10 + c.to_digit(10).unwrap();
    }
    num
}

fn create_first_letter_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<Option<usize>>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut matrix: Vec<Vec<Option<usize>>> = Vec::with_capacity(m);
    for grid_row in grid.iter().take(m) {
        let mut row = Vec::with_capacity(n);
        let mut start_idx = None;
        for (j, c) in grid_row.iter().take(n).enumerate() {
            // Tracking the first index of the string.
            if c.is_ascii_digit() {
                if start_idx.is_none() {
                    start_idx = Some(j);
                }
                row.push(start_idx);
            } else {
                start_idx = None;
                row.push(None);
            }
        }
        matrix.push(row);
    }
    matrix
}

impl Solution for Day03_1 {
    fn name(&self) -> &str {
        "day03_gear_ratios"
    }

    fn solve(&self, lines: &[String]) -> String {
        let grid = &lines
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let fl_grid = create_first_letter_grid(grid);

        let mut parts = HashSet::new();
        for (m, row) in grid.iter().enumerate() {
            for (n, c) in row.iter().enumerate() {
                if is_symbol(*c) {
                    for (i, j) in get_adjacent_cells(m, n, grid) {
                        if fl_grid[i][j].is_some() {
                            parts.insert((i, fl_grid[i][j].unwrap()));
                        }
                    }
                }
            }
        }
        return parts
            .iter()
            .map(|(m, n)| parse_num_at(*m, *n, grid))
            .sum::<u32>()
            .to_string();
    }
}

impl Solution for Day03_2 {
    fn name(&self) -> &str {
        "day03_gear_ratios"
    }

    fn solve(&self, lines: &[String]) -> String {
        let grid = &lines
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        let fl_grid = create_first_letter_grid(grid);
        let mut running_sum = 0;
        for (m, row) in grid.iter().enumerate() {
            for (n, c) in row.iter().enumerate() {
                if *c == '*' {
                    let mut parts: HashSet<(usize, usize)> = HashSet::new();

                    for (i, j) in get_adjacent_cells(m, n, grid) {
                        if fl_grid[i][j].is_some() {
                            parts.insert((i, fl_grid[i][j].unwrap()));
                        }
                    }
                    if parts.len() == 2 {
                        let gear_ratio: i32 = parts
                            .iter()
                            .cloned()
                            .map(|(m, n)| parse_num_at(m, n, grid) as i32)
                            .product();
                        running_sum += gear_ratio;
                    }
                }
            }
        }
        running_sum.to_string()
    }
}
