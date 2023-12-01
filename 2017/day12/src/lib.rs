use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day12/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents = contents.chars().filter(|v| v.clone() != ',').collect();

    let lines: Vec<Vec<_>> = contents
        .split('\n')
        .map(|line| {
            line.split(' ')
                .filter(|v| *v != "<->")
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut connections = HashMap::new();
    let mut all_numbers = Vec::<usize>::new();

    for line in lines {
        let curr_program = line[0];
        let num_progs = line.len();

        let ref mut cons = connections
            .entry(curr_program)
            .or_insert(HashSet::<usize>::new());

        for i in 1..num_progs {
            cons.insert(line[i]);
        }

        all_numbers.push(curr_program);
    }

    let mut visited = HashSet::<usize>::new();
    let mut islands = Vec::<Vec<usize>>::new();

    let mut island_idx = 0;
    while all_numbers.len() > 0 {
        let start_number = all_numbers.pop().unwrap();
        if visited.contains(&start_number) {
            continue;
        }
        let mut stack = Vec::<usize>::new();
        stack.push(start_number);
        islands.push(Vec::new());

        while stack.len() > 0 {
            let curr_program = stack.pop().unwrap();
            if visited.contains(&curr_program) {
                continue;
            }

            islands[island_idx].push(curr_program);
            visited.insert(curr_program);
            let cons = connections.get(&curr_program).unwrap();
            for v in cons {
                stack.push(v.clone());
            }
        }

        island_idx += 1;
    }

    let mut num_progs_in_island = 0;
    for island in &islands {
        let goal = 0;
        if island.contains(&goal) {
            num_progs_in_island = island.len();
            break;
        }
    }

    println!("Part one: {}", num_progs_in_island);
    println!("Part two: {}", islands.len());
}
