extern crate common;
extern crate intcode;

use std::collections::HashMap;

const COLOR_BLACK: i64 = 0;
const COLOR_WHITE: i64 = 1;

const TURN_LEFT: i64 = 0;
const TURN_RIGHT: i64 = 1;

const DIR_N: i64 = 0;
const DIR_E: i64 = 1;
const DIR_S: i64 = 2;
const DIR_W: i64 = 3;
const NUM_DIRS: i64 = 4;

fn run_painter(mut memory: Vec<i64>, start_color: i64) -> HashMap<(i64, i64), i64> {
    let mut ptr = 0;
    let mut relative_base = 0;
    let mut dir = DIR_N;
    let mut pos: (i64, i64) = (0, 0);
    let mut color_pos: HashMap<(i64, i64), i64> = HashMap::new();
    color_pos.entry(pos).or_insert(start_color);

    loop {
        let color = color_pos.entry(pos).or_insert(COLOR_BLACK);
        let (mem_new, output_numbers, ptr_new, relative_base_new, halted) =
            intcode::run(memory.clone(), [*color as i64].to_vec(), ptr, relative_base);
        assert_eq!(output_numbers.len(), 2);
        let color_new = output_numbers[0];
        let turn = output_numbers[1];

        color_pos.insert(pos, color_new);

        assert!(color_new == COLOR_BLACK || color_new == COLOR_WHITE);
        assert!(turn == TURN_LEFT || turn == TURN_RIGHT);

        if turn == TURN_LEFT {
            dir = (dir + NUM_DIRS - 1) % NUM_DIRS;
        } else {
            dir = (dir + 1) % NUM_DIRS;
        }

        if dir == DIR_N {
            pos = (pos.0, pos.1 - 1);
        } else if dir == DIR_W {
            pos = (pos.0 - 1, pos.1);
        } else if dir == DIR_E {
            pos = (pos.0 + 1, pos.1);
        } else if dir == DIR_S {
            pos = (pos.0, pos.1 + 1);
        }

        if halted {
            break;
        }

        ptr = ptr_new;
        memory = mem_new;
        relative_base = relative_base_new;
    }

    return color_pos;
}

fn draw_map(map: HashMap<(i64, i64), i64>) -> String {
    let mut min = (0, 0);
    let mut max = (0, 0);

    for (k, _v) in &map {
        min = (std::cmp::min(min.0, k.0), std::cmp::min(min.1, k.1));
        max = (std::cmp::max(max.0, k.0), std::cmp::max(max.1, k.1));
    }

    let w = 1 + max.0 - min.0;
    let h = 1 + max.1 - min.1;

    let mut s = String::new();

    for y in 0..h {
        for x in 0..w {
            let color = map
                .get(&(min.0 + x, min.1 + y))
                .unwrap_or(&COLOR_BLACK)
                .clone();
            s.push(if color == COLOR_BLACK { ' ' } else { '#' });
        }
        s.push('\n')
    }

    return s;
}

pub fn solve() {
    let input = common::read_file("2019/day11/input");
    let map_first = run_painter(intcode::parse_input(input.as_str()), COLOR_BLACK);
    let map_second = run_painter(intcode::parse_input(input.as_str()), COLOR_WHITE);

    println!("Part one: {}", map_first.len());
    print!("Part two:\n{}", draw_map(map_second));
}
