use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2016/day06/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<_> = contents.lines().collect();
    let num_cols = lines[0].len();

    let mut decrypted0 = String::new();
    let mut decrypted1 = String::new();

    for i in 0..num_cols {
        let mut hm = HashMap::new();
        for line in &lines {
            assert!(line.len() == num_cols);
            let c = line.chars().nth(i).unwrap();
            let hits = match hm.get(&c) {
                Some(&number) => number,
                _ => 0,
            };
            hm.insert(c, hits + 1);
        }

        let (max_char, _) = hm.iter().fold(('0', 0), |sum, pair| {
            if sum.0 == '0' || pair.1 > &sum.1 {
                (pair.0.clone(), pair.1.clone())
            } else {
                sum
            }
        });
        let (min_char, _) = hm.iter().fold(('0', 0), |sum, pair| {
            if sum.0 == '0' || pair.1 < &sum.1 {
                (pair.0.clone(), pair.1.clone())
            } else {
                sum
            }
        });

        decrypted0.push(max_char);
        decrypted1.push(min_char);
    }

    println!("Part1: {}", decrypted0);
    println!("Part2: {}", decrypted1);
}
