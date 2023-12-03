extern crate regex;

use regex::Regex;

#[derive(Debug, Clone)]
struct Reg {
    value: i32,
    reg: char,
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy { source: Reg, dest: Reg },
    Inc { variable: char },
    Dec { variable: char },
    Jnz { value: Reg, steps: Reg },
    Tgl { variable: char },
}

fn string_to_reg(s: String) -> Reg {
    let mut va = 0;
    let mut re = '0';

    match s.parse::<i32>() {
        Ok(v) => {
            va = v;
        }
        _ => {
            re = s.chars().collect::<Vec<_>>()[0].clone();
        }
    }

    return Reg { value: va, reg: re };
}

fn char_to_idx(c: char) -> usize {
    return (c as u8 - 'a' as u8) as usize;
}

fn parse_instructions(contents: String) -> Vec<Instruction> {
    let regex_cpy = Regex::new(r"cpy (-*\w+) (-*\w+)").unwrap();
    let regex_inc = Regex::new(r"inc (\w+)").unwrap();
    let regex_dec = Regex::new(r"dec (\w+)").unwrap();
    let regex_jnz = Regex::new(r"jnz (-*\w+) (-*\w+)").unwrap();
    let regex_tgl = Regex::new(r"tgl (-*\w+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in contents.lines() {
        let mut matched = false;
        match regex_cpy.captures(line) {
            Some(cap) => {
                let source = string_to_reg(cap[1].to_string());
                let dest = string_to_reg(cap[2].to_string());
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
                let variable = cap[1].to_string().chars().collect::<Vec<_>>()[0].clone();
                instructions.push(Instruction::Inc { variable: variable });
                matched = true;
            }
            None => (),
        }

        match regex_dec.captures(line) {
            Some(cap) => {
                let variable = cap[1].to_string().chars().collect::<Vec<_>>()[0].clone();
                instructions.push(Instruction::Dec { variable: variable });
                matched = true;
            }
            None => (),
        }

        match regex_jnz.captures(line) {
            Some(cap) => {
                let value = string_to_reg(cap[1].to_string());
                let steps = string_to_reg(cap[2].to_string());
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
                let variable = cap[1].to_string().chars().collect::<Vec<_>>()[0].clone();
                instructions.push(Instruction::Tgl { variable: variable });
                matched = true;
            }
            None => (),
        }
        assert!(matched);
    }

    return instructions;
}

fn get_variable(name: char, values: &mut [i32; 4]) -> i32 {
    return values[char_to_idx(name)];
}

fn set_variable(name: char, value: i32, values: &mut [i32; 4]) {
    values[char_to_idx(name)] = value;
}

fn get_variable_reg(reg: Reg, values: &mut [i32; 4]) -> i32 {
    if reg.reg != '0' {
        return values[char_to_idx(reg.reg)];
    }
    return reg.value;
}

fn set_variable_reg(reg: Reg, value: i32, values: &mut [i32; 4]) {
    if reg.reg != '0' {
        values[char_to_idx(reg.reg)] = value;
    }
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
    };

    return ret;
}

fn run_code(instructions_in: &Vec<Instruction>, values: &mut [i32; 4]) {
    let mut instructions = instructions_in.clone();
    let mut i_ptr: i32 = 0;
    while i_ptr < instructions.len() as i32 {
        let instr = instructions[i_ptr as usize].clone();
        match instr {
            Instruction::Cpy {
                ref source,
                ref dest,
            } => {
                let v = get_variable_reg(source.clone(), values);
                set_variable_reg(dest.clone(), v, values);
                i_ptr += 1;
            }
            Instruction::Inc { ref variable } => {
                let v = get_variable(*variable, values);
                set_variable(*variable, v + 1, values);
                i_ptr += 1;
            }
            Instruction::Dec { ref variable } => {
                let v = get_variable(*variable, values);
                set_variable(*variable, v - 1, values);
                i_ptr += 1;
            }
            Instruction::Jnz {
                ref value,
                ref steps,
            } => {
                let v = get_variable_reg(value.clone(), values);
                let s = get_variable_reg(steps.clone(), values);
                if v != 0 {
                    i_ptr = i_ptr + s;
                } else {
                    i_ptr += 1;
                }
            }
            Instruction::Tgl { ref variable } => {
                let v = get_variable(*variable, values);
                let i = (i_ptr + v) as usize;
                if i < instructions.len() {
                    instructions[i] = toggle_instruction(instructions[i].clone());
                }
                i_ptr += 1;
            }
        }
    }
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let instructions = parse_instructions(input);

    {
        let mut values = [0, 0, 0, 0];
        set_variable('a', 7, &mut values);
        run_code(&instructions, &mut values);
        println!("Part 1: {:?}", values[char_to_idx('a')]);
    }
    {
        let mut values = [0, 0, 0, 0];
        set_variable('a', 12, &mut values);
        run_code(&instructions, &mut values);
        println!("Part 2: {:?}", values[char_to_idx('a')]);
    }
}
