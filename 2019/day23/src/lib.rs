extern crate common;
extern crate intcode;

use std::collections::HashSet;

#[derive(Debug)]
struct Computer {
    id: usize,
    memory: Vec<i64>,
    index: usize,
    relative_base: i64,
    input: Vec<i64>,
}

fn init_computers(memory: &Vec<i64>) -> Vec<Computer> {
    let mut computers = Vec::new();
    for i in 0..50 {
        computers.push(Computer {
            id: i,
            memory: memory.clone(),
            index: 0,
            relative_base: 0,
            input: Vec::new(),
        });
    }
    return computers;
}

fn solve_both(memory: Vec<i64>, is_first_problem: bool) -> usize {
    let mut computers = init_computers(&memory);

    // init
    for i in 0..50 {
        let comp = computers.get_mut(i).unwrap();
        let (memory_new, _output_numbers, index, relative_base, _halted) =
            intcode::run(comp.memory.clone(), [comp.id as i64].to_vec(), 0, 0);
        comp.memory = memory_new;
        comp.index = index;
        comp.relative_base = relative_base;
    }

    let mut nat = (0, 0);
    let mut delivered_nat_ys = HashSet::new();

    loop {
        for i in 0..50 {
            let comp = computers.get(i).unwrap();
            let inputs = if comp.input.is_empty() {
                [-1].to_vec()
            } else {
                comp.input.clone()
            };

            for value in inputs {
                let mut comp = computers.get_mut(i).unwrap();
                comp.input.clear();

                let (memory_new, output_numbers, index, relative_base, _halted) = intcode::run(
                    comp.memory.clone(),
                    [value].to_vec(),
                    comp.index,
                    comp.relative_base,
                );

                comp.memory = memory_new;
                comp.index = index;
                comp.relative_base = relative_base;

                if !output_numbers.is_empty() {
                    assert_eq!(output_numbers.len() % 3, 0);

                    for i in 0..(output_numbers.len() / 3) {
                        let start_i = i * 3;
                        let dest = output_numbers[start_i];
                        let x = output_numbers[start_i + 1];
                        let y = output_numbers[start_i + 2];

                        if dest == 255 {
                            if is_first_problem {
                                return y as usize;
                            }

                            nat = (x, y);
                            continue;
                        }

                        let comp_dest = computers.get_mut(dest as usize).unwrap();
                        comp_dest.input.push(x);
                        comp_dest.input.push(y);
                    }
                }
            }
        }

        let mut do_nat = true;
        for i in 0..50 {
            let comp = computers.get(i).unwrap();
            if !comp.input.is_empty() {
                do_nat = false;
                break;
            }
        }

        if do_nat {
            if delivered_nat_ys.contains(&nat.1) {
                return nat.1 as usize;
            }
            delivered_nat_ys.insert(nat.1);
            computers
                .get_mut(0)
                .unwrap()
                .input
                .append(&mut [nat.0, nat.1].to_vec());
        }
    }
}

fn solve_part_one(memory: Vec<i64>) -> usize {
    return solve_both(memory, true);
}
fn solve_part_two(memory: Vec<i64>) -> usize {
    return solve_both(memory, false);
}

pub fn solve() {
    let input = common::read_file("2019/day23/input");
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone()));
    println!("Part two: {}", solve_part_two(memory));
}
