use std::collections::HashMap;

use crate::utils::Solution;
use itertools::Itertools;
pub struct Day12_1;
pub struct Day12_2;

fn find_ways(condition: &[char], records: &[usize]) -> usize {
    fn helper(
        condition: &[char],
        records: &[usize],
        m: usize,
        n: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if cache.contains_key(&(m, n)) {
            return *cache.get(&(m, n)).unwrap();
        }

        // Ensuring we begin at a # or ?
        let mut i = 0;
        for &c in condition[m..].iter() {
            if c != '.' {
                break;
            }
            i += 1;
        }

        if i > 0 {
            let val = helper(condition, records, m + i, n, cache);
            cache.insert((m, n), val);
            return val;
        }

        if (condition.len() == m && records.len() == n)
            || (records.len() == n && condition[m..].iter().all(|&x| (x == '?' || x == '.')))
        {
            // 1 way.
            cache.insert((m, n), 1);
            return 1;
        } else if condition.len() == m || records.len() == n {
            // invalid
            cache.insert((m, n), 0);
            return 0;
        }
        let cur = records[n];
        let mut n_ways = 0;

        if condition.len() - m >= cur {
            // Consume char starting at this point.
            let candidate = condition[m..m + cur].iter().all(|&x| x == '#' || x == '?');
            let end_condition: bool = condition.len() - m == cur
                || condition[m + cur] == '.'
                || condition[m + cur] == '?';
            if candidate && end_condition {
                n_ways += helper(
                    condition,
                    records,
                    m + cur + (condition.len() - m != cur) as usize,
                    n + 1,
                    cache,
                );
            }
        }
        if condition[m] == '?' {
            n_ways += helper(condition, records, m + 1, n, cache);
        }
        cache.insert((m, n), n_ways);
        n_ways
    }
    let mut cache = HashMap::new();
    helper(condition, records, 0, 0, &mut cache);
    *cache.get(&(0, 0)).unwrap()
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

impl Solution for Day12_2 {
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
                    .collect_vec()
                    .repeat(5);
                let mut conditions = cstr.chars().collect_vec();
                conditions.push('?');
                conditions = conditions.repeat(5);
                conditions.pop();
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
