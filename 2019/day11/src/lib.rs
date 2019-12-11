extern crate common;

use std::collections::HashMap;
use std::collections::VecDeque;

const OPADD: i64 = 1;
const OPMUL: i64 = 2;
const OPSTORE: i64 = 3;
const OPPRINT: i64 = 4;
const OPJMPTRUE: i64 = 5;
const OPJMPFALSE: i64 = 6;
const OPLT: i64 = 7;
const OPEQ: i64 = 8;
const OPRBADD: i64 = 9;

const OPHALT: i64 = 99;

fn parse_input(input: &str) -> Vec<i64> {
    return input
        .split(',')
        .map(|v| common::string_to_i64(v))
        .collect::<Vec<_>>();
}

fn fetch_value(value: i64, mode: i64, relative_base: i64, memory: &mut Vec<i64>) -> i64 {
    assert!(mode == 2 || mode == 1 || mode == 0);

    if mode == 1 {
        return value;
    } else if mode == 0 {
        common::grow(memory, value as usize + 1, 0);
        return memory[value as usize];
    } else if mode == 2 {
        common::grow(memory, (value + relative_base) as usize + 1, 0);
        return memory[(value + relative_base) as usize];
    }
    panic!("invalid mode {}", mode);
}

fn write_value(value: i64, dest: i64, mode: i64, relative_base: i64, memory: &mut Vec<i64>) {
    assert!(mode == 2 || mode == 1 || mode == 0);

    if mode == 1 {
    } else if mode == 0 {
        common::grow(memory, dest as usize + 1, 0);
        memory[dest as usize] = value;
        return;
    } else if mode == 2 {
        common::grow(memory, (dest + relative_base) as usize + 1, 0);
        memory[(dest + relative_base) as usize] = value;
        return;
    }
    panic!("invalid mode {}", mode);
}

fn run_continue(
    mut memory: Vec<i64>,
    mut input_numbers: VecDeque<i64>,
    mut index: usize,
    mut relative_base: i64,
) -> (Vec<i64>, Vec<i64>, usize, i64, bool) {
    let size = memory.len();
    let mut output_numbers = Vec::new();
    let mut halted = false;

    while index < size {
        let op_raw = memory[index];
        let op = op_raw % 100;
        let mode0 = (op_raw % 1000) / 100;
        let mode1 = (op_raw % 10000) / 1000;
        let mode2 = (op_raw % 100000) / 10000;

        assert!(mode0 == 2 || mode0 == 1 || mode0 == 0);
        assert!(mode1 == 2 || mode1 == 1 || mode1 == 0);
        assert!(mode2 == 2 || mode2 == 1 || mode2 == 0);

        if op == OPADD {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let i2 = memory[index + 3];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            let v1 = fetch_value(i1, mode1, relative_base, &mut memory);
            let value = v0 + v1;
            write_value(value, i2, mode2, relative_base, &mut memory);
            index += 4;
        } else if op == OPMUL {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let i2 = memory[index + 3];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            let v1 = fetch_value(i1, mode1, relative_base, &mut memory);
            let value = v0 * v1;
            write_value(value, i2, mode2, relative_base, &mut memory);
            index += 4;
        } else if op == OPSTORE {
            if input_numbers.is_empty() {
                break;
            }
            let i0 = memory[index + 1];
            let value = input_numbers.pop_front().unwrap();
            write_value(value, i0, mode0, relative_base, &mut memory);
            index += 2;
        } else if op == OPPRINT {
            let i0 = memory[index + 1];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            output_numbers.push(v0);
            index += 2;
        } else if op == OPJMPTRUE || op == OPJMPFALSE {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            let v1 = fetch_value(i1, mode1, relative_base, &mut memory);
            let jmptrue = op == OPJMPTRUE && v0 != 0;
            let jmpfalse = op == OPJMPFALSE && v0 == 0;
            if jmptrue || jmpfalse {
                assert!(v1 >= 0);
                index = v1 as usize;
            } else {
                index += 3
            }
        } else if op == OPLT || op == OPEQ {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let i2 = memory[index + 3];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            let v1 = fetch_value(i1, mode1, relative_base, &mut memory);
            let jmplt = op == OPLT && v0 < v1;
            let jmpeq = op == OPEQ && v0 == v1;

            let value = if jmplt || jmpeq { 1 } else { 0 };
            write_value(value, i2, mode2, relative_base, &mut memory);
            index += 4
        } else if op == OPRBADD {
            let i0 = memory[index + 1];
            let v0 = fetch_value(i0, mode0, relative_base, &mut memory);
            relative_base += v0;
            index += 2;
        } else if op == OPHALT {
            halted = true;
            break;
        } else {
            panic!("Invalid State");
        }
    }
    return (memory, output_numbers, index, relative_base, halted);
}

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
        let mut input_numbers: VecDeque<i64> = VecDeque::new();
        input_numbers.push_back(color.clone() as i64);
        let (mem_new, output_numbers, ptr_new, relative_base_new, halted) =
            run_continue(memory.clone(), input_numbers, ptr, relative_base);
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
    let map_first = run_painter(parse_input(input.as_str()), COLOR_BLACK);
    let map_second = run_painter(parse_input(input.as_str()), COLOR_WHITE);

    println!("Part one: {}", map_first.len());
    print!("Part two:\n{}", draw_map(map_second));
}
