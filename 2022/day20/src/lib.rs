use std::collections::VecDeque;

extern crate common;

fn shift(state: &mut VecDeque<(i64, usize)>, number_idx: usize) {
    let idx = state.iter().position(|&(_, n)| n == number_idx).unwrap();
    let number = state[idx].0;
    let n = state.len() - 1;
    state.rotate_left(idx);
    let removed = state.remove(0).unwrap();
    if number < 0 {
        state.rotate_right(number.abs() as usize % n);
    } else {
        state.rotate_left(number.abs() as usize % n)
    }
    state.insert(0, removed);
}

fn solve_both(input: &String, key: i64, repeats: usize) -> i64 {
    let orig: VecDeque<i64> = input.lines().map(|v| v.trim().parse().unwrap()).collect();
    let mut state: VecDeque<(i64, usize)> =
        orig.iter().zip(0..).map(|(v, i)| (*v * key, i)).collect();
    let num = orig.len();
    for _ in 0..repeats {
        for number in 0..num {
            shift(&mut state, number);
        }
    }

    let idx_zero = state.iter().position(|&(v, _)| v == 0).unwrap();
    let a = state[(idx_zero + 1000) % num].0;
    let b = state[(idx_zero + 2000) % num].0;
    let c = state[(idx_zero + 3000) % num].0;
    return a + b + c;
}

fn part_one(input: &String) -> i64 {
    solve_both(input, 1, 1)
}

fn part_two(input: &String) -> i64 {
    solve_both(input, 811589153, 10)
}

pub fn solve() {
    let input = common::read_file("2022/day20/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
