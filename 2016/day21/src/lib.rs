extern crate regex;

use regex::Regex;

#[derive(Debug, Clone)]
enum Instruction {
    SwapPosition { first: usize, second: usize },
    SwapLetter { first: char, second: char },
    RotateLeft { steps: usize },
    RotateRight { steps: usize },
    RotateLetter { letter: char },
    Reverse { start: usize, end: usize },
    Move { from: usize, to: usize },
}

fn parse_instructions(contents: String) -> Vec<Instruction> {
    let regex_swap_pos = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let regex_swap_let = Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
    let regex_rot_left = Regex::new(r"rotate left (\d+) step(s*)").unwrap();
    let regex_rot_right = Regex::new(r"rotate right (\d+) step(s*)").unwrap();
    let regex_rot_letter = Regex::new(r"rotate based on position of letter ([a-z0-9])").unwrap();
    let regex_reverese = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let regex_move = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in contents.lines() {
        let mut matched = false;
        match regex_swap_pos.captures(line) {
            Some(cap) => {
                let first = cap[1].parse::<usize>().unwrap();
                let second = cap[2].parse::<usize>().unwrap();
                matched = true;
                instructions.push(Instruction::SwapPosition {
                    first: first,
                    second: second,
                });
            }
            None => (),
        }
        match regex_swap_let.captures(line) {
            Some(cap) => {
                let first = cap[1].chars().nth(0).unwrap();
                let second = cap[2].chars().nth(0).unwrap();
                matched = true;
                instructions.push(Instruction::SwapLetter {
                    first: first,
                    second: second,
                });
            }
            None => (),
        }
        match regex_rot_left.captures(line) {
            Some(cap) => {
                let steps = cap[1].parse::<usize>().unwrap();
                matched = true;
                instructions.push(Instruction::RotateLeft { steps: steps });
            }
            None => (),
        }
        match regex_rot_right.captures(line) {
            Some(cap) => {
                let steps = cap[1].parse::<usize>().unwrap();
                matched = true;
                instructions.push(Instruction::RotateRight { steps: steps });
            }
            None => (),
        }
        match regex_rot_letter.captures(line) {
            Some(cap) => {
                let letter = cap[1].chars().nth(0).unwrap();
                matched = true;
                instructions.push(Instruction::RotateLetter { letter: letter });
            }
            None => (),
        }
        match regex_reverese.captures(line) {
            Some(cap) => {
                let start = cap[1].parse::<usize>().unwrap();
                let end = cap[2].parse::<usize>().unwrap();
                matched = true;
                instructions.push(Instruction::Reverse {
                    start: start,
                    end: end,
                });
            }
            None => (),
        }
        match regex_move.captures(line) {
            Some(cap) => {
                let from = cap[1].parse::<usize>().unwrap();
                let to = cap[2].parse::<usize>().unwrap();
                matched = true;
                instructions.push(Instruction::Move { from: from, to: to });
            }
            None => (),
        }
        assert!(matched);
    }

    return instructions;
}

fn solve_internal(input: &str, instructions_inp: &Vec<Instruction>, reverse: bool) -> String {
    let mut instructions = instructions_inp.clone();
    if reverse {
        instructions.reverse();
    }

    let mut curr_str: Vec<char> = input.to_string().chars().collect();
    let num_chars = curr_str.len();

    let rotate_steps = |curr_str: &mut Vec<char>, steps: usize| {
        let last_str = curr_str.clone();
        for from in 0..num_chars {
            let dest = (from + steps) % num_chars;
            curr_str[dest] = last_str[from];
        }
    };

    for instr in instructions {
        match instr.clone() {
            Instruction::SwapPosition { first, second } => {
                let tmp = curr_str[second];
                curr_str[second] = curr_str[first];
                curr_str[first] = tmp;
            }
            Instruction::SwapLetter { first, second } => {
                for i in 0..num_chars {
                    if curr_str[i] == first {
                        curr_str[i] = second;
                    } else if curr_str[i] == second {
                        curr_str[i] = first;
                    }
                }
            }
            Instruction::RotateLeft { steps } => {
                let steps_i = if reverse { steps } else { num_chars - steps };
                rotate_steps(&mut curr_str, steps_i);
            }
            Instruction::RotateRight { steps } => {
                let steps_i = if reverse { num_chars - steps } else { steps };
                rotate_steps(&mut curr_str, steps_i);
            }
            Instruction::RotateLetter { letter } => {
                let rotate_letter = |mut in_str: &mut Vec<char>| {
                    let pos = in_str.iter().position(|&c| c == letter).unwrap();
                    let steps = if pos >= 4 { pos + 2 } else { pos + 1 };
                    rotate_steps(&mut in_str, steps);
                };

                if reverse {
                    for steps in 0..num_chars {
                        let mut str_test = curr_str.clone();
                        rotate_steps(&mut str_test, steps);
                        rotate_letter(&mut str_test);
                        if str_test == curr_str {
                            rotate_steps(&mut curr_str, steps);
                            break;
                        }
                    }
                } else {
                    rotate_letter(&mut curr_str);
                }
            }
            Instruction::Reverse { start, end } => {
                let last_str = curr_str.clone();
                for i in 0..(1 + end - start) {
                    curr_str[start + i] = last_str[end - i];
                }
            }
            Instruction::Move { from, to } => {
                let from_i = if reverse { to } else { from };
                let to_i = if reverse { from } else { to };
                let from_c = curr_str[from_i];

                curr_str.remove(from_i);
                curr_str.insert(to_i, from_c);
            }
        }
    }

    let mut out = String::new();
    for c in curr_str {
        out.push(c);
    }
    return out;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let instructions = parse_instructions(input);
    let res = solve_internal("abcdefgh", &instructions, false);
    let res2 = solve_internal("fbgdceah", &instructions, true);
    println!("Part 1: {:?}", res);
    println!("Part 2: {:?}", res2);
}
