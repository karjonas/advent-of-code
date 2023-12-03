extern crate common;

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let mut calories: Vec<usize> = input
        .split("\n\n")
        .map(|nums| nums.split('\n').map(|v| v.parse::<usize>().unwrap()).sum())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    println!("Part one: {}", calories[0]);
    println!("Part two: {}", calories[0] + calories[1] + calories[2]);
}
