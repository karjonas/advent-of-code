extern crate common;
extern crate intcode;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (i64, i64);
const EMPTY: usize = 1;
const WALL: usize = 0;
const OXYGEN: usize = 2;

type State = (usize, usize); // type, num steps

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

struct ProgramState {
    position: Point,
    steps: usize,
    memory: Vec<i64>,
    index: usize,
    relative_base: i64,
    result: usize,
}

fn build_map(memory: Vec<i64>) -> HashMap<Point, State> {
    let mut visited: HashMap<Point, State> = HashMap::new();
    let mut stack: VecDeque<ProgramState> = VecDeque::new();

    // Init stack
    stack.push_back(ProgramState {
        position: (0, 0),
        steps: 0,
        memory: memory.clone(),
        index: 0,
        relative_base: 0,
        result: EMPTY,
    });

    while !stack.is_empty() {
        let state = stack.pop_front().unwrap();

        if visited.contains_key(&state.position) {
            continue;
        }

        assert!(state.result == 0 || state.result == 1 || state.result == 2);

        visited.insert(state.position, (state.result, state.steps));

        if state.result == WALL {
            continue;
        }

        if state.result == OXYGEN {
            continue;
        }

        let p = state.position;

        for (dir, pos) in [
            (NORTH, (p.0, p.1 - 1)),
            (SOUTH, (p.0, p.1 + 1)),
            (WEST, (p.0 - 1, p.1)),
            (EAST, (p.0 + 1, p.1)),
        ]
        .iter()
        {
            let (memory_new, output_numbers, index, relative_base, _halted) = intcode::run(
                state.memory.clone(),
                [*dir].to_vec(),
                state.index,
                state.relative_base,
            );

            stack.push_back(ProgramState {
                position: *pos,
                steps: state.steps + 1,
                memory: memory_new,
                index: index,
                relative_base: relative_base,
                result: output_numbers[0] as usize,
            });
        }
    }

    return visited;
}

fn solve_part_one(memory: Vec<i64>) -> usize {
    let map = build_map(memory);
    for (_k, (typ, numsteps)) in map {
        if typ == OXYGEN {
            return numsteps;
        }
    }
    return 0;
}

fn solve_part_two(memory: Vec<i64>) -> usize {
    let mut map = build_map(memory);
    let mut start_pos = (0, 0);
    for (pos, (typ, numsteps)) in &mut map {
        if *typ == OXYGEN {
            start_pos = *pos;
        }
        *numsteps = 0;
    }

    let mut visited: HashSet<Point> = HashSet::new();
    let mut stack: VecDeque<(Point, usize)> = VecDeque::new();
    stack.push_back((start_pos, 0));
    let mut max_dist = 0;
    while !stack.is_empty() {
        let (pos, dist) = stack.pop_front().unwrap();

        if visited.contains(&pos) {
            continue;
        }

        if !map.contains_key(&pos) {
            continue;
        }

        let (typ, _numsteps) = map.get(&pos).unwrap();

        if *typ == WALL {
            continue;
        }

        visited.insert(pos);

        max_dist = std::cmp::max(max_dist, dist);

        for pos in [
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
        ]
        .iter()
        {
            stack.push_back((*pos, dist + 1));
        }
    }

    return max_dist;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone()));
    println!("Part two: {}", solve_part_two(memory));
}
