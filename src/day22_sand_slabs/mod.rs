use std::collections::{HashMap, HashSet};

use crate::utils::Solution;
use itertools::Itertools;
pub struct Day22_1;

#[derive(Clone, Debug)]
struct Brick {
    pub id: usize,
    pub c0: (usize, usize, usize),
    pub c1: (usize, usize, usize),
}

impl Brick {
    fn height(&self) -> usize {
        self.c1.2 - self.c0.2 + 1
    }
}

fn parse(lines: &[String]) -> Vec<Brick> {
    lines
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let (c0s, c1s) = x.split_once('~').unwrap();
            Brick {
                id: i + 1,
                c0: c0s
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
                c1: c1s
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            }
        })
        .collect_vec()
}

fn construct_graph(bricks: &[Brick]) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let max_x = bricks.iter().map(|x| x.c0.0).max().unwrap() + 1;
    let max_y = bricks.iter().map(|x| x.c0.1).max().unwrap() + 1;
    println!("{max_x} {max_y}");
    let sorted_bricks = bricks.iter().sorted_by_key(|x| x.c0.2);
    let mut grid = vec![vec![(0, 0); max_y]; max_x];
    let mut in_graph = HashMap::new();
    let mut out_graph = HashMap::new();

    for brick in sorted_bricks {
        println!("{} {:?}", brick.id, grid);
        out_graph.insert(brick.id, vec![]);
        let mut max_height = 0;
        let mut adj = HashSet::new();
        for row in grid.iter().take(brick.c1.0 + 1).skip(brick.c0.0) {
            for &(cell_height, brid) in row {
                if cell_height > max_height {
                    max_height = cell_height;
                    adj.clear();
                }
                if cell_height >= max_height && brid != 0 {
                    adj.insert(brid);
                }
            }
        }
        adj.iter().for_each(|x| {
            out_graph.get_mut(x).unwrap().push(brick.id);
        });
        in_graph.insert(brick.id, adj.into_iter().collect_vec());
        for row in grid.iter_mut().take(brick.c1.0 + 1).skip(brick.c0.0) {
            for v in row.iter_mut().take(brick.c1.1 + 1).skip(brick.c0.1) {
                *v = (max_height + brick.height(), brick.id);
            }
        }
        println!("{} {:?}", brick.id, grid);
    }
    (in_graph, out_graph)
}

impl Solution for Day22_1 {
    fn name(&self) -> &str {
        "day22_sand_slabs"
    }
    fn solve(&self, lines: &[String]) -> String {
        let bricks = parse(lines);
        let (in_graph, out_graph) = construct_graph(&bricks);
        println!("In Graph: {in_graph:?}");
        println!("Out Graph: {out_graph:?}");
        let n_safe = out_graph
            .iter()
            .sorted_by_key(|x| x.0)
            .map(|(brid, supports)| {
                let is_safe = supports.iter().all(|x| in_graph[x].len() > 1);
                println!("{brid} | {:?} | {is_safe}", bricks[brid - 1].c0);
                is_safe as usize
            })
            .sum::<usize>();
        n_safe.to_string()
    }
}
