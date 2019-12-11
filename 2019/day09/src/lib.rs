extern crate common;
extern crate intcode;

use std::collections::VecDeque;

fn run_input_vec(memory: Vec<i64>, input_numbers: VecDeque<i64>) -> (Vec<i64>, Vec<i64>) {
    let (mem_new, output_numbers, _index, _relative_base, _halted) =
        intcode::run(memory, input_numbers, 0, 0);
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
    println!(
        "Part one: {}",
        solve_part_one(intcode::parse_input(input.as_str()))
    );
    println!(
        "Part two: {}",
        solve_part_two(intcode::parse_input(input.as_str()))
    );
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
