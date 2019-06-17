use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct State {
    if_one_write: usize,
    if_zero_write: usize,

    if_one_move: i32,
    if_zero_move: i32,

    if_one_goto_state: char,
    if_zero_goto_state: char,
}

fn parse_input(start_state: &mut char, diagnostic_after: &mut usize) -> HashMap<char, State> {
    let mut file = File::open("2017/day25/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.chars().filter(|&v| v != '.' && v != ':').collect();

    let lines: Vec<Vec<_>> = contents
        .trim()
        .split('\n')
        .map(|l| l.split(' ').collect())
        .collect();

    *start_state = lines[0][3].chars().nth(0).unwrap();
    *diagnostic_after = lines[1][5].parse::<usize>().unwrap();

    let mut states = HashMap::new();

    let mut c_idx = 3;
    loop {
        if c_idx >= lines.len() {
            break;
        }

        let name = lines[c_idx][2].chars().nth(0).unwrap();
        let if_zero_write = lines[c_idx + 2][8].parse::<usize>().unwrap();
        let if_zero_move = if lines[c_idx + 3][10] == "left" {
            -1
        } else {
            1
        };
        let if_zero_goto_state = lines[c_idx + 4][8].chars().nth(0).unwrap();

        let if_one_write = lines[c_idx + 6][8].parse::<usize>().unwrap();
        let if_one_move = if lines[c_idx + 7][10] == "left" {
            -1
        } else {
            1
        };
        let if_one_goto_state = lines[c_idx + 8][8].chars().nth(0).unwrap();

        c_idx += 10;

        let state = State {
            if_one_write: if_one_write,
            if_zero_write: if_zero_write,

            if_one_move: if_one_move,
            if_zero_move: if_zero_move,

            if_one_goto_state: if_one_goto_state,
            if_zero_goto_state: if_zero_goto_state,
        };

        states.insert(name, state);
    }

    return states;
}

pub fn solve() {
    let mut start_state = 'A';
    let mut diagnostic_after = 0;
    let states = parse_input(&mut start_state, &mut diagnostic_after);
    let mut tape = HashMap::<i32, usize>::new();

    let mut curr_state = start_state;
    let mut tape_pos = 0;
    for _ in 0..diagnostic_after {
        let v = tape.entry(tape_pos).or_insert(0).clone();
        let state = states.get(&curr_state).unwrap();

        if v == 0 {
            tape.insert(tape_pos, state.if_zero_write);
            tape_pos += state.if_zero_move;
            curr_state = state.if_zero_goto_state;
        } else {
            tape.insert(tape_pos, state.if_one_write);
            tape_pos += state.if_one_move;
            curr_state = state.if_one_goto_state;
        }
    }

    let mut checksum = 0;
    for (_, val) in &tape {
        checksum += val;
    }

    println!("Part one: {:?}", checksum);
}
