use std::collections::HashMap;

use crate::utils::Solution;

pub struct Day08_1;

fn parse(lines: &[String]) -> (&str, HashMap<&str, (&str, &str)>) {
    let seq = lines[0].as_str();
    let nodes = lines[2..]
        .iter()
        .map(|s| {
            let (p1, p2) = s.split_once(" = ").unwrap();
            let (p3, p4) = p2.split_once(',').unwrap();
            (p1.trim(), (p3[1..].trim(), p4[..p4.len() - 1].trim()))
        })
        .collect();
    (seq, nodes)
}

impl Solution for Day08_1 {
    fn name(&self) -> &str {
        "day08_haunted_wasteland"
    }
    fn solve(&self, lines: &[String]) -> String {
        let (seq, nodes) = parse(lines);
        let mut cur_node = "AAA";
        let mut n: u32 = 0;
        for s in seq.chars().cycle() {
            println!("{} | {}", n, cur_node);
            if cur_node == "ZZZ" {
                break;
            }
            let options = nodes[cur_node];
            cur_node = match s {
                'L' => options.0,
                'R' => options.1,
                _ => panic!("Invalid char for step {}", s),
            };
            n += 1;
        }
        n.to_string()
    }
}
