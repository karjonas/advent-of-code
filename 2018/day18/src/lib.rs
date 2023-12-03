extern crate common;
use std::mem;
const GRID_SIZE: usize = 50;

fn parse_input(input: &String) -> Vec<char> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut ret = Vec::new();
    for line in lines {
        let mut line_chars = line.chars().collect::<Vec<char>>();
        ret.append(&mut line_chars);
    }
    return ret;
}

fn pos_to_idx(x: usize, y: usize) -> usize {
    return y * GRID_SIZE + x;
}

fn print_grid(grid: &Vec<char>) {
    for y in 0..GRID_SIZE {
        let mut s = String::new();
        for x in 0..GRID_SIZE {
            s.push(grid[pos_to_idx(x, y)]);
        }
        println!("{}", s);
    }
}

fn next_object(pos: (usize, usize), grid: &Vec<char>) -> char {
    let x = pos.0 as i64;
    let y = pos.1 as i64;
    let width = GRID_SIZE as i64;
    let height = GRID_SIZE as i64;

    let mut neighs = Vec::new();
    neighs.reserve(8);

    let possibles = [
        (x, y - 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
    ];

    for (x, y) in possibles.iter() {
        if *x < width && *x >= 0 && *y < height && *y >= 0 {
            neighs.push(grid[pos_to_idx(*x as usize, *y as usize)])
        }
    }

    // open ground (.)
    // trees (|)
    // lumberyard (#)

    let num_trees = neighs
        .iter()
        .fold(0, |sum, n| sum + if *n == '|' { 1 } else { 0 });
    let num_lumber = neighs
        .iter()
        .fold(0, |sum, n| sum + if *n == '#' { 1 } else { 0 });

    let curr = grid[pos_to_idx(x as usize, y as usize)];
    let mut next_curr = curr;

    if curr == '.' && num_trees >= 3 {
        next_curr = '|'
    }
    if curr == '|' && num_lumber >= 3 {
        next_curr = '#'
    }
    if curr == '#' {
        if num_trees >= 1 && num_lumber >= 1 {
            next_curr = '#'
        } else {
            next_curr = '.'
        }
    }

    return next_curr;
}

fn run(grid: &Vec<char>, grid_next: &mut Vec<char>) {
    let width = GRID_SIZE;
    let height = GRID_SIZE;

    for y in 0..height {
        for x in 0..width {
            grid_next[pos_to_idx(x, y)] = next_object((x, y), &grid);
        }
    }
}

fn calc_value(grid: &Vec<char>) -> usize {
    let mut num_trees = 0;
    let mut num_lumber = 0;
    for c in grid {
        if *c == '#' {
            num_lumber += 1
        }
        if *c == '|' {
            num_trees += 1
        }
    }

    return num_trees * num_lumber;
}

fn grid_equal(a: &Vec<char>, b: &Vec<char>) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    return true;
}

fn solve_internal(num_iters_in: usize, input: &String) -> usize {
    let grid = parse_input(input);
    let mut num_iters = num_iters_in;

    let mut grid_next = grid.clone();
    let mut grid_curr = grid.clone();

    let mut previous_grid: Vec<Vec<char>> = Vec::new();
    let mut ctr = 0;
    while ctr < num_iters {
        mem::swap(&mut grid_curr, &mut grid_next);
        run(&grid_curr, &mut grid_next);

        let mut loop_found = false;
        for j in 0..previous_grid.len() {
            if grid_equal(&grid_curr, &previous_grid[j]) {
                let repeat = ctr - j;
                num_iters = (num_iters - j) % repeat;
                grid_next = previous_grid[j].clone();
                previous_grid.clear();
                ctr = 0;
                loop_found = true;
                break;
            }
        }
        if !loop_found {
            previous_grid.push(grid_curr.clone());
            ctr += 1;
        }
    }

    if false {
        print_grid(&grid)
    }

    return calc_value(&grid_next);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", solve_internal(10, &input));
    println!("Part two: {}", solve_internal(1000000000, &input));
}
