extern crate common;

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct State {
    iea_string: Vec<char>,
    lit_pixels: HashSet<(i32, i32)>,
    outside: usize,
}

fn read_9bit(
    pos: (i32, i32),
    lit_pixels: &HashSet<(i32, i32)>,
    min: (i32, i32),
    max: (i32, i32),
    outside: usize,
) -> usize {
    let mut value = 0;
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let p = (pos.0 + dx, pos.1 + dy);
        let digit = if p.0 < min.0 || p.0 > max.0 || p.1 < min.1 || p.1 > max.1 {
            outside
        } else {
            lit_pixels.contains(&p) as usize
        };
        value = value * 2 + digit;
    }

    return value;
}

fn parse_input(input: &String) -> State {
    let mut state = State {
        iea_string: Vec::new(),
        lit_pixels: HashSet::new(),
        outside: 0,
    };

    let lines: Vec<&str> = input.lines().collect();
    state.iea_string = lines[0].chars().collect();
    assert_eq!(state.iea_string.len(), 512);

    for y in 2..lines.len() {
        let chars: Vec<_> = lines[y].chars().collect();
        for x in 0..chars.len() {
            if chars[x] == '#' {
                state.lit_pixels.insert((x as i32, y as i32 - 2));
            }
        }
    }

    return state;
}

fn enhance_image(state: &State) -> State {
    let mut lit_pixels = HashSet::new();
    let mut min = (0, 0);
    let mut max = (0, 0);
    let pad = 3;

    for p in &state.lit_pixels {
        min = (std::cmp::min(min.0, p.0), std::cmp::min(min.1, p.1));
        max = (std::cmp::max(max.0, p.0), std::cmp::max(max.1, p.1));
    }

    for x in (min.0 - pad)..(max.0 + 1 + pad) {
        for y in (min.1 - pad)..(max.1 + 1 + pad) {
            let p = (x, y);
            let index = read_9bit(p, &state.lit_pixels, min, max, state.outside);
            if state.iea_string[index] == '#' {
                lit_pixels.insert(p);
            }
        }
    }

    let outside = ((state.outside == 0 && state.iea_string[0] == '#')
        || (state.outside == 1 && state.iea_string[511] == '#')) as usize;

    return State {
        iea_string: state.iea_string.clone(),
        lit_pixels: lit_pixels,
        outside: outside,
    };
}

fn solve_repeat(mut state: State, repeat: usize) -> usize {
    for _ in 0..repeat {
        state = enhance_image(&state);
    }
    return state.lit_pixels.len();
}

pub fn solve() {
    let input = common::read_file("2021/day20/input");
    let state = parse_input(&input);
    println!("Part one: {:?}", solve_repeat(state.clone(), 2));
    println!("Part two: {:?}", solve_repeat(state.clone(), 50));
}
