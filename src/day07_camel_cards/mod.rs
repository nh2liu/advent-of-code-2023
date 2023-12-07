use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::Solution;

pub struct Day07_1;
pub struct Day07_2;

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
    let n_distinct = char_frequency.len();
    match (n_distinct, max_frequency) {
        (5, _) => 1, // Single
        (4, _) => 2, // Pair
        (3, 2) => 3, // Two Pair
        (3, 3) => 4, // Triple,
        (2, 3) => 5, // House,
        (2, 4) => 6, // Quads,
        (1, 5) => 7, // Fives
        _ => panic!("{:?} cards not valid.", cards),
    }
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

fn get_card_value2(c: char) -> u32 {
    let v = get_card_value(c);
    if v == 11 {
        return 1;
    }
    v
}

fn convert_cards2(c: &[char]) -> Cards {
    let v: Vec<u32> = c.iter().cloned().map(get_card_value2).collect();
    (v[0], v[1], v[2], v[3], v[4])
}

fn rank2(cards: Cards) -> u32 {
    //  Returns rank of the cards
    let card_vec = vec![cards.0, cards.1, cards.2, cards.3, cards.4];
    let mut char_frequency = HashMap::new();
    let mut jacks = 0;
    for c in card_vec {
        if c == 1 {
            jacks += 1;
        } else {
            let counter = char_frequency.entry(c).or_insert(0);
            *counter += 1;
        }
    }
    let max_frequency = *char_frequency.values().max().unwrap_or(&0);
    match (jacks, max_frequency) {
        (5, _) | (4, _) => 7,
        (3, 2) => 7,
        (3, 1) => 6,
        (2, 3) => 7,
        (2, 2) => 6,
        (2, 1) => 4,
        (1, 4) => 7,
        (1, 3) => 6,
        (1, 2) => {
            if char_frequency.len() == 3 {
                4
            } else {
                5
            }
        }
        (1, 1) => 2,
        (0, _) => rank(cards),
        _ => panic!("Invalid combo: {:?}", cards),
    }
}

impl Solution for Day07_2 {
    fn name(&self) -> &str {
        "day07_camel_cards"
    }
    fn solve(&self, lines: &[String]) -> String {
        let mut cards_bids = Vec::new();

        for (card, bid) in lines.iter().map(|line| line.split_once(' ').unwrap()) {
            let cards = convert_cards2(&card.chars().collect::<Vec<char>>());
            let bid = bid.parse::<u32>().unwrap();
            cards_bids.push((cards, bid));
        }
        let sorted_cards: Vec<&(Cards, u32)> = cards_bids
            .iter()
            .sorted_by(|(c1, _), (c2, _)| (rank2(*c2), c2).cmp(&(rank2(*c1), c1)))
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
