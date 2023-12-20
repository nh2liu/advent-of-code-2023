use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::Solution;

pub struct Day19_1;
pub struct Day19_2;

type Conditions<'a> = (Vec<(char, char, usize, &'a str)>, &'a str);

fn parse_instr(line: &str) -> (&str, Conditions) {
    let (name, instr_str) = line.split_once('{').unwrap();
    let if_chain = instr_str[..instr_str.len() - 1].split(',').collect_vec();
    let else_goto = if_chain[if_chain.len() - 1];
    let conditions = if_chain[..if_chain.len() - 1]
        .iter()
        .map(|x| {
            let (cond, goto) = x.split_once(':').unwrap();
            (
                cond.chars().next().unwrap(),
                cond.chars().nth(1).unwrap(),
                cond[2..].parse::<usize>().unwrap(),
                goto,
            )
        })
        .collect_vec();
    (name, (conditions, else_goto))
}

fn parse_struct(line: &str) -> HashMap<char, usize> {
    let mut ret = HashMap::new();
    line[1..line.len() - 1].split(',').for_each(|x| {
        let (key, value) = x.split_once('=').unwrap();
        ret.insert(key.chars().next().unwrap(), value.parse::<usize>().unwrap());
    });
    ret
}

fn resolve(s: &HashMap<char, usize>, conditions: &HashMap<&str, Conditions>) -> bool {
    let mut cur_state = "in";
    loop {
        let (ifs, else_goto) = &conditions[cur_state];
        let mut goto_state = None;
        for &(var, cmp, cmp_val, goto) in ifs {
            let val = s[&var];
            let b = match cmp {
                '>' => val > cmp_val,
                '<' => val < cmp_val,
                _ => panic!("Not valid {cmp}"),
            };
            if b {
                goto_state = Some(goto);
                break;
            }
        }
        let goto_state = goto_state.unwrap_or(else_goto);
        if goto_state == "A" {
            return true;
        } else if goto_state == "R" {
            return false;
        }
        cur_state = goto_state;
    }
}

impl Solution for Day19_1 {
    fn name(&self) -> &str {
        "day19_aplenty"
    }
    fn solve(&self, lines: &[String]) -> String {
        let string_block = lines.join("\n");
        let (instructions_str, structs_str) = string_block.split_once("\n\n").unwrap();
        let instructions: HashMap<&str, Conditions> =
            instructions_str.split('\n').map(parse_instr).collect();
        let structs = structs_str.split('\n').map(parse_struct).collect_vec();
        let mut ans = 0;
        for s in structs {
            let b = resolve(&s, &instructions);
            println!("{s:?} -> {b}");
            if b {
                ans += s.values().sum::<usize>();
            }
        }
        ans.to_string()
    }
}
#[derive(Debug)]
struct XMASRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl XMASRange {
    pub fn new() -> XMASRange {
        Self {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.x.1 > self.x.0 && self.m.1 > self.m.0 && self.a.1 > self.a.0 && self.s.1 > self.s.0
    }

    pub fn n_combos(&self) -> usize {
        let diff = |(a, b)| b - a;

        diff(self.x) * diff(self.m) * diff(self.a) * diff(self.s)
    }

    pub fn add_bound(&self, c: char, op: char, bound: usize) -> XMASRange {
        let clip = |(a, b)| match op {
            '>' => (std::cmp::max(a, bound + 1), b),
            '<' => (a, std::cmp::min(b, bound)),
            _ => panic!("Operator {op}"),
        };
        XMASRange {
            x: if c == 'x' { clip(self.x) } else { self.x },
            m: if c == 'm' { clip(self.m) } else { self.m },
            a: if c == 'a' { clip(self.a) } else { self.a },
            s: if c == 's' { clip(self.s) } else { self.s },
        }
    }

    pub fn complement(&mut self, c: char, op: char, bound: usize) {
        let clip = |(a, b)| match op {
            '>' => (a, std::cmp::min(b, bound + 1)),
            '<' => (std::cmp::max(a, bound), b),
            _ => panic!("Operator {op}"),
        };
        match c {
            'x' => self.x = clip(self.x),
            'm' => self.m = clip(self.m),
            'a' => self.a = clip(self.a),
            's' => self.s = clip(self.s),
            _ => panic!("Not valid"),
        }
    }
}

impl Solution for Day19_2 {
    fn name(&self) -> &str {
        "day19_aplenty"
    }
    fn solve(&self, lines: &[String]) -> String {
        let string_block = lines.join("\n");
        let (instructions_str, _) = string_block.split_once("\n\n").unwrap();
        let instructions: HashMap<&str, Conditions> =
            instructions_str.split('\n').map(parse_instr).collect();
        let mut stack = vec![(XMASRange::new(), "in")];
        let mut accepted = vec![];
        while let Some((mut xrange, cur_state)) = stack.pop() {
            if !xrange.is_valid() {
                continue;
            }
            match cur_state {
                "R" => {} // No-op
                "A" => accepted.push(xrange),
                _ => {
                    let (ifs, else_goto) = &instructions[cur_state];
                    for &(var, cmp, cmp_val, goto) in ifs {
                        stack.push((xrange.add_bound(var, cmp, cmp_val), goto));
                        xrange.complement(var, cmp, cmp_val);
                    }
                    stack.push((xrange, else_goto));
                }
            }
        }
        let mut n_combos = 0;
        for xmas_range in accepted {
            n_combos += xmas_range.n_combos();
            println!("{xmas_range:?} - {}", xmas_range.n_combos());
        }
        n_combos.to_string()
    }
}
