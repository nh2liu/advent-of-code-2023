use itertools::Itertools;

use crate::utils::Solution;

pub struct Day06_1;

fn count_ways(time: u32, record: u32) -> u32 {
    // distance * speed = record
    // (time - hold) * hold = record
    // hold^2 - time * hold + record = 0
    // Strictly convex, find bounds.
    let discriminant = ((time * time - 4 * (record + 1)) as f32).sqrt();
    let left_bound = (time as f32 - discriminant) / 2.;
    let right_bound = (time as f32 + discriminant) / 2.;
    let n_ways: u32 = right_bound.floor() as u32 - left_bound.ceil() as u32 + 1;
    println!(
        "{} {} | {:.2} {:.2} {} ",
        time, record, left_bound, right_bound, n_ways
    );
    return n_ways;
}

impl Solution for Day06_1 {
    fn name(&self) -> &str {
        "day06_wait_for_it"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let parsed = lines
            .iter()
            .map(|x| {
                let (_, line) = x.split_once(':').unwrap();
                line.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let (times, records) = (&parsed[0], &parsed[1]);
        return times
            .iter()
            .zip(records)
            .map(|(time, record)| count_ways(*time, *record))
            .product::<u32>()
            .to_string();
    }
}
