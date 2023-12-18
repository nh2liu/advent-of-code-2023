use crate::utils::{in_bounds, Solution};
use itertools::Itertools;
use std::collections::VecDeque;

pub struct Day17_1;
pub struct Day17_2;

const RIGHT: usize = 0;
const LEFT: usize = 1;
const DOWN: usize = 2;
const UP: usize = 3;

fn traverse(grid: &[Vec<u32>]) -> u32 {
    let mut min_loss = vec![vec![vec![vec![u32::MAX; 3]; 4]; grid[0].len()]; grid.len()];
    let mut q = VecDeque::from([((0, 1, RIGHT), 0, 0), ((1, 0, DOWN), 0, 0)]);
    while let Some(((i, j, d), c, v)) = q.pop_front() {
        if !in_bounds(grid, i, j) {
            continue;
        }
        let (right, left, down, up) = (
            (i, j + 1, RIGHT),
            (i, j - 1, LEFT),
            (i + 1, j, DOWN),
            (i - 1, j, UP),
        );

        let i = i as usize;
        let j = j as usize;
        let v = v + grid[i][j];
        if v >= min_loss[i][j][d][c] {
            continue;
        }
        for z in c..3 {
            min_loss[i][j][d][z] = std::cmp::min(min_loss[i][j][d][z], v);
        }

        let next = match d {
            RIGHT => [up, down],
            LEFT => [up, down],
            DOWN => [right, left],
            UP => [right, left],
            _ => panic!("Invalid direction"),
        };
        next.into_iter().for_each(|x| q.push_back((x, 0, v)));
        if c < 2 {
            q.push_back((
                match d {
                    RIGHT => right,
                    LEFT => left,
                    DOWN => down,
                    UP => up,
                    _ => panic!("Invalid direction"),
                },
                c + 1,
                v,
            ))
        }
    }
    *min_loss[min_loss.len() - 1][min_loss[0].len() - 1]
        .iter()
        .flatten()
        .min()
        .unwrap()
}

impl Solution for Day17_1 {
    fn name(&self) -> &str {
        "day17_clumsy_crucible"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines
            .iter()
            .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        traverse(&grid).to_string()
    }
}

fn traverse2(grid: &[Vec<u32>]) -> u32 {
    let mut min_loss = vec![vec![vec![vec![u32::MAX; 10]; 4]; grid[0].len()]; grid.len()];
    let mut q = VecDeque::from([((0, 1, RIGHT), 0, 0), ((1, 0, DOWN), 0, 0)]);
    while let Some(((i, j, d), c, v)) = q.pop_front() {
        if !in_bounds(grid, i, j) {
            continue;
        }
        let (right, left, down, up) = (
            (i, j + 1, RIGHT),
            (i, j - 1, LEFT),
            (i + 1, j, DOWN),
            (i - 1, j, UP),
        );

        let i = i as usize;
        let j = j as usize;
        let v = v + grid[i][j];
        if v >= min_loss[i][j][d][c] {
            continue;
        }
        min_loss[i][j][d][c] = v;

        if c >= 3 {
            let next = match d {
                RIGHT => [up, down],
                LEFT => [up, down],
                DOWN => [right, left],
                UP => [right, left],
                _ => panic!("Invalid direction"),
            };
            next.into_iter().for_each(|x| q.push_back((x, 0, v)));
        }
        if c < 9 {
            q.push_back((
                match d {
                    RIGHT => right,
                    LEFT => left,
                    DOWN => down,
                    UP => up,
                    _ => panic!("Invalid direction"),
                },
                c + 1,
                v,
            ))
        }
    }
    *min_loss[min_loss.len() - 1][min_loss[0].len() - 1]
        .iter()
        .flat_map(|x| x[3..].iter().collect_vec())
        .min()
        .unwrap()
}

impl Solution for Day17_2 {
    fn name(&self) -> &str {
        "day17_clumsy_crucible"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines
            .iter()
            .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();
        traverse2(&grid).to_string()
    }
}
