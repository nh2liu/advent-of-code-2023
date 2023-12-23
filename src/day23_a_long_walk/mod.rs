use crate::utils::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day23_1;

fn long_walk(grid: &[Vec<char>]) -> usize {
    fn helper(
        grid: &[Vec<char>],
        path_so_far: &mut HashSet<(isize, isize)>,
        (i, j): (isize, isize),
    ) -> usize {
        if path_so_far.contains(&(i, j)) {
            return 0;
        }
        match grid[i as usize][j as usize] {
            '#' | 'S' => 0,
            'E' => path_so_far.len(),
            e => {
                path_so_far.insert((i, j));
                let max_len = match e {
                    '.' => *[(1, 0), (-1, 0), (0, 1), (0, -1)]
                        .map(|(dy, dx)| helper(grid, path_so_far, (i + dy, j + dx)))
                        .iter()
                        .max()
                        .unwrap(),
                    'v' => helper(grid, path_so_far, (i + 1, j)),
                    '^' => helper(grid, path_so_far, (i - 1, j)),
                    '<' => helper(grid, path_so_far, (i, j - 1)),
                    '>' => helper(grid, path_so_far, (i, j + 1)),
                    _ => panic!("Not valid"),
                };
                path_so_far.remove(&(i, j));
                max_len
            }
        }
    }
    helper(grid, &mut HashSet::<(isize, isize)>::from([(0, 1)]), (1, 1))
}

impl Solution for Day23_1 {
    fn name(&self) -> &str {
        "day23_a_long_walk"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        long_walk(&grid).to_string()
    }
}
