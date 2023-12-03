extern crate regex;
use regex::Regex;

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

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let mut grid_rc = vec![vec![false; NCOLS]; NROWS];

    let regex_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let regex_row = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
    let regex_col = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();

    for line in input.lines() {
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
