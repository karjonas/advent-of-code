#[macro_use]
extern crate bitflags;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

const NUM_FLOORS: usize = 4;
const NUM_TOT: usize = 14;

#[derive(PartialEq, PartialOrd, Clone, Eq, Hash)]
struct SolveState {
    curr_floor: u8,
    rtgs: [RTGFlag; NUM_FLOORS],
}

bitflags! {
    struct RTGFlag: usize {
        const CHIP_A       = 0b00000000000001;
        const CHIP_B       = 0b00000000000010;
        const CHIP_C       = 0b00000000000100;
        const CHIP_D       = 0b00000000001000;
        const CHIP_E       = 0b00000000010000;
        const CHIP_F       = 0b00000000100000;
        const CHIP_G       = 0b00000001000000;
        const GENERATOR_A  = 0b00000010000000;
        const GENERATOR_B  = 0b00000100000000;
        const GENERATOR_C  = 0b00001000000000;
        const GENERATOR_D  = 0b00010000000000;
        const GENERATOR_E  = 0b00100000000000;
        const GENERATOR_F  = 0b01000000000000;
        const GENERATOR_G  = 0b10000000000000;
    }
}

fn all_on_last_floor(rtgs: &[RTGFlag; NUM_FLOORS]) -> bool {
    for i in 0..NUM_FLOORS - 1 {
        if !rtgs[i].is_empty() {
            return false;
        }
    }
    return true;
}

fn floor_ok(rtgs: RTGFlag) -> bool {
    let all_gens = RTGFlag::GENERATOR_A
        | RTGFlag::GENERATOR_B
        | RTGFlag::GENERATOR_C
        | RTGFlag::GENERATOR_D
        | RTGFlag::GENERATOR_E
        | RTGFlag::GENERATOR_F
        | RTGFlag::GENERATOR_G;
    let has_generator = !(rtgs & all_gens).is_empty();

    return !has_generator
        || !((rtgs.contains(RTGFlag::CHIP_A) && !rtgs.contains(RTGFlag::GENERATOR_A))
            || (rtgs.contains(RTGFlag::CHIP_B) && !rtgs.contains(RTGFlag::GENERATOR_B))
            || (rtgs.contains(RTGFlag::CHIP_C) && !rtgs.contains(RTGFlag::GENERATOR_C))
            || (rtgs.contains(RTGFlag::CHIP_D) && !rtgs.contains(RTGFlag::GENERATOR_D))
            || (rtgs.contains(RTGFlag::CHIP_E) && !rtgs.contains(RTGFlag::GENERATOR_E))
            || (rtgs.contains(RTGFlag::CHIP_F) && !rtgs.contains(RTGFlag::GENERATOR_F))
            || (rtgs.contains(RTGFlag::CHIP_G) && !rtgs.contains(RTGFlag::GENERATOR_G)));
}

fn try_move_objects(
    last_floor: u8,
    index: (RTGFlag, RTGFlag),
    new_floor: u8,
    curr_state: &SolveState,
) -> Option<SolveState> {
    let mut left_floor = curr_state.rtgs[last_floor as usize];
    left_floor.remove(index.0);
    left_floor.remove(index.1);

    if !floor_ok(left_floor) {
        return None;
    }

    let mut next_floor = curr_state.rtgs[new_floor as usize];
    next_floor.insert(index.0);
    next_floor.insert(index.1);

    if !floor_ok(next_floor) {
        return None;
    }

    let mut rtgs_next = curr_state.rtgs.clone();
    rtgs_next[new_floor as usize] = next_floor;
    rtgs_next[last_floor as usize] = left_floor;

    return Some(SolveState {
        curr_floor: new_floor as u8,
        rtgs: rtgs_next,
    });
}

fn get_next_states(
    curr_state: &SolveState,
    steps: usize,
    output: &mut VecDeque<(usize, SolveState)>,
) {
    let floor_rtgs = curr_state.rtgs[curr_state.curr_floor as usize];

    for i in 0..NUM_TOT {
        let obj = RTGFlag::from_bits(1 << i).unwrap();
        if floor_rtgs.contains(obj) {
            for j in i..NUM_TOT {
                let obj1 = RTGFlag::from_bits(1 << j).unwrap();
                if floor_rtgs.contains(obj1) {
                    if curr_state.curr_floor < 3 {
                        let state = try_move_objects(
                            curr_state.curr_floor,
                            (obj, obj1),
                            curr_state.curr_floor + 1,
                            &curr_state,
                        );
                        match state {
                            Some(s) => output.push_back((steps, s)),
                            None => (),
                        }
                    }

                    if curr_state.curr_floor > 0 {
                        let state = try_move_objects(
                            curr_state.curr_floor,
                            (obj, obj1),
                            curr_state.curr_floor - 1,
                            &curr_state,
                        );
                        match state {
                            Some(s) => output.push_back((steps, s)),
                            None => (),
                        }
                    }
                }
            }
        }
    }
}

fn solve_brute(rtgs: [RTGFlag; NUM_FLOORS]) -> usize {
    let mut states: VecDeque<(usize, SolveState)> = VecDeque::new();
    let mut visited_states = HashSet::new();

    states.push_back((
        0,
        SolveState {
            curr_floor: 0,
            rtgs: rtgs,
        },
    ));
    let mut num_best = 0;

    loop {
        let (num_moves, best) = states.pop_front().unwrap();
        if visited_states.contains(&best) {
            continue;
        }

        if num_moves != num_best {
            num_best = num_moves;
            // println!("moves {:?}", num_best);
            // println!("state {:?}", states.len());
        }

        if all_on_last_floor(&best.rtgs) {
            return num_moves;
        }
        get_next_states(&best, num_moves + 1, &mut states);

        visited_states.insert(best);
    }
}

fn solve_internal(input_str: &str) -> usize {
    let mut file = File::open(input_str).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let line_words: Vec<Vec<String>> = contents
        .lines()
        .map(|l| {
            l.split(|c| c == ' ' || c == '-' || c == ',' || c == '.')
                .map(|w| w.to_string())
                .collect()
        })
        .collect();

    let mut hm = HashMap::new();
    let mut rtgs = [RTGFlag::empty(); NUM_FLOORS];
    let mut floor = 0;
    for line in line_words {
        for word_idx in 2..line.len() {
            if line[word_idx] == "microchip" {
                let elem = line[word_idx - 2].clone();
                let l = hm.len();
                let idx = hm.entry(elem).or_insert(l).clone();
                assert!(idx < NUM_TOT / 2);
                let obj = RTGFlag::from_bits(1 << (idx)).unwrap();
                rtgs[floor] = rtgs[floor] | obj;
            } else if line[word_idx] == "generator" {
                let elem = line[word_idx - 1].clone();
                let l = hm.len();
                let idx = hm.entry(elem).or_insert(l).clone();
                assert!(idx < NUM_TOT / 2);
                let obj = RTGFlag::from_bits(1 << (idx + NUM_TOT / 2)).unwrap();
                rtgs[floor] = rtgs[floor] | obj
            }
        }
        floor += 1;
    }

    return solve_brute(rtgs);
}

pub fn solve() {
    println!("Part 1: {:?}", solve_internal("2016/day11/input"));
    println!("Part 2: {:?}", solve_internal("2016/day11/input2"));
}
