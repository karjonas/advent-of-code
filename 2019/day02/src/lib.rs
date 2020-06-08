extern crate common;
extern crate intcode;

const GOAL: i64 = 19690720;

fn run_input(input_str: &str, noun: i64, verb: i64) -> i64 {
    let mut memory = intcode::parse_input(input_str);
    memory[1] = noun;
    memory[2] = verb;
    let (memory_new, _output_numbers, _index, _relative_base, _halted) =
        intcode::run(memory, Vec::new(), 0, 0);
    return memory_new[0];
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
