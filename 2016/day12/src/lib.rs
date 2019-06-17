extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Instruction {
    CpyVal { value: i32, dest: String },
    CpyVar { source: String, dest: String },
    Inc { variable: String },
    Dec { variable: String },
    JnzVar { variable: String, steps: i32 },
    JnzVal { value: i32, steps: i32 },
}

fn parse_instructions(contents: String) -> Vec<Instruction> {
    let regex_cpy_val = Regex::new(r"cpy (-*\d+) (\w+)").unwrap();
    let regex_cpy_var = Regex::new(r"cpy ([a-z]+) (\w+)").unwrap();
    let regex_inc = Regex::new(r"inc (\w+)").unwrap();
    let regex_dec = Regex::new(r"dec (\w+)").unwrap();
    let regex_jnz_var = Regex::new(r"jnz ([a-z]) (-*\w+)").unwrap();
    let regex_jnz_val = Regex::new(r"jnz (-*\d+) (-*\w+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in contents.lines() {
        match regex_cpy_val.captures(line) {
            Some(cap) => {
                let value = cap[1].parse::<i32>().unwrap();
                let dest = cap[2].to_string();
                instructions.push(Instruction::CpyVal {
                    value: value,
                    dest: dest,
                });
            }
            None => (),
        }

        match regex_cpy_var.captures(line) {
            Some(cap) => {
                let source = cap[1].to_string();
                let dest = cap[2].to_string();
                instructions.push(Instruction::CpyVar {
                    source: source,
                    dest: dest,
                });
            }
            None => (),
        }

        match regex_inc.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Inc { variable: variable });
            }
            None => (),
        }

        match regex_dec.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Dec { variable: variable });
            }
            None => (),
        }

        match regex_jnz_var.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                let steps = cap[2].parse::<i32>().unwrap();
                instructions.push(Instruction::JnzVar {
                    variable: variable,
                    steps: steps,
                });
            }
            None => (),
        }
        match regex_jnz_val.captures(line) {
            Some(cap) => {
                let value = cap[1].parse::<i32>().unwrap();
                let steps = cap[2].parse::<i32>().unwrap();
                instructions.push(Instruction::JnzVal {
                    value: value,
                    steps: steps,
                });
            }
            None => (),
        }
    }

    return instructions;
}

fn get_variable(name: &String, values: &mut HashMap<String, i32>) -> i32 {
    let v = match values.get(name) {
        Some(v) => v.clone(),
        None => 0,
    };

    values.insert(name.clone(), v);
    return v;
}

fn set_variable(name: &String, value: i32, values: &mut HashMap<String, i32>) {
    values.insert(name.clone(), value);
}

fn run_code(instructions: &Vec<Instruction>, values: &mut HashMap<String, i32>) {
    let mut i_ptr: i32 = 0;
    while i_ptr < instructions.len() as i32 {
        match instructions[i_ptr as usize] {
            Instruction::CpyVal {
                ref value,
                ref dest,
            } => {
                set_variable(dest, value.clone(), values);
                i_ptr += 1;
            }
            Instruction::CpyVar {
                ref source,
                ref dest,
            } => {
                let v = get_variable(source, values);
                set_variable(dest, v, values);
                i_ptr += 1;
            }
            Instruction::Inc { ref variable } => {
                let v = get_variable(variable, values);
                set_variable(variable, v + 1, values);
                i_ptr += 1;
            }
            Instruction::Dec { ref variable } => {
                let v = get_variable(variable, values);
                set_variable(variable, v - 1, values);
                i_ptr += 1;
            }
            Instruction::JnzVar {
                ref variable,
                ref steps,
            } => {
                let v = get_variable(variable, values);
                if v != 0 {
                    i_ptr = i_ptr + steps.clone();
                } else {
                    i_ptr += 1;
                }
            }
            Instruction::JnzVal {
                ref value,
                ref steps,
            } => {
                if value.clone() != 0 {
                    i_ptr = i_ptr + steps.clone();
                } else {
                    i_ptr += 1;
                }
            }
        }
    }
}

pub fn solve() {
    let mut file = File::open("2016/day12/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let instructions = parse_instructions(contents);

    {
        let mut values: HashMap<String, i32> = HashMap::new();
        run_code(&instructions, &mut values);
        println!("Part 1: {:?}", values.get("a").unwrap());
    }
    {
        let mut values: HashMap<String, i32> = HashMap::new();
        set_variable(&"c".to_string(), 1, &mut values);
        run_code(&instructions, &mut values);
        println!("Part 2: {:?}", values.get("a").unwrap());
    }
}
