use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn match_recursive(
    match_val: usize,
    vals: &Vec<Vec<usize>>,
    connections: &HashMap<usize, Vec<usize>>,
    used: &Vec<bool>,
    maximize_length: bool,
) -> (usize, usize) {
    let mut best_strength = 0;
    let mut best_length = 0;
    for i in connections.get(&match_val).unwrap() {
        let idx = i.clone();

        if used[idx] {
            continue;
        }

        let v0 = vals[idx][0];
        let v1 = vals[idx][1];

        let mut strength0 = 0;
        let mut strength1 = 0;

        let mut length0 = 0;
        let mut length1 = 0;

        if v0 == match_val {
            let mut used_next = used.clone();
            used_next[idx] = true;
            let res = match_recursive(v1, &vals, &connections, &used_next, maximize_length);
            strength0 = res.0;
            length0 = res.1;
        }
        if v1 == match_val {
            let mut used_next = used.clone();
            used_next[idx] = true;
            let res = match_recursive(v0, &vals, &connections, &used_next, maximize_length);
            strength1 = res.0;
            length1 = res.1;
        }

        if maximize_length {
            let mut best_strength_curr = 0;
            let mut best_length_curr = std::cmp::max(length0, length1);
            if length0 == best_length_curr {
                best_strength_curr = std::cmp::max(best_strength_curr, strength0);
            }
            if length1 == best_length_curr {
                best_strength_curr = std::cmp::max(best_strength_curr, strength1);
            }

            best_strength_curr += v0 + v1;
            best_length_curr += 1;

            if best_length == best_length_curr {
                best_strength = std::cmp::max(best_strength, best_strength_curr);
            } else if best_length < best_length_curr {
                best_strength = best_strength_curr;
                best_length = best_length_curr;
            }
        } else {
            best_strength = std::cmp::max(best_strength, v0 + v1 + strength0);
            best_strength = std::cmp::max(best_strength, v0 + v1 + strength1);
        }
    }
    return (best_strength, best_length);
}
pub fn solve() {
    let mut file = File::open("2017/day24/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let vals: Vec<Vec<usize>> = contents
        .split('\n')
        .map(|l| l.split('/').map(|v| v.parse::<usize>().unwrap()).collect())
        .collect();

    let mut connections = HashMap::new();

    for idx in 0..vals.len() {
        connections
            .entry(vals[idx][0])
            .or_insert(Vec::<usize>::new())
            .push(idx);
        connections
            .entry(vals[idx][1])
            .or_insert(Vec::<usize>::new())
            .push(idx);
    }

    let used = vec![false; vals.len()];
    let best_first = match_recursive(0, &vals, &connections, &used, false);
    let best_second = match_recursive(0, &vals, &connections, &used, true);

    println!("Part one: {:?}", best_first.0);
    println!("Part two: {:?}", best_second.0);
}
