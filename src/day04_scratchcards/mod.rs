use crate::utils::Solution;

use std::collections::HashSet;

pub struct Day04_1;

fn score(winners: &HashSet<u32>, hand: &HashSet<u32>) -> u32 {
    let n_winners = winners.intersection(hand).collect::<Vec<_>>().len();
    if n_winners == 0 {
        return 0;
    } else {
        return 2u32.pow((n_winners - 1) as u32);
    }
}

fn parse_line(line: &String) -> (HashSet<u32>, HashSet<u32>) {
    let card = line.split_once(':').unwrap().1;
    let (winners_string, hand_string) = card.split_once('|').unwrap();
    return (
        winners_string
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect(),
        hand_string
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect(),
    );
}

impl Solution for Day04_1 {
    fn name(&self) -> &str {
        "day04_scratchcards"
    }
    fn solve(&self, lines: &Vec<String>) -> String {
        return lines
            .iter()
            .map(parse_line)
            .map(|(winners, hand)| score(&winners, &hand))
            .sum::<u32>()
            .to_string();
    }
}
