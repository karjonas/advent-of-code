extern crate common;
extern crate intcode;

use std::collections::VecDeque;

fn sum_alignment(input: String) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|value| value.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut sum = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut intersection = true;
            for c in [
                lines[y][x],
                lines[y][x - 1],
                lines[y][x + 1],
                lines[y - 1][x],
                lines[y + 1][x],
            ]
            .iter()
            {
                if !['#', '^', 'v', '<', '>'].contains(c) {
                    intersection = false;
                    break;
                }
            }

            if intersection {
                sum += y * x;
            }
        }
    }

    return sum;
}

fn solve_part_one(memory: Vec<i64>) -> usize {
    let (memory_new, output_numbers, index, relative_base, _halted) =
        intcode::run(memory, VecDeque::new(), 0, 0);
    let mut s = String::new();
    for number in output_numbers {
        s.push(number as u8 as char);
    }
    println!("{}", s);
    return sum_alignment(s.trim().to_string());
}

pub fn solve() {
    let input = common::read_file("2019/day17/input");
    let memory = intcode::parse_input(input.as_str());
    println!("Part one: {}", solve_part_one(memory));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_a() {
        let input = "..#..........\n..#..........\n#######...###\n#.#...#...#.#\n#############\n..#...#...#..\n..#####...^..";
        assert_eq!(sum_alignment(String::from(input)), 76);
    }
}
