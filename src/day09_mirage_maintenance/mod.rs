use crate::utils::Solution;
use itertools::Itertools;

pub struct Day09_1;
pub struct Day09_2;

fn pred_next(v: &[i32]) -> i32 {
    let mut s: i32 = 0;
    let mut v = v.iter().cloned().collect_vec();
    while !v.iter().all(|x| *x == 0) {
        s += v[v.len() - 1];
        v = (1..v.len()).map(|i| v[i] - v[i - 1]).collect_vec();
    }
    s
}

fn pred_before(v: &[i32]) -> i32 {
    let mut s: i32 = 0;
    let mut v = v.iter().cloned().collect_vec();
    for i in 0.. {
        if v.iter().all(|x| *x == 0) {
            break;
        }
        if i % 2 == 0 {
            s += v[0];
        } else {
            s -= v[0];
        }

        v = (1..v.len()).map(|i| v[i] - v[i - 1]).collect_vec();
    }
    s
}

fn solve(lines: &[String], solver: fn(&[i32]) -> i32) -> i32 {
    let ans = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|v| (solver(&v), v))
        .collect_vec();
    ans.iter().map(|(s, _)| s).sum::<i32>()
}

impl Solution for Day09_1 {
    fn name(&self) -> &str {
        "day09_mirage_maintenance"
    }
    fn solve(&self, lines: &[String]) -> String {
        solve(lines, pred_next).to_string()
    }
}

impl Solution for Day09_2 {
    fn name(&self) -> &str {
        "day09_mirage_maintenance"
    }
    fn solve(&self, lines: &[String]) -> String {
        solve(lines, pred_before).to_string()
    }
}
