use crate::utils::Solution;
use itertools::Itertools;
pub struct Day14_1;

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
        let ans = tilted_grid
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&&x| x == 'O').count() * (tilted_grid.len() - i))
            .sum::<usize>();
        ans.to_string()
    }
}
