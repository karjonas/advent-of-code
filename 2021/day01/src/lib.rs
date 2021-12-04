extern crate common;

fn solve_internal(lines: &Vec<i64>, window: usize) -> usize {
        let mut last = std::i64::MAX;
        let mut num_incs = 0;
        for i in window..lines.len() + 1 {
                let mut curr = 0;
                for j in 0..window {
                        curr += lines[i + j - window];
                }
                num_incs += if curr > last { 1 } else { 0 };
                last = curr;
        }
        return num_incs;
}

pub fn solve() {
        let input: Vec<i64> = common::read_file("2021/day01/input")
                .lines()
                .map(|s| common::string_to_i64(s))
                .collect();

        println!("Part one: {}", solve_internal(&input, 1));
        println!("Part two: {}", solve_internal(&input, 3));
}
