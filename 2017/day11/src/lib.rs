use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day11/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input_p1: Vec<_> = contents.split(',').collect();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;

    let mut furthest = 0;

    for d in input_p1 {
        if d == "n" {
            x += 1;
            z -= 1;
        } else if d == "ne" {
            y -= 1;
            x += 1;
        } else if d == "se" {
            y -= 1;
            z += 1;
        } else if d == "s" {
            x -= 1;
            z += 1;
        } else if d == "sw" {
            x -= 1;
            y += 1;
        } else if d == "nw" {
            y += 1;
            z -= 1;
        }

        let d = (x.abs() + y.abs() + z.abs()) / 2;
        furthest = std::cmp::max(furthest, d);
    }

    let d = (x.abs() + y.abs() + z.abs()) / 2;

    println!("Part one: {}", d);
    println!("Part two: {}", furthest);
}
