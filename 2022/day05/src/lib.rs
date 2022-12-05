use std::collections::VecDeque;
extern crate common;
#[macro_use]
extern crate scan_fmt;

fn parse_stacks(input: &String) -> (Vec<VecDeque<char>>, usize) {
    let chars: Vec<VecDeque<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let num_cols = 1 + chars[0].len() / 4;
    let mut stacks = vec![VecDeque::<char>::new(); num_cols];

    for row in 0.. {
        for col in 0..num_cols {
            let c = chars[row][col * 4 + 1];
            if c == '1' {
                return (stacks, row);
            } else if c != ' ' {
                stacks[col].push_front(c);
            }
        }
    }

    return (Vec::new(), 0);
}

fn solve_internal(input: &String, part_one: bool) -> String {
    let (mut stacks, num_rows) = parse_stacks(input);
    let mut row = 0;
    for line in input.lines() {
        row += 1;
        if row < num_rows + 3 {
            continue;
        }

        let (num_move, from, to) =
            scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
        let mut temp_stack = VecDeque::new();

        for _ in 0..num_move {
            let cargo = stacks[from - 1].pop_back().unwrap();
            temp_stack.push_back(cargo);
        }

        for _ in 0..num_move {
            let cargo = if part_one {
                temp_stack.pop_front().unwrap()
            } else {
                temp_stack.pop_back().unwrap()
            };
            stacks[to - 1].push_back(cargo);
        }
    }

    let mut result = String::new();
    for stack in stacks {
        result.push(*stack.back().unwrap());
    }

    return result;
}

pub fn solve() {
    let input = common::read_file("2022/day05/input");
    println!("Part one: {}", solve_internal(&input, true));
    println!("Part two: {}", solve_internal(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        let result = solve_internal(&input, true);
        assert_eq!(result, "CMZ");
        let result = solve_internal(&input, false);
        assert_eq!(result, "MCD");
    }
}
