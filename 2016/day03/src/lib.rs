use std::fs::File;
use std::io::prelude::*;

fn valid_tri(a: i32, b: i32, c: i32) -> bool {
    return a + b > c && b + c > a && c + a > b;
}

pub fn solve() {
    let mut file = File::open("2016/day03/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tris: Vec<Vec<_>> = contents
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut tris_flip: Vec<Vec<i32>> = Vec::new();

    for i in 0..(tris.len() / 3) {
        for j in 0..3 {
            tris_flip.push(vec![
                tris[i * 3][j],
                tris[(i * 3) + 1][j],
                tris[(i * 3) + 2][j],
            ]);
        }
    }

    let num_ok = tris.iter().fold(0, |sum, t| {
        sum + if valid_tri(t[0], t[1], t[2]) { 1 } else { 0 }
    });
    let num_ok_flip = tris_flip.iter().fold(0, |sum, t| {
        sum + if valid_tri(t[0], t[1], t[2]) { 1 } else { 0 }
    });

    println!("Part 1: {}", num_ok);
    println!("Part 2: {}", num_ok_flip);
}
