use crate::utils::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day23_1;
pub struct Day23_2;

type StackState = Option<Vec<(isize, isize)>>;
type WeightedGraph = HashMap<(isize, isize), HashMap<(isize, isize), usize>>;

fn reduce_graph(grid: &[Vec<char>], is_part2: bool) -> WeightedGraph {
    let mut reduced_graph: WeightedGraph = HashMap::new();
    for iu in 1..grid.len() - 1 {
        for ju in 1..grid[0].len() - 1 {
            let (i, j) = (iu as isize, ju as isize);
            let all_adjacent = || {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .map(|(dy, dx)| (i + dy, j + dx))
                    .into_iter()
                    .collect_vec()
            };
            let adjacents = match grid[iu][ju] {
                'S' | '#' => {
                    vec![]
                }
                '.' => all_adjacent(),
                _ if is_part2 => all_adjacent(),
                'v' => vec![(i + 1, j)],
                '^' | 'E' => vec![(i - 1, j)],
                '<' => vec![(i, j - 1)],
                '>' => vec![(i, j + 1)],
                e => panic!("Not valid: {e}"),
            };
            let lengths: HashMap<(isize, isize), usize> = adjacents
                .into_iter()
                .filter_map(|(k, l)| {
                    if !['S', '#'].contains(&grid[k as usize][l as usize]) {
                        Some(((k, l), 0))
                    } else {
                        None
                    }
                })
                .collect();
            reduced_graph.insert((i, j), lengths);
        }
    }
    let m = grid.len() as isize;
    let n = grid[0].len() as isize;
    reduced_graph.insert((m - 1, n - 2), HashMap::from([((m - 2, n - 2), 0)]));
    let coords = reduced_graph.keys().cloned().collect_vec();
    for coord in coords {
        let adjacents = reduced_graph[&coord]
            .iter()
            .map(|(&x, &y)| (x, y))
            .collect_vec();
        if adjacents.len() == 2 {
            let (c1, l1) = adjacents[0];
            let (c2, l2) = adjacents[1];
            let new_len = l1 + l2 + 1;
            reduced_graph.remove(&coord);
            reduced_graph.get_mut(&c1).unwrap().remove(&coord);
            reduced_graph.get_mut(&c2).unwrap().remove(&coord);
            reduced_graph.get_mut(&c1).unwrap().insert(c2, new_len);
            reduced_graph.get_mut(&c2).unwrap().insert(c1, new_len);
        };
    }
    reduced_graph
}

fn walk_on_weighted(grid: &[Vec<char>], g: WeightedGraph) -> usize {
    let mut stack: Vec<((isize, isize), StackState, usize)> = vec![((1, 1), None, 0)];
    let mut path_so_far = HashSet::<(isize, isize)>::from([(1, 0)]);
    let mut path_len_so_far = 1;
    let mut max_length = 0;
    while let Some(((i, j), state, sub_val)) = stack.pop() {
        if let Some(mut to_process) = state {
            path_len_so_far -= sub_val;
            if to_process.is_empty() {
                path_len_so_far -= 1;
                path_so_far.remove(&(i, j));
            } else {
                let next = to_process.pop().unwrap();
                let w = g[&(i, j)][&next];
                path_len_so_far += w;
                stack.push(((i, j), Some(to_process), w));
                stack.push((next, None, 0));
            }
        } else if path_so_far.contains(&(i, j)) {
            continue;
        } else if grid[i as usize][j as usize] == 'E' {
            max_length = max_length.max(path_len_so_far);
        } else {
            path_so_far.insert((i, j));
            path_len_so_far += 1;
            stack.push(((i, j), Some(g[&(i, j)].keys().cloned().collect_vec()), 0));
        }
    }
    max_length
}

#[allow(dead_code)]
fn long_walk(grid: &[Vec<char>], is_part2: bool) -> usize {
    let mut stack: Vec<((isize, isize), StackState)> = vec![((1, 1), None)];
    let mut path_so_far = HashSet::<(isize, isize)>::from([(1, 0)]);
    let mut max_length = 0;
    while let Some(((i, j), state)) = stack.pop() {
        if let Some(mut to_process) = state {
            if to_process.is_empty() {
                path_so_far.remove(&(i, j));
            } else {
                let next = to_process.pop().unwrap();
                stack.push(((i, j), Some(to_process)));
                stack.push((next, None));
            }
        } else {
            if path_so_far.contains(&(i, j)) {
                continue;
            }
            match grid[i as usize][j as usize] {
                '#' | 'S' => {}
                'E' => {
                    max_length = max_length.max(path_so_far.len());
                }
                e => {
                    path_so_far.insert((i, j));
                    let all_adjacent = || {
                        [(1, 0), (-1, 0), (0, 1), (0, -1)]
                            .map(|(dy, dx)| (i + dy, j + dx))
                            .into_iter()
                            .collect_vec()
                    };
                    let to_process = match e {
                        '.' => all_adjacent(),
                        _ if is_part2 => all_adjacent(),
                        'v' => vec![(i + 1, j)],
                        '^' => vec![(i - 1, j)],
                        '<' => vec![(i, j - 1)],
                        '>' => vec![(i, j + 1)],
                        _ => panic!("Not valid: {e}"),
                    };
                    stack.push(((i, j), Some(to_process)));
                }
            }
        }
    }
    max_length
}

impl Solution for Day23_1 {
    fn name(&self) -> &str {
        "day23_a_long_walk"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        //long_walk(&grid, false).to_string()

        let g = reduce_graph(&grid, false);
        walk_on_weighted(&grid, g).to_string()
    }
}

impl Solution for Day23_2 {
    fn name(&self) -> &str {
        "day23_a_long_walk"
    }
    fn solve(&self, lines: &[String]) -> String {
        let grid = lines.iter().map(|x| x.chars().collect_vec()).collect_vec();
        //long_walk(&grid, true).to_string()

        let g = reduce_graph(&grid, true);
        walk_on_weighted(&grid, g).to_string()
    }
}
