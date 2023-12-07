use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::Solution;

pub struct Day07_1;

type Cards = (u32, u32, u32, u32, u32);

fn get_card_value(c: char) -> u32 {
    if c.is_ascii_digit() {
        return c.to_digit(10).unwrap();
    } else if c == 'T' {
        return 10;
    } else if c == 'J' {
        return 11;
    } else if c == 'Q' {
        return 12;
    } else if c == 'K' {
        return 13;
    } else if c == 'A' {
        return 14;
    }
    panic!("Not a valid card: {}", c);
}

fn convert_cards(c: &[char]) -> Cards {
    let v: Vec<u32> = c.iter().cloned().map(get_card_value).collect();
    (v[0], v[1], v[2], v[3], v[4])
}

fn rank(cards: Cards) -> u32 {
    //  Returns rank of the cards
    let card_vec = vec![cards.0, cards.1, cards.2, cards.3, cards.4];
    let mut char_frequency = HashMap::new();
    for c in card_vec {
        let counter = char_frequency.entry(c).or_insert(0);
        *counter += 1;
    }
    let max_frequency = *char_frequency.values().max().unwrap();
    if char_frequency.len() == 5 {
        // Single
        return 1;
    } else if char_frequency.len() == 4 {
        // Pair
        return 2;
    } else if char_frequency.len() == 3 {
        if max_frequency == 2 {
            // Two Pair
            return 3;
        } else if max_frequency == 3 {
            // Triple
            return 4;
        }
    } else if char_frequency.len() == 2 {
        if max_frequency == 3 {
            // House
            return 5;
        } else if max_frequency == 4 {
            // Quads
            return 6;
        }
    } else if char_frequency.len() == 1 {
        // Five of a kind.
        return 7;
    }
    panic!("{:?} cards not valid.", cards);
}

impl Solution for Day07_1 {
    fn name(&self) -> &str {
        "day07_camel_cards"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut cards_bids = Vec::new();

        for (card, bid) in lines.iter().map(|line| line.split_once(' ').unwrap()) {
            let cards = convert_cards(&card.chars().collect::<Vec<char>>());
            let bid = bid.parse::<u32>().unwrap();
            cards_bids.push((cards, bid));
        }
        let sorted_cards: Vec<&(Cards, u32)> = cards_bids
            .iter()
            .sorted_by(|(c1, _), (c2, _)| (rank(*c2), c2).cmp(&(rank(*c1), c1)))
            .collect();
        println!("{:?}", sorted_cards);
        let ans = sorted_cards
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| bid * (cards_bids.len() - rank) as u32)
            .sum::<u32>();
        ans.to_string()
    }
}
