use crate::utils::Solution;

use std::collections::HashSet;

pub struct Day04_1;
pub struct Day04_2;

fn score(winners: &HashSet<u32>, hand: &HashSet<u32>) -> usize {
    winners.intersection(hand).collect::<Vec<_>>().len()
}

fn parse_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let card = line.split_once(':').unwrap().1;
    let (winners_string, hand_string) = card.split_once('|').unwrap();
    return (
        winners_string
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect(),
        hand_string
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect(),
    );
}

impl Solution for Day04_1 {
    fn name(&self) -> &str {
        "day04_scratchcards"
    }
    fn solve(&self, lines: &[String]) -> String {
        return lines
            .iter()
            .map(|s| parse_line(s.as_str()))
            .map(|(winners, hand)| {
                let n_winners = score(&winners, &hand);
                if n_winners == 0 {
                    0
                } else {
                    2u32.pow((n_winners - 1) as u32)
                }
            })
            .sum::<u32>()
            .to_string();
    }
}

impl Solution for Day04_2 {
    fn name(&self) -> &str {
        "day04_scratchcards"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut cards = vec![1; lines.len()];
        for (i, line) in lines.iter().enumerate() {
            let (winner, hand) = parse_line(line);
            let n_winners = score(&winner, &hand);
            for j in 1..=n_winners {
                cards[i + j] += cards[i];
            }
        }
        cards.iter().sum::<usize>().to_string()
    }
}
