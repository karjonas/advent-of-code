use std::fs::File;
use std::io::prelude::*;

fn solve_internal(path: &str) -> (usize, usize) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let values: Vec<Vec<usize>> = contents
        .lines()
        .map(|l| {
            l.split(|c| c == '-')
                .map(|w| w.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for vecs in values {
        ranges.push((vecs[0], vecs[1]));
    }

    ranges.dedup();
    ranges.sort_by(|&p0, &p1| p0.0.cmp(&p1.0));

    let mut ctr = 0;
    while ctr < ranges.len() - 1 {
        if ranges[ctr + 1].0 <= ranges[ctr].1 {
            ranges[ctr].1 = std::cmp::max(ranges[ctr + 1].1, ranges[ctr].1);
            ranges.remove(ctr + 1);
        } else {
            ctr += 1;
        }
    }

    let mut smallest = std::usize::MAX;
    for i in 0..ranges.len() {
        let min_i = if ranges[i].0 == 0 { 0 } else { ranges[i].0 - 1 };
        let max_i = ranges[i].1 + 1;

        let mut ok_min = true;
        let mut ok_max = true;
        for j in 0..ranges.len() {
            let r = ranges[j];
            if min_i >= r.0 && min_i <= r.1 {
                ok_min = false;
            }

            if max_i >= r.0 && max_i <= r.1 {
                ok_max = false;
            }
        }

        if ok_min {
            smallest = std::cmp::min(smallest, min_i);
        }
        if ok_max {
            smallest = std::cmp::min(smallest, max_i);
        }
    }

    let mut curr_pos = 0;
    let mut num_hit: usize = 0;
    for i in 0..ranges.len() {
        if curr_pos < ranges[i].0 {
            num_hit += ranges[i].0 - curr_pos;
        }
        curr_pos = ranges[i].1 + 1;
    }

    return (smallest, num_hit);
}

pub fn solve() {
    let res = solve_internal("2016/day20/input");
    println!("Part 1: {}", res.0);
    println!("Part 2: {}", res.1);
}
