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

const OPHALT: i64 = 99;

fn parse_input(input: &str) -> Vec<i64> {
    return input
        .split(',')
        .map(|v| common::string_to_i64(v))
        .collect::<Vec<_>>();
}

fn run_continue(
    mut memory: Vec<i64>,
    mut input_numbers: VecDeque<i64>,
    mut index: usize,
) -> (Vec<i64>, i64, usize, bool) {
    let size = memory.len();
    let mut output_number = 0;
    let mut halted = false;
    let mut outputset = false;

    while index < size {
        let op_raw = memory[index];
        let op = op_raw % 100;
        let mode0 = (op_raw % 1000) / 100;
        let mode1 = (op_raw % 10000) / 1000;
        let mode2 = (op_raw % 100000) / 10000;

        assert!(mode0 == 1 || mode0 == 0);
        assert!(mode1 == 1 || mode1 == 0);
        assert!(mode2 == 0);
        assert!(mode2 == 1 || mode2 == 0);

        if op == OPADD {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let i2 = memory[index + 3];
            let v0 = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            let v1 = if mode1 == 1 { i1 } else { memory[i1 as usize] };
            let dest = i2;
            let value = v0 + v1;
            memory[dest as usize] = value;
            index += 4;
        } else if op == OPMUL {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let i2 = memory[index + 3];
            let v0 = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            let v1 = if mode1 == 1 { i1 } else { memory[i1 as usize] };
            let dest = i2;
            let value = v0 * v1;
            memory[dest as usize] = value;
            index += 4;
        } else if op == OPSTORE {
            assert!(mode0 == 0);
            if input_numbers.is_empty() {
                break;
            }

            let i0 = memory[index + 1];
            let dest = i0;
            memory[dest as usize] = input_numbers.pop_front().unwrap();
            index += 2;
        } else if op == OPPRINT {
            let i0 = memory[index + 1];
            let dest = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            assert_eq!(outputset, false);
            outputset = true;
            output_number = dest;
            index += 2;
        } else if op == OPJMPTRUE || op == OPJMPFALSE {
            let i0 = memory[index + 1];
            let i1 = memory[index + 2];
            let v0 = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            let v1 = if mode1 == 1 { i1 } else { memory[i1 as usize] };
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
            let v0 = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            let v1 = if mode1 == 1 { i1 } else { memory[i1 as usize] };
            let v2 = i2;
            assert!(v2 >= 0);

            let jmplt = op == OPLT && v0 < v1;
            let jmpeq = op == OPEQ && v0 == v1;
            if jmplt || jmpeq {
                memory[v2 as usize] = 1;
            } else {
                memory[v2 as usize] = 0;
            }
            index += 4
        } else if op == OPHALT {
            halted = true;
            break;
        } else {
            panic!("Invalid State");
        }
    }
    return (memory, output_number, index, halted);
}

fn run_input_vec(memory: Vec<i64>, input_numbers: VecDeque<i64>) -> (Vec<i64>, i64) {
    let (mem_new, output_number, _index, _halted) = run_continue(memory, input_numbers, 0);
    return (mem_new, output_number);
}

fn run_part_one(memory: Vec<i64>, thruster_inputs: [i64; 5]) -> i64 {
    let mut output_last = 0;
    for thruster_input in &thruster_inputs {
        let (_memory, output_number) = run_input_vec(
            memory.clone(),
            VecDeque::from([thruster_input.clone(), output_last].to_vec()),
        );
        output_last = output_number;
    }
    return output_last;
}

fn solve_part_one(memory: Vec<i64>) -> i64 {
    let mut max_value = 0;
    let perms = common::permute(&Vec::new(), &(0..5).collect::<Vec<i64>>());

    for p in perms {
        let thruster_inputs = [p[0], p[1], p[2], p[3], p[4]];
        let value = run_part_one(memory.clone(), thruster_inputs);
        max_value = std::cmp::max(max_value, value);
    }

    return max_value;
}

fn run_part_two(memory: Vec<i64>, thruster_inputs: [i64; 5]) -> i64 {
    let mut thruster_id = 0;
    let mut first_run = [true, true, true, true, true];
    let mut positions = [0, 0, 0, 0, 0];
    let mut outputs = [0, 0, 0, 0, 0];
    let mut memories = [
        memory.clone(),
        memory.clone(),
        memory.clone(),
        memory.clone(),
        memory.clone(),
    ];
    loop {
        let mut input_numbers = VecDeque::<i64>::new();
        if first_run[thruster_id] {
            input_numbers.push_back(thruster_inputs[thruster_id]);
        }
        let prev_thruster_id = (thruster_id + 5 - 1) % 5;
        input_numbers.push_back(outputs[prev_thruster_id]);

        let (mem_new, output_number, position_new, halted) = run_continue(
            memories[thruster_id].clone(),
            input_numbers,
            positions[thruster_id],
        );

        outputs[thruster_id] = output_number;

        if halted && thruster_id == 4 {
            return outputs[4];
        }

        memories[thruster_id] = mem_new;
        positions[thruster_id] = position_new;

        first_run[thruster_id] = false;
        thruster_id = (thruster_id + 1) % 5;
    }
}

