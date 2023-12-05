use crate::utils::Solution;

use itertools::Itertools;
use std::cmp::{max, min};

pub struct Day05_1;

struct RangeMap {
    ranges: Vec<(u64, u64, u64)>,
}

fn collapse_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.len() == 0 {
        return Vec::new();
    };
    let mut ret = vec![ranges[0]];
    for (start, end) in ranges[1..].iter().cloned() {
        let (cur_start, cur_end) = ret.pop().unwrap();
        if start <= cur_end {
            ret.push((cur_start, max(end, cur_end)));
        } else {
            ret.push((cur_start, cur_end));
            ret.push((start, end));
        }
    }
    return ret;
}

impl RangeMap {
    pub fn new(ranges: &Vec<(u64, u64, u64)>) -> Self {
        return Self {
            ranges: ranges
                .iter()
                .sorted_by_key(|(_, i, _)| i)
                .cloned()
                .collect_vec(),
        };
    }

    pub fn chain_range(&self, ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut mapped_ranges = ranges
            .iter()
            .map(|(start, end)| self.get_range(*start, *end))
            .flatten()
            .collect_vec();
        mapped_ranges.sort_by_key(|(start, _)| *start);
        return collapse_ranges(&mapped_ranges);
    }

    pub fn get(&self, e: u64) -> u64 {
        let range = self.get_range(e, e + 1);
        assert!(range.len() == 1, "Should return 1 element.");
        return range[0].0;
    }

    pub fn get_range(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        // For a range [start, end), maps the range to the output range.
        let mut ret: Vec<(u64, u64)> = Vec::new();
        let mut cur = start;
        for (j, i, n) in self.ranges.iter().cloned() {
            if cur >= end {
                break;
            }
            if i > cur {
                // Need to map the 1-1s
                let range_end = min(end, i);
                ret.push((cur, range_end));
                cur = range_end;
            } else if cur < i + n && end > i {
                // Found valid map.
                let range_end = min(i + n, end);
                let start_offset = cur - i;
                let q = range_end - cur;
                ret.push((j + start_offset, j + start_offset + q));
                cur = range_end;
            }
        }
        if cur < end {
            // 1-1s for range above the map.
            ret.push((cur, end));
        }
        return ret;
    }
}

fn parse_file(lines: &Vec<String>) -> (Vec<u64>, Vec<RangeMap>) {
    // seeds
    let (_, seeds_line) = lines[0].split_once(':').unwrap();
    let seeds: Vec<u64> = seeds_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    println!("seeds: {:?}", seeds);

    let mut rmaps = Vec::new();
    let mut ranges = Vec::new();
    for line in &lines[2..] {
        if line.is_empty() {
            continue;
        } else if line.contains("map:") {
            println!("ranges: {:?}", ranges);
            rmaps.push(RangeMap::new(&ranges));
            ranges.clear();
        } else {
            let m: Vec<u64> = line
                .splitn(3, ' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            ranges.push((m[0], m[1], m[2]));
        }
    }
    if !ranges.is_empty() {
        println!("ranges: {:?}", ranges);
        rmaps.push(RangeMap::new(&ranges));
    }
    return (seeds, rmaps);
}

fn chain_map(seed: u64, maps: &Vec<RangeMap>) -> u64 {
    return maps.iter().fold(seed, |acc, map| {
        println!("seed: {} | acc: {}", seed, acc);
        map.get(acc)
    });
}

impl Solution for Day05_1 {
    fn name(&self) -> &str {
        "day05_fertilizer"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        let (seeds, maps) = parse_file(lines);
        return seeds
            .iter()
            .map(|seed| chain_map(*seed, &maps))
            .min()
            .unwrap()
            .to_string();
    }
}
