use std::fs::File;
use std::io::prelude::*;

const W: usize = 35;
const H: usize = 29;

#[derive(Debug, Copy, Clone)]
struct Node {
    used: usize,
    avail: usize,
}

struct State {
    arr: [[Node; H]; W],
    dist: [[usize; H]; W],
}

fn parse_solve(path: &str) -> (usize, usize) {
    let mut out = State {
        arr: [[Node { used: 0, avail: 0 }; H]; W],
        dist: [[std::usize::MAX; H]; W],
    };
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut ctr = 0;
    let mut limit_available = 0;
    let mut start_pos_cr = (0, 0);

    for line in contents.lines().skip(2) {
        let f = line.to_string();
        let lol: Vec<String> = f
            .split_whitespace()
            .map(|v| v.chars().filter(|&c| c != 'T').collect())
            .collect();

        let node = Node {
            used: lol[2].parse::<usize>().unwrap(),
            avail: lol[3].parse::<usize>().unwrap(),
        };

        let row = ctr % H;
        let col = ctr / H;

        if node.used == 0 {
            limit_available = node.avail;
            start_pos_cr = (col, row);
        }
        out.arr[col][row] = node;
        ctr += 1;
    }

    let mut pairs = 0;
    for r0 in 0..H {
        for c0 in 0..W {
            let mut num_hits_c = 0;
            let v0 = out.arr[c0][r0];
            for r1 in 0..H {
                for c1 in 0..W {
                    let v1 = out.arr[c1][r1];
                    if v0.avail >= v1.used && v1.used > 0 {
                        num_hits_c += 1;
                    }
                }
            }

            pairs += num_hits_c;
        }
    }

    let mut stack: Vec<(usize, (usize, usize))> = Vec::new();
    stack.push((0, start_pos_cr));

    while !stack.is_empty() {
        let (steps, (c, r)) = stack.pop().unwrap();

        if out.arr[c][r].used > limit_available || out.dist[c][r] < std::usize::MAX {
            continue;
        }

        out.dist[c][r] = steps;

        if (c + 1) < W {
            stack.push((steps + 1, (c + 1, r)));
        }
        if c > 0 {
            stack.push((steps + 1, (c - 1, r)));
        }

        if (r + 1) < H {
            stack.push((steps + 1, (c, r + 1)));
        }
        if r > 0 {
            stack.push((steps + 1, (c, r - 1)));
        }

        stack.sort_by(|a, b| b.0.cmp(&a.0));
    }

    // number of moves to get from empy space to the data and nudge it
    let dist_data = out.dist[W - 1][0];

    // It takes 5 steps to move data one step closer to goal, and we are W-2 steps away
    let moves = dist_data + (W - 2) * 5;

    return (pairs, moves);
}

pub fn solve() {
    let values = parse_solve("2016/day22/input");
    println!("Part 1: {}", values.0);
    println!("Part 2: {}", values.1);
}
