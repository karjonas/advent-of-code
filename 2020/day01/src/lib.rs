extern crate common;

fn solve_first(input: &Vec<i64>) -> i64 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }

    panic!("No solution found");
}

fn solve_second(input: &Vec<i64>) -> i64 {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }

    panic!("No solution found");
}

pub fn solve(filepath: &str) {
    let input: Vec<i64> = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|s| common::string_to_i64(s))
        .collect();

    println!("Part one: {:?}", solve_first(&input));
    println!("Part two: {:?}", solve_second(&input));
}
