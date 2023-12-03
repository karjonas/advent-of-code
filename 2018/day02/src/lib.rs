extern crate common;

use std::collections::HashMap;

pub fn part_two(lines: Vec<Vec<char>>) -> String {
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            let mut num_diffs = 0;
            let mut diff_pos = 0;
            let mut found_diff = false;
            for k in 0..lines[i].len() {
                num_diffs += if lines[i][k] != lines[j][k] { 1 } else { 0 };
                if num_diffs == 1 && !found_diff {
                    diff_pos = k;
                    found_diff = true;
                }
            }
            if num_diffs == 1 {
                let mut result = String::new();
                for k in 0..lines[i].len() {
                    if k != diff_pos {
                        result.push(lines[i][k])
                    }
                }
                return result;
            }
        }
    }

    return String::new();
}

pub fn part_one(lines: Vec<Vec<char>>) -> usize {
    let mut num_twos = 0;
    let mut num_threes = 0;
    for line in lines.clone() {
        let mut counts = HashMap::new();
        for c in line {
            *counts.entry(c).or_insert(0) += 1;
        }

        let found_two = counts.values().fold(false, |sum, i| sum || i.clone() == 2);
        let found_three = counts.values().fold(false, |sum, i| sum || i.clone() == 3);

        num_twos += if found_two { 1 } else { 0 };
        num_threes += if found_three { 1 } else { 0 };
    }
    return num_twos * num_threes;
}

pub fn solve(filepath: &str) {
    let contents = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string();
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|v| v.trim().chars().collect())
        .collect();
    println!("Part one: {}", part_one(lines.clone()));
    println!("Part two: {}", part_two(lines));
}
