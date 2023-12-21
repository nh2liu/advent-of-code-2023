use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use num::integer::lcm;

use crate::utils::Solution;
pub struct Day20_1;
pub struct Day20_2;

#[derive(Clone)]
enum PulseNode<'a> {
    Normal,
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<&'a str, bool> },
}
#[derive(Clone)]
struct PulseGraph<'a> {
    neighbor_graph: HashMap<&'a str, Vec<&'a str>>,
    states: HashMap<&'a str, PulseNode<'a>>,
}

impl<'a> PulseGraph<'a> {
    fn push(&mut self, catch: Option<&str>) -> ((usize, usize), bool) {
        let mut lowc = 1;
        let mut highc = 0;
        let mut q: VecDeque<(&'a str, &'a str, bool)> =
            VecDeque::from([("button", "broadcaster", false)]);

        let mut high_fired = false;
        while let Some((prev, cur, is_high)) = q.pop_front() {
            if let Some(x) = catch {
                if x == prev && is_high {
                    high_fired = true;
                }
            }
            let mut send_to_neighbors = |cur_node, neighbors: &[&'a str], is_high| {
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
        ((lowc, highc), high_fired)
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
        let mut cum_low = 0;
        let mut cum_high = 0;
        for _ in 0..1000 {
            let (count, _) = g.push(None);
            cum_low += count.0;
            cum_high += count.1;
        }
        println!("Low count: {cum_low} | High count: {cum_high}");
        (cum_low * cum_high).to_string()
    }
}

impl Solution for Day20_2 {
    fn name(&self) -> &str {
        "day20_pulse_propagation"
    }
    fn solve(&self, lines: &[String]) -> String {
        let g = parse(lines);

        let in_nodes = if let PulseNode::Conjunction { inputs } = &g.states["sq"] {
            inputs.keys().cloned().collect_vec()
        } else {
            panic!("sq not valid.")
        };
        println!("{in_nodes:?}");

        let cycles = in_nodes
            .iter()
            .map(|x| {
                let mut tg = g.clone();
                for i in 1i64.. {
                    let (_, high_fired) = tg.push(Some(x));
                    if high_fired {
                        println!("{x} {i}");
                        return i;
                    }
                }
                panic!("must return");
            })
            .collect_vec();
        cycles.into_iter().reduce(lcm).unwrap().to_string()
    }
}
