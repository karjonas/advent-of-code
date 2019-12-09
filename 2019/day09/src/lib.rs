extern crate common;

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
) -> (Vec<i64>, Vec<i64>, usize, bool) {
    let size = memory.len();
    let mut output_numbers = Vec::new();
    let mut halted = false;
    let mut relative_base = 0;

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
                panic!("No numbers");
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
    return (memory, output_numbers, index, halted);
}

fn run_input_vec(memory: Vec<i64>, input_numbers: VecDeque<i64>) -> (Vec<i64>, Vec<i64>) {
    let (mem_new, output_numbers, _index, _halted) = run_continue(memory, input_numbers, 0);
    return (mem_new, output_numbers);
}

fn solve_part_one(memory: Vec<i64>) -> i64 {
    let (_memory_out, output_numbers) = run_input_vec(memory, VecDeque::from([1].to_vec()));
    return output_numbers[0];
}
fn solve_part_two(memory: Vec<i64>) -> i64 {
    let (_memory_out, output_numbers) = run_input_vec(memory, VecDeque::from([2].to_vec()));
    return output_numbers[0];
}

pub fn solve() {
    let input = common::read_file("2019/day09/input");
    println!("Part one: {}", solve_part_one(parse_input(input.as_str())));
    println!("Part two: {}", solve_part_two(parse_input(input.as_str())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_old() {
        assert_eq!(
            run_input_vec(
                [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec(),
                VecDeque::from([0].to_vec())
            )
            .0,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].to_vec()
        );
        assert_eq!(
            run_input_vec([1, 0, 0, 0, 99].to_vec(), VecDeque::from([0].to_vec())).0,
            [2, 0, 0, 0, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 3, 0, 3, 99].to_vec(), VecDeque::from([0].to_vec())).0,
            [2, 3, 0, 6, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 4, 4, 5, 99, 0].to_vec(), VecDeque::from([0].to_vec())).0,
            [2, 4, 4, 5, 99, 9801].to_vec()
        );
        assert_eq!(
            run_input_vec(
                [1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec(),
                VecDeque::from([0].to_vec())
            )
            .0,
            [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec()
        );
    }

    #[test]
    fn test_samples_old_0() {
        let prog0 = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec();
        assert_eq!(
            run_input_vec(prog0.clone(), VecDeque::from([8].to_vec())).1,
            [1].to_vec()
        );
        assert_eq!(
            run_input_vec(prog0.clone(), VecDeque::from([888].to_vec())).1,
            [0].to_vec()
        );
    }

    #[test]
    fn test_samples_old_1() {
        let prog1 = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec();
        assert_eq!(
            run_input_vec(prog1.clone(), VecDeque::from([7].to_vec())).1,
            [1].to_vec()
        );
        assert_eq!(
            run_input_vec(prog1.clone(), VecDeque::from([8].to_vec())).1,
            [0].to_vec()
        );
    }
    #[test]
    fn test_samples_old_2() {
        let prog2 = [3, 3, 1108, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(
            run_input_vec(prog2.clone(), VecDeque::from([8].to_vec())).1,
            [1].to_vec()
        );
        assert_eq!(
            run_input_vec(prog2.clone(), VecDeque::from([888].to_vec())).1,
            [0].to_vec()
        );
    }
    #[test]
    fn test_samples_old_3() {
        let prog3 = [3, 3, 1107, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(
            run_input_vec(prog3.clone(), VecDeque::from([7].to_vec())).1,
            [1].to_vec()
        );
        assert_eq!(
            run_input_vec(prog3.clone(), VecDeque::from([8].to_vec())).1,
            [0].to_vec()
        );
    }
    #[test]
    fn test_samples_old_4() {
        let prog4 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec();
        assert_eq!(
            run_input_vec(prog4.clone(), VecDeque::from([0].to_vec())).1,
            [0].to_vec()
        );
        assert_eq!(
            run_input_vec(prog4.clone(), VecDeque::from([1].to_vec())).1,
            [1].to_vec()
        );
    }
    #[test]
    fn test_samples_old_5() {
        let prog5 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].to_vec();
        assert_eq!(
            run_input_vec(prog5.clone(), VecDeque::from([0].to_vec())).1,
            [0].to_vec()
        );
        assert_eq!(
            run_input_vec(prog5.clone(), VecDeque::from([1].to_vec())).1,
            [1].to_vec()
        );
    }
    #[test]
    fn test_samples_old_6() {
        let prog6 = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
        .to_vec();

        assert_eq!(
            run_input_vec(prog6.clone(), VecDeque::from([7].to_vec())).1,
            [999].to_vec()
        );
        assert_eq!(
            run_input_vec(prog6.clone(), VecDeque::from([8].to_vec())).1,
            [1000].to_vec()
        );
        assert_eq!(
            run_input_vec(prog6.clone(), VecDeque::from([9].to_vec())).1,
            [1001].to_vec()
        );
    }

    #[test]
    fn test_samples_part_one_a() {
        let prog = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]
        .to_vec();

        assert_eq!(run_input_vec(prog.clone(), VecDeque::new()).1, prog);
    }

    #[test]
    fn test_samples_part_one_b() {
        let prog = [1102, 34915192, 34915192, 7, 4, 7, 99, 0].to_vec();
        assert_eq!(
            run_input_vec(prog.clone(), VecDeque::new()).1[0]
                .to_string()
                .len(),
            16
        );
    }

    #[test]
    fn test_samples_part_one_c() {
        let prog = [104, 1125899906842624, 99].to_vec();
        assert_eq!(
            run_input_vec(prog.clone(), VecDeque::new()).1,
            [1125899906842624]
        );
    }
}
