use crate::utils::{in_bounds, Solution};
use itertools::Itertools;
use sprs::{CsMat, CsVec, TriMat};

pub struct Day21_1;

fn get_idx(i: usize, j: usize, grid: &[Vec<char>]) -> usize {
    i * grid[0].len() + j
}

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

fn construct_adj_matrix(grid: &[Vec<char>]) -> CsMat<usize> {
    let n_values = grid.len() * grid[0].len();
    let mut a = TriMat::new((n_values, n_values));
    for (i, row) in (0..grid.len()).zip(grid.iter()) {
        for (j, &c) in (0..row.len()).zip(row.iter()) {
            if c != '#' {
                let idx = get_idx(i, j, grid);

                for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (1, 0)] {
                    let (k, l) = ((i as isize + dy), (j as isize + dx));
                    if in_bounds(grid, k, l) && grid[k as usize][l as usize] != '#' {
                        let idx2 = get_idx(k as usize, l as usize, grid);
                        a.add_triplet(idx, idx2, 1);
                        a.add_triplet(idx2, idx, 1);
                    }
                }
            }
        }
    }
    a.to_csr()
}

impl Solution for Day21_1 {
    fn name(&self) -> &str {
        "day21_step_counter"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        let (i, j) = find_s(&grid);
        let start_idx = get_idx(i, j, &grid);

        let adj_mat = construct_adj_matrix(&grid);
        let mut v = CsVec::new(adj_mat.rows(), vec![start_idx], vec![1]);
        for i in 0..64 {
            v = &adj_mat * &v;
            v = v.map(|_| 1);
            println!("{i} {}", v.nnz());
        }
        v.nnz().to_string()
    }
}
