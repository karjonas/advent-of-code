extern crate common;

use std::collections::HashSet;

pub fn solve(filepath: &str) {
    let contents = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string();
    let ints: Vec<i32> = contents
        .lines()
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect();

    {
        let sum = ints.iter().fold(0i32, |sum, val| sum + val);
        println!("Part one: {}", sum);
    }

    {
        let mut idx = 0;
        let mut sum = 0;
        let mut freqs = HashSet::new();
        let answer;
        loop {
            sum = sum + ints[idx];
            if freqs.contains(&sum) {
                answer = sum;
                break;
            } else {
                freqs.insert(sum);
            }

            idx = (idx + 1) % ints.len();
        }

        println!("Part two: {}", answer)
    }
}
