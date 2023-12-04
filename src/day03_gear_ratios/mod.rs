use crate::utils::Solution;

pub struct Day03_1;

fn is_adjacent_to_symbol(m: i32, n: i32, grid: &Vec<Vec<char>>) -> bool {
    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    return deltas.iter().any(|(dy, dx)| -> bool {
        let (y, x) = ((m + dy), (n + dx));
        grid.len() as i32 > y
            && y >= 0
            && grid[y as usize].len() as i32 > x
            && x >= 0
            && ({
                let adjc = grid[y as usize][x as usize];
                !(adjc.is_digit(10) || adjc == '.')
            })
    });
}

fn parse_num_at(m: usize, n: usize, grid: &Vec<Vec<char>>) -> (usize, u32) {
    let mut n_digits = 0;
    let mut num = 0;
    for i in n..grid[0].len() {
        let c = grid[m][i];
        if !c.is_digit(10) {
            break;
        }
        num = num * 10 + c.to_digit(10).unwrap();
        n_digits += 1;
    }
    return (n_digits as usize, num);
}

fn solve_grid(grid: &Vec<Vec<char>>) -> u32 {
    let mut running_sum = 0;

    for (m, row) in grid.iter().enumerate() {
        let mut current_start_number_index = None;
        let mut last_parsed_idx: i32 = -1;
        for (n, c) in row.iter().enumerate() {
            if n as i32 <= last_parsed_idx {
                continue;
            }
            if c.is_digit(10) {
                if current_start_number_index.is_none() {
                    current_start_number_index = Some(n);
                }
                if is_adjacent_to_symbol(m as i32, n as i32, grid) {
                    let sidx = current_start_number_index.unwrap();
                    let (n_digits, sum) = parse_num_at(m, sidx, grid);
                    last_parsed_idx = (sidx + n_digits - 1) as i32;
                    running_sum += sum;
                }
            } else {
                current_start_number_index = None;
            }
        }
    }
    return running_sum;
}

impl Solution for Day03_1 {
    fn name(&self) -> &str {
        "day03_gear_ratios"
    }

    fn solve(&self, lines: &Vec<String>) -> String {
        return solve_grid(
            &lines
                .iter()
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect(),
        )
        .to_string();
    }
}
