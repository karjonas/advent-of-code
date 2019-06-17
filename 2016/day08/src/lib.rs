extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const NROWS: usize = 6;
const NCOLS: usize = 50;

fn set_rect(grid_rc: &mut Vec<Vec<bool>>, cols: usize, rows: usize) {
    for r in 0..rows {
        for c in 0..cols {
            grid_rc[r][c] = true;
        }
    }
}

fn rotate_column(grid_rc: &mut Vec<Vec<bool>>, col: usize, steps: usize) {
    let grid_cpy = grid_rc.clone();
    for r in 0..NROWS {
        grid_rc[(r + steps) % NROWS][col] = grid_cpy[r][col];
    }
}

fn rotate_row(grid_rc: &mut Vec<Vec<bool>>, row: usize, steps: usize) {
    let grid_cpy = grid_rc.clone();
    for c in 0..NCOLS {
        grid_rc[row][(c + steps) % NCOLS] = grid_cpy[row][c];
    }
}

fn print_rec(grid_rc: &Vec<Vec<bool>>) {
    for r in 0..NROWS {
        for c in 0..NCOLS {
            print!("{}", if grid_rc[r][c] { "#" } else { "." });
        }
        println!("");
    }
}

pub fn solve() {
    let mut file = File::open("2016/day08/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut grid_rc = vec![vec![false; NCOLS]; NROWS];

    let regex_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let regex_row = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let regex_col = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();

    for line in contents.lines() {
        match regex_rect.captures(line) {
            Some(cap) => set_rect(
                &mut grid_rc,
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ),
            None => (),
        }

        match regex_row.captures(line) {
            Some(cap) => rotate_column(
                &mut grid_rc,
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ),
            None => (),
        }

        match regex_col.captures(line) {
            Some(cap) => rotate_row(
                &mut grid_rc,
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ),
            None => (),
        }
    }

    let num_set = grid_rc.iter().fold(0, |sum, v| {
        sum + v.iter().fold(0, |sum_v, &i| sum_v + if i { 1 } else { 0 })
    });
    println!("Part 1: {}", num_set);
    println!("Part 2:");
    print_rec(&grid_rc);
}
