use crate::utils::Solution;
use itertools::Itertools;
pub struct Day11_1;

fn find_expanded_rows_cols(grid: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let expanded_rows = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|v| *v == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec();

    let expanded_cols = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).all(|i| grid[i][j] == '.'))
        .collect_vec();
    (expanded_rows, expanded_cols)
}

fn find_galaxies(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter_map(|(j, &c)| if c == '#' { Some((i, j)) } else { None })
                .collect_vec()
        })
        .collect_vec()
}

fn count_between(x1: usize, x2: usize, v: &[usize]) -> usize {
    v.iter()
        .filter(|&&x| (x1 > x && x > x2) || (x2 > x && x > x1))
        .count()
}

impl Solution for Day11_1 {
    fn name(&self) -> &str {
        "day11_cosmic_expansion"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines
            .iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let (erows, ecols) = find_expanded_rows_cols(&grid);
        println!("Rows expanded: {:?}", erows);
        println!("Cols expanded: {:?}", ecols);

        let galaxies = find_galaxies(&grid);
        let mut sum = 0;
        for ((i1, (y1, x1)), (i2, (y2, x2))) in galaxies
            .into_iter()
            .enumerate()
            .tuple_combinations::<((usize, (usize, usize)), (usize, (usize, usize)))>()
        {
            let dx = (x1).abs_diff(x2);
            let dx_add = count_between(x1, x2, &ecols);
            let dy = (y1).abs_diff(y2);
            let dy_add = count_between(y1, y2, &erows);

            let d = dx + dy + dx_add + dy_add;
            println!(
                "{i1} to {i2} | {:?} to {:?} | ({dx} + {dx_add}, {dy} + {dy_add}) = {d}",
                (x1, y1),
                (x2, y2)
            );
            sum += d;
        }
        sum.to_string()
    }
}
