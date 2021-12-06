extern crate common;

use std::collections::HashMap;

fn solve_recursive(number: usize, days: usize, spawns: &mut HashMap<usize, usize>) -> usize {
    if number >= days {
        return 1;
    }

    let spawn_days = days - number - 1;

    let result;
    if spawns.contains_key(&spawn_days) {
        return *spawns.get(&spawn_days).unwrap();
    } else {
        result = solve_recursive(6, spawn_days, spawns) + solve_recursive(8, spawn_days, spawns);
        spawns.insert(spawn_days, result);
    }

    return result;
}

fn solve_internal(input: &String, days: usize) -> usize {
    let numbers: Vec<usize> = input
        .split(",")
        .map(|v| common::string_to_usize(v))
        .collect();

    let mut sum = 0;
    let mut spawns = HashMap::<usize, usize>::new();

    for number in numbers {
        sum += solve_recursive(number, days, &mut spawns);
    }

    return sum;
}

fn solve_internal_p1(input: &String) -> usize {
    return solve_internal(input, 80);
}

fn solve_internal_p2(input: &String) -> usize {
    return solve_internal(input, 256);
}

pub fn solve() {
    let input = common::read_file("2021/day06/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "3,4,3,1,2";

        assert_eq!(solve_internal(&String::from(input), 2), 6);
        assert_eq!(solve_internal(&String::from(input), 3), 7);
        assert_eq!(solve_internal(&String::from(input), 8), 10);
        assert_eq!(solve_internal(&String::from(input), 14), 20);
        assert_eq!(solve_internal(&String::from(input), 15), 20);
        assert_eq!(solve_internal(&String::from(input), 16), 21);
        assert_eq!(solve_internal(&String::from(input), 17), 22);
        assert_eq!(solve_internal(&String::from(input), 18), 26);
        assert_eq!(solve_internal(&String::from(input), 80), 5934);
    }
}
