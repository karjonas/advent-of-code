extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct Disc {
    start_pos: usize,
    num_pos: usize,
}

fn parse_input(contents: String) -> Vec<Disc> {
    let regex =
        Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)").unwrap();
    let mut discs: Vec<Disc> = Vec::new();

    for line in contents.lines() {
        match regex.captures(line) {
            Some(cap) => {
                let num_pos = cap[2].parse::<usize>().unwrap();
                let start_pos = cap[3].parse::<usize>().unwrap();
                discs.push(Disc {
                    start_pos: start_pos,
                    num_pos: num_pos,
                });
            }
            None => (),
        }
    }

    return discs;
}

fn discs_solved(time: usize, discs: &Vec<Disc>) -> bool {
    for i in 0..discs.len() {
        let p = (discs[i].start_pos + time) % discs[i].num_pos;
        let goal_p = discs[i].num_pos - (i % discs[i].num_pos) - 1;
        if p != goal_p {
            return false;
        }
    }

    return true;
}

fn solve_internal(input: String) -> usize {
    let discs = parse_input(input);

    for i in 0.. {
        if discs_solved(i, &discs) {
            return i;
        }
    }
    return 0;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part 1: {:?}", solve_internal(input.clone()));
    println!("Part 2: {:?}", solve_internal(input));
}
