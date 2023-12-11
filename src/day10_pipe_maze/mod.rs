use crate::utils::Solution;
use itertools::Itertools;
use std::ops::Div;
pub struct Day10_1;

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

fn solve1(grid: &[Vec<char>]) -> usize {
    let ((mut i, mut j), mut dir) = find_s(grid);

    for n in 1.. {
        let c = grid[i][j];
        println!("{c} {dir} ({i}, {j})");

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
            return n;
        }
        (i, j) = match dir {
            'd' => (i + 1, j),
            'u' => (i - 1, j),
            'l' => (i, j - 1),
            'r' => (i, j + 1),
            _ => panic!("lol"),
        }
    }
    panic!("This can't happen.")
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
        solve1(&grid).div(2).to_string()
    }
}
