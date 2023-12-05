use std::collections::HashSet;

use crate::utils::Solution;

pub struct Day03_1;

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

fn get_adjacent_cells(m: usize, n: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let deltas = vec![
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

fn parse_num_at(m: usize, n: usize, grid: &Vec<Vec<char>>) -> u32 {
    let mut num = 0;
    for i in n..grid[0].len() {
        let c = grid[m][i];
        if !c.is_digit(10) {
            break;
        }
        num = num * 10 + c.to_digit(10).unwrap();
    }
    return num;
}

fn create_first_letter_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<Option<usize>>> {
    let m = grid.len();
    let n = grid[0].len();
    let mut matrix: Vec<Vec<Option<usize>>> = Vec::with_capacity(m);
    for i in 0..m {
        let mut row = Vec::with_capacity(n);
        let mut start_idx = None;
        for j in 0..n {
            // Tracking the first index of the string.
            if grid[i][j].is_digit(10) {
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
    return matrix;
}

fn solve_grid(grid: &Vec<Vec<char>>) -> u32 {
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
    return parts.iter().map(|(m, n)| parse_num_at(*m, *n, grid)).sum();
}

impl Solution for Day03_1 {
    fn name(&self) -> &str {
        "day03_gear_ratios"
    }

    fn solve(&self, lines: &Vec<String>) -> String {
        return solve_grid(
            &lines
                .iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect(),
        )
        .to_string();
    }
}
