use crate::utils::Solution;
use itertools::Itertools;
pub struct Day15_1;

fn compute_hash(word: &str) -> u32 {
    word.chars().fold(0, |acc, e| ((acc + e as u32) * 17) % 256)
}

impl Solution for Day15_1 {
    fn name(&self) -> &str {
        "day15_lens_library"
    }
    fn solve(&self, lines: &[String]) -> String {
        let words = lines[0].split(',').collect_vec();
        words
            .iter()
            .map(|x| {
                let hash = compute_hash(x);
                println!("{x} | {hash}");
                hash
            })
            .sum::<u32>()
            .to_string()
    }
}
