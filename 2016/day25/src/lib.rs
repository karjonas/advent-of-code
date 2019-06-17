extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Instruction {
    Cpy { source: String, dest: String },
    Inc { variable: String },
    Dec { variable: String },
    Jnz { value: String, steps: String },
    Tgl { variable: String },
    Out { variable: String },
}

fn parse_instructions(contents: String) -> Vec<Instruction> {
    let regex_cpy = Regex::new(r"cpy (-*\w+) (\w+)").unwrap();
    let regex_inc = Regex::new(r"inc (\w+)").unwrap();
    let regex_dec = Regex::new(r"dec (\w+)").unwrap();
    let regex_jnz = Regex::new(r"jnz (-*\w+) (-*\w+)").unwrap();
    let regex_tgl = Regex::new(r"tgl (-*\w+)").unwrap();
    let regex_out = Regex::new(r"out (-*\w+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in contents.lines() {
        let mut matched = false;
        match regex_cpy.captures(line) {
            Some(cap) => {
                let source = cap[1].to_string();
                let dest = cap[2].to_string();
                instructions.push(Instruction::Cpy {
                    source: source,
                    dest: dest,
                });
                matched = true;
            }
            None => (),
        }

        match regex_inc.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Inc { variable: variable });
                matched = true;
            }
            None => (),
        }

        match regex_dec.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Dec { variable: variable });
                matched = true;
            }
            None => (),
        }
        match regex_jnz.captures(line) {
            Some(cap) => {
                let value = cap[1].to_string();
                let steps = cap[2].to_string();
                instructions.push(Instruction::Jnz {
                    value: value,
                    steps: steps,
                });
                matched = true;
            }
            None => (),
        }
        match regex_tgl.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Tgl { variable: variable });
                matched = true;
            }
            None => (),
        }
        match regex_out.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string();
                instructions.push(Instruction::Out { variable: variable });
                matched = true;
            }
            None => (),
        }
        assert!(matched);
    }

    return instructions;
}

fn get_variable(name: &String, values: &mut HashMap<String, i32>) -> i32 {
    match name.parse::<i32>() {
        Ok(val) => {
            return val;
        }
        Err(_) => (),
    }

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

fn toggle_instruction(inst: Instruction) -> Instruction {
    let ret = match inst {
        Instruction::Cpy {
            ref source,
            ref dest,
        } => Instruction::Jnz {
            value: source.clone(),
            steps: dest.clone(),
        },
        Instruction::Inc { ref variable } => Instruction::Dec {
            variable: variable.clone(),
        },
        Instruction::Dec { ref variable } => Instruction::Inc {
            variable: variable.clone(),
        },
        Instruction::Jnz {
            ref value,
            ref steps,
        } => Instruction::Cpy {
            source: value.clone(),
            dest: steps.clone(),
        },
        Instruction::Tgl { ref variable } => Instruction::Inc {
            variable: variable.clone(),
        },
        Instruction::Out { ref variable } => Instruction::Out {
            variable: variable.clone(),
        },
    };

    return ret;
}

fn run_code(instructions_in: &Vec<Instruction>, values: &mut HashMap<String, i32>) -> usize {
    let mut output: Vec<i32> = Vec::new();
    let mut tried_a = 0;
    let mut instructions = instructions_in.clone();
    let mut i_ptr: i32 = 0;

    while i_ptr < instructions.len() as i32 {
        let instr = instructions[i_ptr as usize].clone();
        let mut found = false;
        match instr {
            Instruction::Cpy {
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
            Instruction::Jnz {
                ref value,
                ref steps,
            } => {
                let v = get_variable(value, values);
                let s = get_variable(steps, values);
                if v != 0 {
                    i_ptr = i_ptr + s;
                } else {
                    i_ptr += 1;
                }
            }
            Instruction::Tgl { ref variable } => {
                let v = get_variable(variable, values);
                let i = (i_ptr + v) as usize;
                if i < instructions.len() {
                    instructions[i] = toggle_instruction(instructions[i].clone());
                }
                i_ptr += 1;
            }
            Instruction::Out { ref variable } => {
                let v = get_variable(variable, values);
                output.push(v);
                if output.len() > 10 {
                    let mut ok = true;
                    for i in 0..output.len() - 1 {
                        if output[i] == output[i + 1] {
                            ok = false;
                        }
                    }

                    if ok {
                        found = true;
                    } else {
                        tried_a += 1;
                        output.clear();
                        values.clear();
                        set_variable(&"a".to_string(), tried_a as i32, values);
                        i_ptr = 0;
                    }
                } else {
                    i_ptr += 1;
                }
            }
        }

        if found {
            break;
        }
    }
    return tried_a;
}

pub fn solve() {
    let mut file = File::open("2016/day25/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let instructions = parse_instructions(contents);

    {
        let mut values: HashMap<String, i32> = HashMap::new();
        let ret = run_code(&instructions, &mut values);
        println!("Part 1: {:?}", ret);
    }
}
