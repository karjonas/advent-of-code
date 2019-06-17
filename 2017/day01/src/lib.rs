use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day01/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let ints: Vec<usize> = contents
        .chars()
        .take_while(|v| v.clone() != '\n')
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect();

    let mut sum = 0;
    let n = ints.len();
    for i in 0..n {
        sum = sum
            + if ints[i] == ints[(i + 1) % n] {
                ints[i]
            } else {
                0
            };
    }

    println!("Part one: {}", sum);

    sum = 0;
    for i in 0..n {
        sum = sum
            + if ints[i] == ints[(i + n / 2) % n] {
                ints[i]
            } else {
                0
            };
    }

    println!("Part two: {}", sum);
}
