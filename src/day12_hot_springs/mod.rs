use crate::utils::Solution;
use itertools::Itertools;
pub struct Day12_1;

fn find_ways(condition: &[char], records: &[usize]) -> usize {
    fn helper(condition: &[char], records: &[usize]) -> usize {
        // Ensuring we begin at a # or ?
        let mut i = 0;
        for &c in condition.iter() {
            if c != '.' {
                break;
            }
            i += 1;
        }

        if i > 0 {
            return helper(&condition[i..], records);
        }
        println!("{condition:?} {records:?}");
        if (condition.len() == 0 && records.len() == 0)
            || (records.len() == 0 && condition.iter().all(|&x| (x == '?' || x == '.')))
        {
            // 1 way.
            return 1;
        } else if condition.len() == 0 || records.len() == 0 {
            // invalid
            return 0;
        }
        let cur = records[0];
        let mut n_ways = 0;

        if condition.len() >= cur {
            // Consume char starting at this point.
            let candidate = condition[0..cur].iter().all(|&x| x == '#' || x == '?');
            let end_condition: bool =
                condition.len() == cur || condition[cur] == '.' || condition[cur] == '?';
            if candidate && end_condition {
                n_ways += helper(
                    &condition[(cur + (condition.len() != cur) as usize)..],
                    &records[1..],
                );
            }
        }
        if condition[0] == '?' {
            n_ways += helper(&condition[1..], records);
        }
        n_ways
    }

    helper(condition, records)
}

impl Solution for Day12_1 {
    fn name(&self) -> &str {
        "day12_hot_springs"
    }
    fn solve(&self, lines: &[String]) -> String {
        let springs = lines
            .iter()
            .map(|line| {
                let (cstr, rstr) = line.split_once(' ').unwrap();
                let records = rstr
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();
                let conditions = cstr.chars().collect_vec();
                (conditions, records)
            })
            .collect_vec();

        let mut ans = 0;
        for (condition, records) in springs.into_iter() {
            let n_ways = find_ways(&condition, &records);
            println!(
                "{} {records:?} | {n_ways}",
                condition.iter().collect::<String>()
            );
            ans += n_ways;
        }
        ans.to_string()
    }
}
