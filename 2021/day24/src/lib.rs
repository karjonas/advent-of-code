extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn parse_input(input: &String) -> Vec<(i64, i64, i64)> {
    let mut result = Vec::new();
    let mut x = 0;
    let mut y;
    let mut z = 0;
    let mut read_x = false;
    let mut read_y = false;
    let mut read_z = false;

    for line in input.lines() {
        if read_z {
            let z_scan = scan_fmt!(line, "div z {d}", i64).unwrap();
            z = z_scan;
            read_z = false;
            read_x = true;
        } else if read_x {
            let x_scan = scan_fmt!(line, "add x {d}", i64).unwrap();
            x = x_scan;
            read_x = false;
        } else if read_y {
            let y_scan = scan_fmt!(line, "add y {d}", i64).unwrap();
            y = y_scan;
            read_y = false;

            result.push((x, y, z));
        }

        if line == "mod x 26" {
            read_z = true;
        } else if line == "add y w" {
            read_y = true;
        }
    }

    assert_eq!(result.len(), 14);

    return result;
}

fn backward(a: i64, b: i64, c: i64, z_goal: i64, w: i64) -> Vec<i64> {
    let mut zs = Vec::new();
    let x = z_goal - w - b;
    if x % 26 == 0 {
        zs.push(x / 26 * c);
    }
    if 0 <= (w - a) && (w - a) < 26 {
        let z0 = z_goal * c;
        zs.push(w - a + z0);
    }
    return zs;
}

fn solve_internal(part_two: bool, xyzs: &Vec<(i64, i64, i64)>) -> usize {
    let mut zs = HashSet::from([0]);
    let mut result = HashMap::<i64, VecDeque<i64>>::new();

    for &(a, b, c) in xyzs.iter().rev() {
        let mut newzs = HashSet::new();
        for w_i in 1..10 {
            let w = if part_two { 10 - w_i } else { w_i };
            for &z in &zs {
                let z0s = backward(a, b, c, z, w);
                for z0 in z0s {
                    newzs.insert(z0);
                    let mut rz = result.entry(z).or_insert(VecDeque::new()).clone();
                    rz.insert(0, w);
                    result.insert(z0, rz);
                }
            }
        }
        zs = newzs;
    }

    return result
        .get(&0)
        .unwrap()
        .iter()
        .fold(0, |sum, &v| sum * 10 + v) as usize;
}

pub fn solve() {
    let input = parse_input(&common::read_file("2021/day24/input"));

    println!("Part one: {}", solve_internal(false, &input));
    println!("Part two: {}", solve_internal(true, &input));
}
