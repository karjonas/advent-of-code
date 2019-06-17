use std::fs::File;
use std::io::prelude::*;

fn execute(mut lines: Vec<i32>, part_one: bool) -> usize {
    let mut p = 0;
    let n = lines.len();
    let mut acc = 0;

    while p < n {
        let steps = lines[p];
        if part_one {
            lines[p] = steps + 1;
        } else {
            lines[p] = if steps >= 3 { steps - 1 } else { steps + 1 };
        }
        p = (p as i32 + steps) as usize;
        acc += 1;
    }

    return acc;
}

pub fn solve() {
    let mut file = File::open("2017/day05/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<i32> = contents
        .trim()
        .split('\n')
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect();

    println!("Part one: {}", execute(lines.clone(), true));
    println!("Part two: {}", execute(lines, false));
}
