use std::collections::HashMap;

use crate::utils::{in_bounds, Solution};
use itertools::Itertools;

pub struct Day16_1;
pub struct Day16_2;

fn energize(grid: &[Vec<char>], (i, j, d): (usize, usize, char)) -> u32 {
    let mut n_visited = 0;
    let mut mut_visited = HashMap::new();
    let mut stack = vec![(i as isize, j as isize, d)];
    while let Some((i, j, d)) = stack.pop() {
        if !in_bounds(grid, i, j) {
            continue;
        } else if let std::collections::hash_map::Entry::Vacant(e) = mut_visited.entry((i, j)) {
            n_visited += 1;
            e.insert(vec![d]);
        } else if mut_visited[&(i, j)].contains(&d) {
            // Cache hit
            continue;
        }
        let (right, left, down, up) = (
            (i, j + 1, 'r'),
            (i, j - 1, 'l'),
            (i + 1, j, 'd'),
            (i - 1, j, 'u'),
        );
        let mut next = match (grid[i as usize][j as usize], d) {
            ('-', 'r') | ('.', 'r') => vec![right],
            ('-', 'l') | ('.', 'l') => vec![left],
            ('|', 'd') | ('.', 'd') => vec![down],
            ('|', 'u') | ('.', 'u') => vec![up],
            ('|', 'r') | ('|', 'l') => vec![down, up],
            ('-', 'd') | ('-', 'u') => vec![right, left],
            ('\\', 'r') => vec![down],
            ('\\', 'l') => vec![up],
            ('\\', 'd') => vec![right],
            ('\\', 'u') => vec![left],
            ('/', 'r') => vec![up],
            ('/', 'l') => vec![down],
            ('/', 'd') => vec![left],
            ('/', 'u') => vec![right],
            _ => panic!("Invalid input {} {}", grid[i as usize][j as usize], d),
        };
        stack.append(&mut next);
    }
    n_visited
}

impl Solution for Day16_1 {
    fn name(&self) -> &str {
        "day16_lava_floor"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        energize(&grid, (0, 0, 'r')).to_string()
    }
}

impl Solution for Day16_2 {
    fn name(&self) -> &str {
        "day16_lava_floor"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        let (m, n) = (grid.len(), grid[0].len());
        let mut v1 = (0..m)
            .flat_map(|i| vec![(i, 0, 'r'), (i, n - 1, 'l')])
            .collect_vec();
        let v2 = (0..n)
            .flat_map(|j| vec![(0, j, 'd'), (m - 1, j, 'u')])
            .collect_vec();
        v1.extend(v2);

        v1.into_iter()
            .map(|x| energize(&grid, x))
            .max()
            .unwrap()
            .to_string()
    }
}
