use crate::utils::Solution;
use itertools::Itertools;
use std::collections::HashSet;

use std::ops::Div;

pub struct Day10_1;
pub struct Day10_2;

fn find_s(grid: &[Vec<char>]) -> ((usize, usize), char) {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                if i > 0 && ['|', 'F', '7'].contains(&grid[i - 1][j]) {
                    return ((i - 1, j), 'u');
                } else if i < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[i + 1][j]) {
                    return ((i + 1, j), 'd');
                } else if j > 0 && ['-', 'F', 'L'].contains(&grid[i][j - 1]) {
                    return ((i, j - 1), 'l');
                } else if j < grid[0].len() && ['-', 'J', '7'].contains(&grid[i][j + 1]) {
                    return ((i, j + 1), 'r');
                }
                panic!("Invalid start");
            }
        }
    }
    panic!("S not found.")
}

fn find_pipes(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let ((mut i, mut j), mut dir) = find_s(grid);
    let mut pipe_seq = Vec::new();
    loop {
        let c = grid[i][j];
        pipe_seq.push((i, j));
        dir = match (c, dir) {
            ('S', _) => 's',
            ('-', dir) => dir,
            ('|', dir) => dir,
            ('L', 'd') => 'r',
            ('L', 'l') => 'u',
            ('J', 'd') => 'l',
            ('J', 'r') => 'u',
            ('7', 'r') => 'd',
            ('7', 'u') => 'l',
            ('F', 'u') => 'r',
            ('F', 'l') => 'd',
            e => panic!("Invalid combo at {:?}", e),
        };
        if dir == 's' {
            return pipe_seq;
        }
        (i, j) = match dir {
            'd' => (i + 1, j),
            'u' => (i - 1, j),
            'l' => (i, j - 1),
            'r' => (i, j + 1),
            _ => panic!("lol"),
        }
    }
}

fn find_area(grid: &[Vec<char>]) -> u32 {
    // Note this doesn't work if the top left square is the S coordinate.
    let pipe_coords = find_pipes(grid);
    let (i, j) = *pipe_coords
        .iter()
        .filter(|(i, j)| grid[*i][*j] == 'F')
        .min()
        .unwrap();
    println!("F coord: {} {}", i, j);

    // By flood fill.
    let mut area = 0;
    let mut stack = vec![((i, j), 'u')];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some(((i, j), d)) = stack.pop() {
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));
        if pipe_coords.contains(&(i, j)) {
            match (grid[i][j], d) {
                ('S', _) => {} // NOOP
                ('-', 'u') => {
                    stack.push(((i, j + 1), 'u'));
                    stack.push(((i, j - 1), 'u'));
                    stack.push(((i + 1, j), 'd'));
                }
                ('-', 'd') => {
                    stack.push(((i, j + 1), 'd'));
                    stack.push(((i, j - 1), 'd'));
                    stack.push(((i - 1, j), 'u'));
                }
                ('|', 'r') => {
                    stack.push(((i + 1, j), 'r'));
                    stack.push(((i - 1, j), 'r'));
                    stack.push(((i, j - 1), 'l'));
                }
                ('|', 'l') => {
                    stack.push(((i + 1, j), 'l'));
                    stack.push(((i - 1, j), 'l'));
                    stack.push(((i, j + 1), 'r'));
                }
                ('L', 'u') | ('L', 'r') => {
                    stack.push(((i - 1, j), 'r'));
                    stack.push(((i, j + 1), 'u'));
                    stack.push(((i, j - 1), 'l'));
                    stack.push(((i + 1, j), 'd'));
                }
                ('L', 'd') | ('L', 'l') => {
                    stack.push(((i - 1, j), 'l'));
                    stack.push(((i, j + 1), 'd'));
                }
                ('J', 'u') | ('J', 'l') => {
                    stack.push(((i - 1, j), 'l'));
                    stack.push(((i, j - 1), 'u'));
                    stack.push(((i, j + 1), 'r'));
                    stack.push(((i + 1, j), 'd'));
                }
                ('J', 'd') | ('J', 'r') => {
                    stack.push(((i - 1, j), 'r'));
                    stack.push(((i, j - 1), 'd'));
                }
                ('7', 'd') | ('7', 'l') => {
                    stack.push(((i - 1, j), 'u'));
                    stack.push(((i, j - 1), 'd'));
                    stack.push(((i, j + 1), 'r'));
                    stack.push(((i + 1, j), 'l'));
                }
                ('7', 'u') | ('7', 'r') => {
                    stack.push(((i + 1, j), 'r'));
                    stack.push(((i, j - 1), 'u'));
                }
                ('F', 'd') | ('F', 'r') => {
                    stack.push(((i - 1, j), 'u'));
                    stack.push(((i, j - 1), 'l'));
                    stack.push(((i, j + 1), 'd'));
                    stack.push(((i + 1, j), 'r'));
                }
                ('F', 'l') | ('F', 'u') => {
                    stack.push(((i + 1, j), 'l'));
                    stack.push(((i, j + 1), 'u'));
                }
                (c, d) => panic!("{} {} invalid combo", c, d),
            };
        } else {
            stack.push(((i - 1, j), 'u'));
            stack.push(((i, j - 1), 'l'));
            stack.push(((i, j + 1), 'r'));
            stack.push(((i + 1, j), 'd'));
        }
        area += 1;
    }
    area
}

impl Solution for Day10_1 {
    fn name(&self) -> &str {
        "day10_pipe_maze"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        find_pipes(&grid).len().div(2).to_string()
    }
}

impl Solution for Day10_2 {
    fn name(&self) -> &str {
        "day10_pipe_maze"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let total_area = find_area(&grid);
        let pipe_len = find_pipes(&grid).len() as u32;
        println!("total area: {}", total_area);
        println!("pipes: {}", pipe_len);
        (total_area - pipe_len).to_string()
    }
}