fn solve_part_two(memory: Vec<i64>) -> i64 {
    let mut max_value = 0;
    let perms = common::permute(&Vec::new(), &(5..10).collect::<Vec<i64>>());

    for p in perms {
        let thruster_inputs = [p[0], p[1], p[2], p[3], p[4]];
        let value = run_part_two(memory.clone(), thruster_inputs);
        max_value = std::cmp::max(max_value, value);
    }

    return max_value;
}

pub fn solve() {
    let input = common::read_file("2019/day07/input");
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
            1
        );
        assert_eq!(
            run_input_vec(prog0.clone(), VecDeque::from([888].to_vec())).1,
            0
        );
    }

    #[test]
    fn test_samples_old_1() {
        let prog1 = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec();
        assert_eq!(
            run_input_vec(prog1.clone(), VecDeque::from([7].to_vec())).1,
            1
        );
        assert_eq!(
            run_input_vec(prog1.clone(), VecDeque::from([8].to_vec())).1,
            0
        );
    }
    #[test]
    fn test_samples_old_2() {
        let prog2 = [3, 3, 1108, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(
            run_input_vec(prog2.clone(), VecDeque::from([8].to_vec())).1,
            1
        );
        assert_eq!(
            run_input_vec(prog2.clone(), VecDeque::from([888].to_vec())).1,
            0
        );
    }
    #[test]
    fn test_samples_old_3() {
        let prog3 = [3, 3, 1107, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(
            run_input_vec(prog3.clone(), VecDeque::from([7].to_vec())).1,
            1
        );
        assert_eq!(
            run_input_vec(prog3.clone(), VecDeque::from([8].to_vec())).1,
            0
        );
    }
    #[test]
    fn test_samples_old_4() {
        let prog4 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec();
        assert_eq!(
            run_input_vec(prog4.clone(), VecDeque::from([0].to_vec())).1,
            0
        );
        assert_eq!(
            run_input_vec(prog4.clone(), VecDeque::from([1].to_vec())).1,
            1
        );
    }
    #[test]
    fn test_samples_old_5() {
        let prog5 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].to_vec();
        assert_eq!(
            run_input_vec(prog5.clone(), VecDeque::from([0].to_vec())).1,
            0
        );
        assert_eq!(
            run_input_vec(prog5.clone(), VecDeque::from([1].to_vec())).1,
            1
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
            999
        );
        assert_eq!(
            run_input_vec(prog6.clone(), VecDeque::from([8].to_vec())).1,
            1000
        );
        assert_eq!(
            run_input_vec(prog6.clone(), VecDeque::from([9].to_vec())).1,
            1001
        );
    }

    #[test]
    fn test_samples_part_one_0() {
        assert_eq!(
            43210,
            run_part_one(
                [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0].to_vec(),
                [4, 3, 2, 1, 0]
            )
        );
    }

    #[test]
    fn test_samples_part_one_1() {
        assert_eq!(
            54321,
            run_part_one(
                [
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ]
                .to_vec(),
                [0, 1, 2, 3, 4]
            )
        );
    }

    #[test]
    fn test_samples_part_one_2() {
        assert_eq!(
            65210,
            run_part_one(
                [
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ]
                .to_vec(),
                [1, 0, 4, 3, 2]
            )
        );
    }

    /////////////////////////////////////////////////////////////////////////////////////////////
    #[test]
    fn test_samples_part_one_0b() {
        assert_eq!(
            43210,
            solve_part_one(
                [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0].to_vec()
            )
        );
    }

    #[test]
    fn test_samples_part_one_1b() {
        assert_eq!(
            54321,
            solve_part_one(
                [
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ]
                .to_vec()
            )
        );
    }

    #[test]
    fn test_samples_part_one_2b() {
        assert_eq!(
            65210,
            solve_part_one(
                [
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ]
                .to_vec()
            )
        );
    }

    #[test]
    fn test_samples_part_two_a() {
        assert_eq!(
            139629729,
            solve_part_two(
                [
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ]
                .to_vec()
            )
        );
    }
    #[test]
    fn test_samples_part_two_b() {
        assert_eq!(
            18216,
            solve_part_two(
                [
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ]
                .to_vec()
            )
        );
    }
}
