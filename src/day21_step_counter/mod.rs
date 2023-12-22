use crate::utils::{in_bounds, Solution};
use itertools::Itertools;
pub struct Day21_1;

pub struct Day21_2;

fn find_s(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &c)| if c == 'S' { Some(j) } else { None })
                .next()
                .map(|j| (i, j))
        })
        .next()
        .unwrap()
}

fn bfs_solve(grid: &[Vec<char>], i: usize, j: usize, n: usize) -> Vec<Vec<isize>> {
    let mut steps_til = vec![vec![-1; grid[0].len()]; grid.len()];
    steps_til[i][j] = 0;

    let mut to_process = vec![(i, j)];
    for step in 0..n {
        if to_process.is_empty() {
            break;
        }
        let mut next_process = Vec::new();
        for &(i, j) in to_process.iter() {
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let (ki, li) = ((i as isize + dy), (j as isize + dx));
                if in_bounds(grid, ki, li) {
                    let (k, l) = (ki as usize, li as usize);
                    if grid[k][l] != '#' && steps_til[k][l] < 0 {
                        steps_til[k][l] = step as isize + 1;
                        next_process.push((k, l));
                    }
                }
            }
        }

        to_process.clear();
        to_process.append(&mut next_process);
    }
    steps_til
}

fn count_squares(steps_til: &[Vec<isize>], steps: usize) -> usize {
    steps_til
        .iter()
        .flatten()
        .filter(|&&x| x >= 0 && x % 2 == (steps as isize % 2))
        .count()
}

impl Solution for Day21_1 {
    fn name(&self) -> &str {
        "day21_step_counter"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        let (i, j) = find_s(&grid);
        let steps_til = bfs_solve(&grid, i, j, 64);
        count_squares(&steps_til, 64).to_string()
    }
}

impl Solution for Day21_2 {
    fn name(&self) -> &str {
        "day21_step_counter"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        //let x = m + 65;
        let m = grid.len();
        let n = m / 2;

        let x = 26501365;

        //let x = 2 * m + n;
        println!("m={m} n={n}");

        let l: usize = (x - n) / m;
        let solve = |i, j, n| count_squares(&bfs_solve(&grid, i, j, n), n);

        let mut ans = 0;
        // Center squares
        let (i, j) = find_s(&grid);
        let (n_odd_square, n_even_square) = if l % 2 == 1 {
            (l.pow(2), (l - 1).pow(2))
        } else {
            ((l - 1).pow(2), l.pow(2))
        };
        let odd_points = solve(i, j, x);
        let even_points = solve(i, j, x + 1);
        let center_contribution = n_odd_square * odd_points + n_even_square * even_points;
        ans += center_contribution;

        // Partial Grids
        let n_incomplete_small = l;
        let small_contribution = n_incomplete_small
            * (solve(0, 0, n - 1)
                + solve(m - 1, 0, n - 1)
                + solve(0, m - 1, n - 1)
                + solve(m - 1, m - 1, n - 1));
        ans += small_contribution;

        let n_incomplete_big = l.saturating_sub(1);

        let big_contribution = n_incomplete_big
            * (solve(0, 0, m + n - 1)
                + solve(m - 1, 0, m + n - 1)
                + solve(0, m - 1, m + n - 1)
                + solve(m - 1, m - 1, m + n - 1));
        ans += big_contribution;

        // Corners
        let corner_contribution = solve(0, n, m - 1)
            + solve(n, 0, m - 1)
            + solve(n, m - 1, m - 1)
            + solve(m - 1, n, m - 1);
        ans += corner_contribution;
        println!("length={l}, odd_center={n_odd_square}, even_center={n_even_square}, small={n_incomplete_small}, big={n_incomplete_big}");
        println!("center={center_contribution}, small={small_contribution}, big={big_contribution}, corner={corner_contribution}");
        ans.to_string()
    }
}
