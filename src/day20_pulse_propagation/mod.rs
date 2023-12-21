use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::utils::Solution;
pub struct Day20_1;

enum PulseNode<'a> {
    Normal,
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<&'a str, bool> },
}

struct PulseGraph<'a> {
    neighbor_graph: HashMap<&'a str, Vec<&'a str>>,
    states: HashMap<&'a str, PulseNode<'a>>,
}

impl<'a> PulseGraph<'a> {
    fn push(&'a mut self, n: usize) -> (usize, usize) {
        let mut cum_low = 0;
        let mut cum_high = 0;
        for _ in 0..n {
            let mut lowc = 1;
            let mut highc = 0;
            let mut q: VecDeque<(&'a str, &'a str, bool)> =
                VecDeque::from([("button", "broadcaster", false)]);
            while let Some((prev, cur, is_high)) = q.pop_front() {
                println!("{prev} -> {cur} | {is_high}");
                let mut send_to_neighbors = |cur_node, neighbors: &'a [&str], is_high| {
                    if is_high {
                        highc += neighbors.len();
                    } else {
                        lowc += neighbors.len();
                    }
                    neighbors
                        .iter()
                        .for_each(|n| q.push_back((cur_node, n, is_high)))
                };
                if !self.states.contains_key(cur) {
                    // terminal
                    continue;
                }
                let pn = self.states.get_mut(cur).unwrap();
                let neighbors = &self.neighbor_graph[cur];
                match pn {
                    PulseNode::Normal => {
                        send_to_neighbors(cur, neighbors, is_high);
                    }
                    PulseNode::Conjunction { ref mut inputs } => {
                        inputs.insert(prev, is_high);
                        send_to_neighbors(cur, neighbors, !inputs.values().all(|&x| x));
                    }
                    PulseNode::FlipFlop { ref mut state } => {
                        if !is_high {
                            *state = !*state;
                            send_to_neighbors(cur, neighbors, *state);
                        }
                    }
                };
            }
            cum_low += lowc;
            cum_high += highc;
        }
        (cum_low, cum_high)
    }
}

fn parse(lines: &[String]) -> PulseGraph {
    let (neighbors_graph, mut states): (HashMap<&str, Vec<&str>>, HashMap<&str, PulseNode>) = lines
        .iter()
        .map(|line| {
            let (node_str, neighbors_str) = line.split_once(" -> ").unwrap();
            let neighbors = neighbors_str.split(", ").collect_vec();

            if let Some(name) = node_str.strip_prefix('%') {
                (
                    (name, neighbors),
                    (name, PulseNode::FlipFlop { state: false }),
                )
            } else if let Some(name) = node_str.strip_prefix('&') {
                (
                    (name, neighbors),
                    (
                        name,
                        PulseNode::Conjunction {
                            inputs: HashMap::new(),
                        },
                    ),
                )
            } else {
                ((node_str, neighbors), (node_str, PulseNode::Normal))
            }
        })
        .unzip();

    for (node, neighbors) in neighbors_graph.iter() {
        neighbors.iter().for_each(|neighbor| {
            if !states.contains_key(neighbor) {
                return;
            }
            if let PulseNode::Conjunction { inputs } = states.get_mut(neighbor).unwrap() {
                inputs.insert(node, false);
            }
        })
    }
    PulseGraph {
        states,
        neighbor_graph: neighbors_graph,
    }
}

impl Solution for Day20_1 {
    fn name(&self) -> &str {
        "day20_pulse_propagation"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut g = parse(lines);
        let (lowc, highc) = g.push(1000);
        println!("Low count: {lowc} | High count: {highc}");
        (lowc * highc).to_string()
    }
}
