extern crate regex;

use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn reverse_columns(arr: &mut Vec<Vec<char>>) {
    let n = arr.len();

    for i in 0..n {
        let mut j = 0;
        let mut k = n - 1;

        while j < k {
            let tmp = arr[j][i];
            arr[j][i] = arr[k][i];
            arr[k][i] = tmp;
            j += 1;
            k -= 1;
        }
    }
}

fn transpose(arr: &mut Vec<Vec<char>>) {
    let n = arr.len();

    for i in 0..n {
        for j in i..n {
            let tmp = arr[i][j];
            arr[i][j] = arr[j][i];
            arr[j][i] = tmp;
        }
    }
}

fn rotate90(arr: &mut Vec<Vec<char>>) {
    transpose(arr);
    reverse_columns(arr);
}

fn flip_columns(arr: &mut Vec<Vec<char>>) {
    let n = arr.len();
    let old = arr.clone();

    for row in 0..n {
        for col in 0..n {
            arr[row][col] = old[row][n - 1 - col];
        }
    }
}

fn flip_rows(arr: &mut Vec<Vec<char>>) {
    let n = arr.len();
    let old = arr.clone();

    for row in 0..n {
        for col in 0..n {
            arr[row][col] = old[n - row - 1][col];
        }
    }
}

pub fn solve() {
    let mut file = File::open("2017/day21/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let start = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];
    let regex = Regex::new(r"([\.#/]+) => ([\.#/]+)").unwrap();
    let mut rule_map = HashMap::new();

    for line in contents.lines() {
        let cap = regex.captures(line).unwrap();
        let from: Vec<Vec<_>> = cap[1].split('/').map(|v| v.chars().collect()).collect();
        let to: Vec<Vec<_>> = cap[2].split('/').map(|v| v.chars().collect()).collect();

        let mut from_rot = from.clone();
        for _ in 0..4 {
            rotate90(&mut from_rot);
            rule_map.insert(from_rot.clone(), to.clone());

            let mut flip_col = from_rot.clone();
            let mut flip_row = from_rot.clone();
            flip_columns(&mut flip_col);
            flip_rows(&mut flip_row);

            rule_map.insert(flip_col, to.clone());
            rule_map.insert(flip_row, to.clone());
        }
    }

    let mut curr_grid = start.clone();

    for ctr in 0..18 {
        let n = curr_grid.len();
        let num_steps;
        let stride;

        if n % 2 == 0 {
            stride = 2;
            num_steps = n / 2;
        } else {
            stride = 3;
            num_steps = n / 3;
        }

        let stride_next = stride + 1;
        let next_size = (stride + 1) * num_steps;

        let mut next_grid = vec![vec![' '; next_size]; next_size];

        for r in 0..num_steps {
            for c in 0..num_steps {
                let mut grid = vec![vec![' '; stride]; stride];
                for i in 0..stride {
                    let row = r * stride + i;
                    for j in 0..stride {
                        let col = c * stride + j;
                        grid[i][j] = curr_grid[row][col];
                    }
                }
                let goal = rule_map.get(&grid).unwrap();

                for i in 0..stride_next {
                    let row = r * stride_next + i;
                    for j in 0..stride_next {
                        let col = c * stride_next + j;
                        next_grid[row][col] = goal[i][j];
                    }
                }
            }
        }

        curr_grid = next_grid;

        let num_set = curr_grid.iter().fold(0, |acc, ref x| {
            acc + x
                .iter()
                .fold(0, |acc0, &x0| acc0 + if x0 == '#' { 1 } else { 0 })
        });

        if ctr == 4 {
            println!("Part one: {}", num_set);
        } else if ctr == 17 {
            println!("Part two: {}", num_set);
        }
    }
}
