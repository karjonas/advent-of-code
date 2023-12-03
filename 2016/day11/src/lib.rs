use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const NUM_FLOORS: usize = 4;
const NUM_TOT: usize = 14;

#[derive(PartialEq, PartialOrd, Clone, Eq, Hash)]
struct SolveState {
    curr_floor: u8,
    rtgs: [usize; NUM_FLOORS],
}

const CHIP_A: usize = 0b00000000000001;
const CHIP_B: usize = 0b00000000000010;
const CHIP_C: usize = 0b00000000000100;
const CHIP_D: usize = 0b00000000001000;
const CHIP_E: usize = 0b00000000010000;
const CHIP_F: usize = 0b00000000100000;
const CHIP_G: usize = 0b00000001000000;
const GENERATOR_A: usize = 0b00000010000000;
const GENERATOR_B: usize = 0b00000100000000;
const GENERATOR_C: usize = 0b00001000000000;
const GENERATOR_D: usize = 0b00010000000000;
const GENERATOR_E: usize = 0b00100000000000;
const GENERATOR_F: usize = 0b01000000000000;
const GENERATOR_G: usize = 0b10000000000000;

fn is_bit_set(rtg: usize, flag: usize) -> bool {
    return (rtg & flag) > 0;
}

fn unset_bit(rtg: usize, flag: usize) -> usize {
    return rtg & !flag;
}

fn set_bit(rtg: usize, flag: usize) -> usize {
    return rtg | flag;
}

fn all_on_last_floor(rtgs: &[usize; NUM_FLOORS]) -> bool {
    for i in 0..NUM_FLOORS - 1 {
        if rtgs[i] > 0 {
            return false;
        }
    }
    return true;
}

fn floor_ok(rtgs: usize) -> bool {
    let all_gens = GENERATOR_A
        | GENERATOR_B
        | GENERATOR_C
        | GENERATOR_D
        | GENERATOR_E
        | GENERATOR_F
        | GENERATOR_G;
    let has_generator = (rtgs & all_gens) > 0;

    return !has_generator
        || !((is_bit_set(rtgs, CHIP_A) && !is_bit_set(rtgs, GENERATOR_A))
            || (is_bit_set(rtgs, CHIP_B) && !is_bit_set(rtgs, GENERATOR_B))
            || (is_bit_set(rtgs, CHIP_C) && !is_bit_set(rtgs, GENERATOR_C))
            || (is_bit_set(rtgs, CHIP_D) && !is_bit_set(rtgs, GENERATOR_D))
            || (is_bit_set(rtgs, CHIP_E) && !is_bit_set(rtgs, GENERATOR_E))
            || (is_bit_set(rtgs, CHIP_F) && !is_bit_set(rtgs, GENERATOR_F))
            || (is_bit_set(rtgs, CHIP_G) && !is_bit_set(rtgs, GENERATOR_G)));
}

fn try_move_objects(
    last_floor: u8,
    index: (usize, usize),
    new_floor: u8,
    curr_state: &SolveState,
) -> Option<SolveState> {
    let mut left_floor = curr_state.rtgs[last_floor as usize];
    left_floor = unset_bit(left_floor, index.0);
    left_floor = unset_bit(left_floor, index.1);

    if !floor_ok(left_floor) {
        return None;
    }

    let mut next_floor = curr_state.rtgs[new_floor as usize];
    next_floor = set_bit(next_floor, index.0);
    next_floor = set_bit(next_floor, index.1);

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
        let obj = 1 << i;
        if is_bit_set(floor_rtgs, obj) {
            for j in i..NUM_TOT {
                let obj1 = 1 << j;
                if is_bit_set(floor_rtgs, obj1) {
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

fn solve_brute(rtgs: [usize; NUM_FLOORS]) -> usize {
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

fn solve_internal(input: &str, part_two: bool) -> usize {
    let mut line_words: Vec<Vec<String>> = input
        .lines()
        .map(|l| {
            l.split(|c| c == ' ' || c == '-' || c == ',' || c == '.')
                .map(|w| w.to_string())
                .collect()
        })
        .collect();

    if part_two {
        let words = [
            "An",
            "elerium",
            "generator",
            "An",
            "elerium",
            "compatible",
            "microchip",
            "A",
            "dilithium",
            "generator",
            "A",
            "dilithium",
            "compatible",
            "microchip",
        ];

        for word in words {
            line_words[0].push(word.to_string());
        }
    }

    let mut hm = HashMap::new();
    let mut rtgs = [0; NUM_FLOORS];
    let mut floor = 0;
    for line in line_words {
        for word_idx in 2..line.len() {
            if line[word_idx] == "microchip" {
                let elem = line[word_idx - 2].clone();
                let l = hm.len();
                let idx = hm.entry(elem).or_insert(l).clone();
                assert!(idx < NUM_TOT / 2);
                let obj = 1 << idx;
                rtgs[floor] = rtgs[floor] | obj;
            } else if line[word_idx] == "generator" {
                let elem = line[word_idx - 1].clone();
                let l = hm.len();
                let idx = hm.entry(elem).or_insert(l).clone();
                assert!(idx < NUM_TOT / 2);
                let obj = 1 << (idx + NUM_TOT / 2);
                rtgs[floor] = rtgs[floor] | obj
            }
        }
        floor += 1;
    }

    return solve_brute(rtgs);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part 1: {:?}", solve_internal(input.as_str(), false));
    println!("Part 2: {:?}", solve_internal(input.as_str(), true));
}
