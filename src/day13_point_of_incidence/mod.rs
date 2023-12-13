use std::cmp::min;

use crate::utils::Solution;
use itertools::Itertools;
pub struct Day13_1;
pub struct Day13_2;

fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            ret[j][i] = c;
        }
    }
    ret
}

fn find_mirror_row(grid: &[Vec<char>]) -> Option<usize> {
    let n_rows = grid.len();
    for j in 1..n_rows {
        let r = min(j, n_rows - j);
        let is_reflection = grid[j - r..j]
            .iter()
            .zip(grid[j..j + r].iter().rev())
            .all(|(row1, row2)| row1 == row2);
        if is_reflection {
            return Some(j);
        }
    }
    None
}

fn find_mirror_row_smudged(grid: &[Vec<char>]) -> Option<usize> {
    let n_rows = grid.len();
    for j in 1..n_rows {
        let r = min(j, n_rows - j);
        let diffs = grid[j - r..j]
            .iter()
            .zip(grid[j..j + r].iter().rev())
            .map(|(row1, row2)| row1.iter().zip(row2).filter(|(&a, &b)| a != b).count())
            .sum::<usize>();
        if diffs == 1 {
            return Some(j);
        }
    }
    None
}

fn parse(lines: &[String]) -> Vec<Vec<Vec<char>>> {
    lines
        .join("\n")
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

impl Solution for Day13_1 {
    fn name(&self) -> &str {
        "day13_point_of_incidence"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grids = parse(lines);
        let mut ans = 0;
        for (i, grid) in grids.iter().enumerate() {
            let r_horizontal = find_mirror_row(grid).unwrap_or(0);
            let r_vertical = find_mirror_row(&transpose(grid)).unwrap_or(0);
            println!("{i} | {r_vertical} {r_horizontal}");
            ans += r_vertical + r_horizontal * 100;
        }
        ans.to_string()
    }
}

impl Solution for Day13_2 {
    fn name(&self) -> &str {
        "day13_point_of_incidence"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grids = parse(lines);
        let mut ans = 0;
        for (i, grid) in grids.iter().enumerate() {
            let r_horizontal = find_mirror_row_smudged(grid).unwrap_or(0);
            let r_vertical = find_mirror_row_smudged(&transpose(grid)).unwrap_or(0);
            println!("{i} | {r_vertical} {r_horizontal}");
            ans += r_vertical + r_horizontal * 100;
        }
        ans.to_string()
    }
}
