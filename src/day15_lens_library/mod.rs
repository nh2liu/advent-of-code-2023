use crate::utils::Solution;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
pub struct Day15_1;
pub struct Day15_2;

fn compute_hash(word: &str) -> usize {
    word.chars()
        .fold(0, |acc, e| ((acc + e as usize) * 17) % 256)
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
            .sum::<usize>()
            .to_string()
    }
}

struct LensMap {
    boxes: [Vec<String>; 256],
    hm: HashMap<String, u32>,
}

impl LensMap {
    fn new() -> Self {
        LensMap {
            boxes: [(); 256].map(|_| Vec::new()),
            hm: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, val: u32) {
        if !self.hm.contains_key(key) {
            self.boxes[compute_hash(key)].push(key.to_string());
        }
        self.hm.insert(key.to_string(), val);
    }
    fn remove(&mut self, key: &str) {
        self.hm.remove(key);
        self.boxes[compute_hash(key)].retain(|x| x != key);
    }

    fn compute_focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, v)| {
                (i + 1) as u32
                    * v.iter()
                        .enumerate()
                        .map(|(j, v)| (j + 1) as u32 * self.hm[v])
                        .sum::<u32>()
            })
            .sum::<u32>()
    }
}

impl fmt::Display for LensMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .boxes
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if e.is_empty() {
                    None
                } else {
                    let contents = e
                        .iter()
                        .map(|k| format!("[{} {}]", k, self.hm[k]))
                        .collect_vec()
                        .join(" ");
                    Some(format!("Box {}: {}", i, contents))
                }
            })
            .join("\n");
        write!(f, "{}", s.as_str())
    }
}

impl Solution for Day15_2 {
    fn name(&self) -> &str {
        "day15_lens_library"
    }
    fn solve(&self, lines: &[String]) -> String {
        let words = lines[0].split(',').collect_vec();
        let mut lm = LensMap::new();
        for word in words {
            if let Some(s) = word.strip_suffix('-') {
                lm.remove(s);
            } else {
                let (key, val) = word.split_once('=').unwrap();
                lm.insert(key, val.parse::<u32>().unwrap());
            }
        }
        println!("{}", lm);
        lm.compute_focusing_power().to_string()
    }
}
