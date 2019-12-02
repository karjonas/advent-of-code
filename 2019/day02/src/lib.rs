extern crate common;

const OPADD: i64 = 1;
const OPMUL: i64 = 2;
const OPHALT: i64 = 99;

const GOAL: i64 = 19690720;

fn run_input(input_str: &str, noun: i64, verb: i64) -> i64 {
    let mut input: Vec<i64> = input_str
        .split(',')
        .map(|v| common::string_to_i64(v))
        .collect::<Vec<_>>();
    input[1] = noun;
    input[2] = verb;
    let output = run_input_vec(input);
    return output[0];
}

fn run_input_vec(mut input: Vec<i64>) -> Vec<i64> {
    let size = input.len();
    let mut index = 0;

    while index < size {
        let op = input[index];
        if op == OPADD {
            let i0 = input[index + 1];
            let i1 = input[index + 2];
            let dest = input[index + 3];
            let value = input[i0 as usize] + input[i1 as usize];
            input[dest as usize] = value;
            index += 4;
        } else if op == OPMUL {
            let i0 = input[index + 1];
            let i1 = input[index + 2];
            let dest = input[index + 3];
            let value = input[i0 as usize] * input[i1 as usize];
            input[dest as usize] = value;
            index += 4;
        } else if op == OPHALT {
            break;
        } else {
            panic!("Invalid State");
        }
    }

    return input;
}

fn part_two(input_str: &str) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run_input(input_str, noun, verb);
            if result == GOAL {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found");
}

pub fn solve() {
    let input = common::read_file("2019/day02/input");
    println!("Part one: {}", run_input(input.as_str(), 12, 2));
    println!("Part two: {}", part_two(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(
            run_input_vec([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec()),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].to_vec()
        );
        assert_eq!(
            run_input_vec([1, 0, 0, 0, 99].to_vec()),
            [2, 0, 0, 0, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 3, 0, 3, 99].to_vec()),
            [2, 3, 0, 6, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 4, 4, 5, 99, 0].to_vec()),
            [2, 4, 4, 5, 99, 9801].to_vec()
        );
        assert_eq!(
            run_input_vec([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec()),
            [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec()
        );
    }
}
