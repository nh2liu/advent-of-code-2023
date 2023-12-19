use crate::utils::Solution;
use itertools::Itertools;

pub struct Day18_1;
pub struct Day18_2;

fn parse_instr(lines: &[String]) -> Vec<(char, isize, &str)> {
    lines
        .iter()
        .map(|line| {
            let (d, l, h) = line
                .split(' ')
                .collect_tuple::<(&str, &str, &str)>()
                .unwrap();
            (d.chars().next().unwrap(), l.parse::<isize>().unwrap(), h)
        })
        .collect_vec()
}

fn shoelace(direction: Vec<(char, isize)>) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut area = 0;
    let mut circumference = 0;
    for (dir, mag) in direction {
        let (x2, y2) = match dir {
            'R' => (x + mag, y),
            'L' => (x - mag, y),
            'U' => (x, y + mag),
            'D' => (x, y - mag),
            _ => panic!("{} not a dir", dir),
        };
        circumference += mag;
        area += (y + y2) * (x - x2);
        (x, y) = (x2, y2);
    }
    let total_area = (area.abs() + circumference) / 2 + 1;
    println!("{} {}", area / 2, circumference);
    total_area as usize
}

impl Solution for Day18_1 {
    fn name(&self) -> &str {
        "day18_lavaduct_lagoon"
    }
    fn solve(&self, lines: &[String]) -> String {
        let instrs = parse_instr(lines);
        shoelace(
            instrs
                .into_iter()
                .map(|(dir, mag, _)| (dir, mag))
                .collect_vec(),
        )
        .to_string()
    }
}

impl Solution for Day18_2 {
    fn name(&self) -> &str {
        "day18_lavaduct_lagoon"
    }
    fn solve(&self, lines: &[String]) -> String {
        let instrs = parse_instr(lines);
        shoelace(
            instrs
                .into_iter()
                .map(|(_, _, hex)| {
                    let dir = match hex.chars().nth(hex.len() - 2).unwrap() {
                        '0' => 'R',
                        '1' => 'D',
                        '2' => 'L',
                        '3' => 'U',
                        e => panic!("not valid dir {e}"),
                    };
                    let mag = isize::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
                    (dir, mag)
                })
                .collect_vec(),
        )
        .to_string()
    }
}
