extern crate common;

pub fn solve() {
    let input = common::read_file("2022/day01/input");
    let mut calories: Vec<usize> = input
        .split("\n\n")
        .map(|nums| nums.split('\n').map(|v| v.parse::<usize>().unwrap()).sum())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    println!("Part one: {}", calories[0]);
    println!("Part two: {}", calories[0] + calories[1] + calories[2]);
}
