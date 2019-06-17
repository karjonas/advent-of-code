extern crate common;

use std::collections::HashMap;

const NUM_EXPAND: usize = 100;

#[derive(Debug)]
struct Rule {
    input: [char; 5],
    result: char,
}

fn calc_value(state: &Vec<char>) -> i64 {
    let mut ctr = 0;
    for i in 0..state.len() {
        let c = state[i];
        let v = i as i64 - NUM_EXPAND as i64;
        ctr += if c == '#' { v } else { 0 }
    }
    return ctr;
}

fn do_step(state: &mut Vec<char>, rules: &Vec<Rule>) {
    let state_prev = state.clone();
    for i in 0..state.len() {
        state[i] = '.'
    }

    for i in 2..state.len() - 2 {
        for rule in rules {
            if rule.input[0] == state_prev[i - 2]
                && rule.input[1] == state_prev[i - 1]
                && rule.input[2] == state_prev[i]
                && rule.input[3] == state_prev[i + 1]
                && rule.input[4] == state_prev[i + 2]
            {
                state[i] = rule.result;
            }
        }
    }
}

fn parse_input(input_path: &str) -> (Vec<char>, Vec<Rule>) {
    let input = common::read_file(input_path);
    let lines = input
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let mut state = common::filled_vector(NUM_EXPAND, '.');
    {
        let mut state_input = lines[0].split_whitespace().collect::<Vec<_>>()[2]
            .chars()
            .collect::<Vec<char>>();
        state.append(&mut state_input);
        state.append(&mut common::filled_vector(NUM_EXPAND, '.'));
    }

    let mut rules = Vec::new();
    for i in 2..lines.len() {
        let strs = lines[i].split_whitespace().collect::<Vec<_>>();
        let c = strs[0].chars().collect::<Vec<_>>();
        let result = strs[2].chars().collect::<Vec<_>>()[0];
        let rule = Rule {
            input: [c[0], c[1], c[2], c[3], c[4]],
            result: result,
        };
        rules.push(rule);
    }

    return (state, rules);
}

fn part_one(state: &mut Vec<char>, rules: &Vec<Rule>) -> String {
    for _i in 0..20 {
        do_step(state, &rules);
    }
    return calc_value(state).to_string();
}

fn part_two(state: &mut Vec<char>, rules: &Vec<Rule>) -> String {
    // Find repeating index
    let mut visited_string: HashMap<String, (usize, usize)> = HashMap::new();
    let mut last_val = 0;
    let mut i = 0;

    loop {
        do_step(state, &rules);

        let val = calc_value(&state);
        let debug_str_untrim = state.iter().collect::<String>();
        let debug_str = debug_str_untrim
            .trim_start_matches('.')
            .trim_end_matches('.')
            .to_string();
        let first_pot = common::first_index_of(&debug_str_untrim, '#');

        if visited_string.contains_key(&debug_str) {
            let result = (50000000000 - i - 1) * (val - last_val) + val;
            return result.to_string();
        }
        last_val = val;
        visited_string.insert(debug_str, (i as usize, first_pot));
        i += 1;
    }
}

pub fn solve() {
    let (mut state, rules) = parse_input("2018/day12/input");
    println!("Part one: {}", part_one(&mut state.clone(), &rules));
    println!("Part two: {}", part_two(&mut state, &rules));
}
