use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::Solution;

pub struct Day19_1;

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
