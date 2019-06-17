extern crate md5;

use std::collections::VecDeque;

const SIZE: usize = 4;

struct State {
    history: String,
    pos: (usize, usize),
}

fn find_possible_dirs(state: &State) -> VecDeque<State> {
    let mut out: VecDeque<State> = VecDeque::new();

    let mut s: String = "lpvhkcbi".to_string();
    s.push_str(state.history.as_str());
    let digest = md5::compute(s);
    let hash = format!("{:x}", digest);

    let b_up = hash.chars().nth(0).unwrap();
    let b_down = hash.chars().nth(1).unwrap();
    let b_left = hash.chars().nth(2).unwrap();
    let b_right = hash.chars().nth(3).unwrap();
    let open_chars = vec!['b', 'c', 'd', 'e', 'f'];
    let is_open = |c: char| open_chars.contains(&c);

    if state.pos.0 > 0 && is_open(b_left) {
        let mut h = state.history.clone();
        h.push('L');
        out.push_back(State {
            history: h,
            pos: (state.pos.0 - 1, state.pos.1),
        });
    }
    if state.pos.0 < SIZE - 1 && is_open(b_right) {
        let mut h = state.history.clone();
        h.push('R');
        out.push_back(State {
            history: h,
            pos: (state.pos.0 + 1, state.pos.1),
        });
    }
    if state.pos.1 > 0 && is_open(b_up) {
        let mut h = state.history.clone();
        h.push('U');
        out.push_back(State {
            history: h,
            pos: (state.pos.0, state.pos.1 - 1),
        });
    }
    if state.pos.1 < SIZE - 1 && is_open(b_down) {
        let mut h = state.history.clone();
        h.push('D');
        out.push_back(State {
            history: h,
            pos: (state.pos.0, state.pos.1 + 1),
        });
    }

    return out;
}

fn solve_internal() {
    let mut states: VecDeque<State> = VecDeque::new();
    states.push_back(State {
        history: String::new(),
        pos: (0, 0),
    });

    let mut shortest_path = std::usize::MAX;
    let mut shortest_path_str = String::new();
    let mut longest_path = 0;

    while !states.is_empty() {
        let s = states.pop_front().unwrap();
        let l = s.history.len();
        if s.pos == (3, 3) {
            if l < shortest_path {
                shortest_path_str = s.history.clone();
                shortest_path = l;
            }

            longest_path = std::cmp::max(longest_path, l);
            continue;
        }
        let mut next = find_possible_dirs(&s);
        states.append(&mut next);
    }

    println!("Part 1: {:?}", shortest_path_str);
    println!("Part 2: {:?}", longest_path);
}

pub fn solve() {
    solve_internal();
}
