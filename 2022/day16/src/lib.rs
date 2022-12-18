extern crate common;
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;

use scan_fmt::scan_fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct State {
    minute: (usize, usize),
    sum: usize,
    position: (usize, usize), // human, elephant
    open: u64,                // each set bit represents valve open, 0 = "AA"
}

fn parse(input: &String) -> (HashMap<(usize, usize), usize>, Vec<usize>) {
    let mut flows = HashMap::new();
    let mut sources = Vec::new();

    let mut neighs = HashMap::new();
    let mut costs: HashMap<(String, String), usize> = HashMap::new();

    for line in input.lines() {
        let (valve, rate) = scan_fmt!(line, "Valve {} has flow rate={};", String, usize).unwrap();
        let valves: Vec<String> = line
            .replace("valves", "valve")
            .replace(",", "")
            .split("valve ")
            .collect::<Vec<_>>()[1]
            .split_ascii_whitespace()
            .map(|v| v.to_string())
            .collect();

        if rate > 0 {
            flows.insert(valve.clone(), rate);
        }

        sources.push(valve.clone());

        let neighs_valve = neighs.entry(valve.clone()).or_insert(HashSet::new());
        for valve_other in &valves {
            costs.insert((valve.clone(), valve_other.clone()), 1);
            costs.insert((valve_other.clone(), valve.clone()), 1); // pointless?
            neighs_valve.insert(valve_other.clone());
        }
    }

    loop {
        let prev_len = costs.len();
        for source in &sources {
            let valves = neighs.get(source).unwrap().clone();
            for valve in valves {
                let base_dist = *costs.get(&(source.clone(), valve.clone())).unwrap();
                for other in neighs.get(&valve).unwrap().clone() {
                    if other == *source {
                        continue;
                    }
                    let dist = *costs.get(&(valve.clone(), other.clone())).unwrap() + base_dist;
                    let dist_new = costs.entry((source.clone(), other.clone())).or_insert(dist);
                    *dist_new = std::cmp::min(dist, *dist_new);
                    neighs.get_mut(source).unwrap().insert(other.clone());
                }
            }
        }

        if prev_len == costs.len() {
            break;
        }
    }

    let mut sources_new = Vec::new();
    let mut costs_new = HashMap::new();

    sources_new.push("AA".to_string());
    for source in sources {
        if flows.get(&source) != None {
            sources_new.push(source);
        }
    }

    for i in 0..sources_new.len() {
        for j in i + 1..sources_new.len() {
            let key = (sources_new[i].clone(), sources_new[j].clone());
            let key_inv = (key.1.clone(), key.0.clone());
            let cost = *costs.get(&key).unwrap();
            costs_new.insert(key, cost);
            if i != 0 {
                // skip "AA" backtrack
                costs_new.insert(key_inv, cost);
            }
        }
    }

    let mut valve_ctr = 0;
    let mut indices = HashMap::new();
    indices.insert("AA".to_string(), 0);
    let mut get_valve_idx = |valve: String| -> usize {
        let valve_idx = *indices.entry(valve).or_insert(valve_ctr + 1);
        valve_ctr = std::cmp::max(valve_idx, valve_ctr);
        return valve_idx;
    };

    let mut costs_final: HashMap<(usize, usize), usize> = HashMap::new();
    let mut flows_final = vec![0; sources_new.len()];

    for i in 0..sources_new.len() {
        let idx_i = get_valve_idx(sources_new[i].clone());
        flows_final[idx_i] = *flows.get(&sources_new[i]).unwrap_or(&0);
        for j in i + 1..sources_new.len() {
            let idx_j = get_valve_idx(sources_new[j].clone());
            let key = (sources_new[i].clone(), sources_new[j].clone());
            let cost = *costs.get(&key).unwrap();

            costs_final.insert((idx_i, idx_j), cost);
            costs_final.insert((idx_j, idx_i), cost);
        }
    }

    return (costs_final, flows_final);
}

fn part_both(input: &String, part_two: bool) -> usize {
    let (costs, flows) = parse(input);
    let mut visited = HashMap::new();
    let mut stack = Vec::new();
    stack.push(State {
        minute: (0, 0),
        sum: 0,
        position: (0, 0),
        open: 0,
    });

    let num_minutes = if part_two { 26 } else { 30 };
    let num_valves = flows.len();

    let mut best = 0;
    while !stack.is_empty() {
        let curr = stack.pop().unwrap();
        best = std::cmp::max(curr.sum, best);

        let key = (curr.position.clone(), curr.open.clone());
        if *visited.entry(key).or_insert(curr.sum) > curr.sum {
            continue;
        }

        // move to neighbour
        for neigh in 0..num_valves {
            let is_open = curr.open & (1 << neigh) != 0;

            // skip start, self, and opened
            if neigh == 0 || neigh == curr.position.0 || is_open {
                continue;
            }

            let minutes = costs
                .get(&(curr.position.0.clone(), neigh.clone()))
                .unwrap()
                + 1;
            if curr.minute.0 + minutes > num_minutes {
                continue;
            }

            let mut next = curr.clone();
            next.minute.0 += minutes;
            next.position.0 = neigh.clone();
            next.sum += flows.get(neigh).unwrap() * (num_minutes - next.minute.0);
            next.open |= 1 << next.position.0;

            if part_two && next.minute.1 < next.minute.0 {
                std::mem::swap(&mut next.minute.0, &mut next.minute.1);
                std::mem::swap(&mut next.position.0, &mut next.position.1);
            }

            stack.push(next);
        }
    }

    best
}

fn part_one(input: &String) -> usize {
    part_both(input, false)
}

fn part_two(input: &String) -> usize {
    part_both(input, true)
}

pub fn solve() {
    let input = common::read_file("2022/day16/input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            .to_string();
        assert_eq!(part_one(&input), 1651);
        assert_eq!(part_two(&input), 1707);
    }
}
