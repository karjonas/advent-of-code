extern crate common;
extern crate intcode;

use std::collections::VecDeque;

fn run(memory: Vec<i64>, instrs: Vec<&str>) -> usize {
    let (memory_new, _output_numbers, index, relative_base, _halted) = intcode::run(
        memory,
        VecDeque::new(),
        0,
        0,
    );

    let input_str = instrs
        .iter()
        .map(|v| v.to_string() + "\n")
        .collect::<String>();

    let input = intcode::string_to_ascii(input_str.as_str());

    let (_memory_new, output_numbers, _index, _relative_base, _halted) = intcode::run(
        memory_new.clone(),
        VecDeque::from(input),
        index,
        relative_base,
    );

    return *output_numbers.last().unwrap() as usize;
}

fn solve_part_two(memory: Vec<i64>) -> usize {
    let instrs = [
        "NOT C J", //
        "AND D J", //
        "AND H J", //
        "NOT B T", //
        "AND D T", //
        "OR T J",  //
        "NOT A T", //
        "OR T J",  //
        "RUN",     //
    ]
    .to_vec();
    return run(memory, instrs);
}

fn solve_part_one(memory: Vec<i64>) -> usize {
    let instrs = [
        "NOT C J", //
        "AND D J", //
        "NOT A T", //
        "OR T J",  //
        "WALK",    //
    ]
    .to_vec();
    return run(memory, instrs);
}

pub fn solve() {
    let input = common::read_file("2019/day21/input");
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone()));
    println!("Part two: {}", solve_part_two(memory));
}
