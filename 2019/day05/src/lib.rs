extern crate common;

const OPADD: i64 = 1;
const OPMUL: i64 = 2;
const OPSTORE: i64 = 3;
const OPPRINT: i64 = 4;
const OPJMPTRUE: i64 = 5;
const OPJMPFALSE: i64 = 6;
const OPLT: i64 = 7;
const OPEQ: i64 = 8;

const OPHALT: i64 = 99;

fn run_input(input_str: &str, input_number: i64) -> i64 {
    let input: Vec<i64> = input_str
        .split(',')
        .map(|v| common::string_to_i64(v))
        .collect::<Vec<_>>();
    let (_, result) = run_input_vec(input, input_number);
    return result;
}

fn run_input_vec(mut memory: Vec<i64>, mut input_number: i64) -> (Vec<i64>, i64) {
    let size = memory.len();
    let mut index = 0;

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

            let i0 = memory[index + 1];
            let dest = i0;
            memory[dest as usize] = input_number;
            index += 2;
        } else if op == OPPRINT {
            let i0 = memory[index + 1];
            let dest = if mode0 == 1 { i0 } else { memory[i0 as usize] };
            input_number = dest;
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
            break;
        } else {
            panic!("Invalid State");
        }
    }
    return (memory, input_number);
}

pub fn solve() {
    let input = common::read_file("2019/day05/input");
    println!("Part one: {}", run_input(input.as_str(), 1));
    println!("Part two: {}", run_input(input.as_str(), 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(
            run_input_vec([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].to_vec(), 0).0,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].to_vec()
        );
        assert_eq!(
            run_input_vec([1, 0, 0, 0, 99].to_vec(), 0).0,
            [2, 0, 0, 0, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 3, 0, 3, 99].to_vec(), 0).0,
            [2, 3, 0, 6, 99].to_vec()
        );
        assert_eq!(
            run_input_vec([2, 4, 4, 5, 99, 0].to_vec(), 0).0,
            [2, 4, 4, 5, 99, 9801].to_vec()
        );
        assert_eq!(
            run_input_vec([1, 1, 1, 4, 99, 5, 6, 0, 99].to_vec(), 0).0,
            [30, 1, 1, 4, 2, 5, 6, 0, 99].to_vec()
        );
    }

    #[test]
    fn test_samples_part_two_0() {
        let prog0 = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec();
        assert_eq!(run_input_vec(prog0.clone(), 8).1, 1);
        assert_eq!(run_input_vec(prog0.clone(), 888).1, 0);
    }

    #[test]
    fn test_samples_part_two_1() {
        let prog1 = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec();
        assert_eq!(run_input_vec(prog1.clone(), 7).1, 1);
        assert_eq!(run_input_vec(prog1.clone(), 8).1, 0);
    }
    #[test]
    fn test_samples_part_two_2() {
        let prog2 = [3, 3, 1108, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(run_input_vec(prog2.clone(), 8).1, 1);
        assert_eq!(run_input_vec(prog2.clone(), 888).1, 0);
    }
    #[test]
    fn test_samples_part_two_3() {
        let prog3 = [3, 3, 1107, -1, 8, 3, 4, 3, 99].to_vec();
        assert_eq!(run_input_vec(prog3.clone(), 7).1, 1);
        assert_eq!(run_input_vec(prog3.clone(), 8).1, 0);
    }
    #[test]
    fn test_samples_part_two_4() {
        let prog4 = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9].to_vec();
        assert_eq!(run_input_vec(prog4.clone(), 0).1, 0);
        assert_eq!(run_input_vec(prog4.clone(), 1).1, 1);
    }
    #[test]
    fn test_samples_part_two_5() {
        let prog5 = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1].to_vec();
        assert_eq!(run_input_vec(prog5.clone(), 0).1, 0);
        assert_eq!(run_input_vec(prog5.clone(), 1).1, 1);
    }
    #[test]
    fn test_samples_part_two_6() {
        let prog6 = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
        .to_vec();

        assert_eq!(run_input_vec(prog6.clone(), 7).1, 999);
        assert_eq!(run_input_vec(prog6.clone(), 8).1, 1000);
        assert_eq!(run_input_vec(prog6.clone(), 9).1, 1001);
    }
}
