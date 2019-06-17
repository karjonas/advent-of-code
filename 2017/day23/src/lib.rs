use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Instruction {
    Set0(usize, i64),
    Set1(usize, usize),
    Sub0(usize, i64),
    Sub1(usize, usize),
    Mul0(usize, i64),
    Mul1(usize, usize),
    Jnz0(usize, i64),
    Jnz1(usize, usize),
    Jmp(i64),
}

fn collect_instructions(cmds: Vec<Vec<String>>) -> Vec<Instruction> {
    let mut instrs = Vec::<Instruction>::new();

    for cmd in cmds.clone() {
        let action = cmd[0].clone();
        match action.as_ref() {
            "set" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Set0(reg_to_index(reg), i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Set1(reg_to_index(reg), reg_to_index(reg1)));
                    }
                }
            }
            "sub" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Sub0(reg_to_index(reg), i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Sub1(reg_to_index(reg), reg_to_index(reg1)));
                    }
                }
            }
            "mul" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Mul0(reg_to_index(reg), i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Mul1(reg_to_index(reg), reg_to_index(reg1)));
                    }
                }
            }
            "jnz" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();
                match val {
                    Ok(i) => {
                        if reg == '1' {
                            instrs.push(Instruction::Jmp(i));
                        } else {
                            instrs.push(Instruction::Jnz0(reg_to_index(reg), i));
                        }
                    }
                    Err(_) => {
                        instrs.push(Instruction::Jnz1(
                            reg_to_index(reg),
                            reg_to_index(cmd[2].chars().nth(0).unwrap()),
                        ));
                    }
                }
            }
            _ => {
                panic!("No match for {}", action);
            }
        }
    }

    return instrs;
}

fn reg_to_index(reg: char) -> usize {
    (reg as u8 - 'a' as u8) as usize
}

fn do_instructions(instrs: &Vec<Instruction>, mul_ctr: &mut usize) -> bool {
    let num_instrs = instrs.len() as i64;
    let mut pc: i64 = 0;
    let mut regs = vec![0 as i64; 8];

    loop {
        if pc < 0 || pc >= num_instrs {
            return false;
        }
        let inst = instrs[pc as usize].clone();
        match inst {
            Instruction::Set0(r, i) => {
                regs[r] = i;
            }
            Instruction::Set1(r0, r1) => {
                let v1 = regs[r1];
                regs[r0] = v1;
            }
            Instruction::Sub0(r, i) => {
                let v = regs[r];
                regs[r] = v - i;
            }
            Instruction::Sub1(r0, r1) => {
                let v0 = regs[r0];
                let v1 = regs[r1];
                regs[r0] = v0 - v1;
            }
            Instruction::Mul0(r, i) => {
                let v = regs[r];
                regs[r] = v * i;
                *mul_ctr += 1;
            }
            Instruction::Mul1(r0, r1) => {
                let v0 = regs[r0];
                let v1 = regs[r1];
                regs[r0] = v0 * v1;
                *mul_ctr += 1;
            }
            Instruction::Jnz0(r0, i) => {
                let v0 = regs[r0];
                if v0 != 0 {
                    pc += i;
                    continue;
                }
            }
            Instruction::Jnz1(r0, r1) => {
                let v0 = regs[r0];
                let v1 = regs[r1];
                if v0 != 0 {
                    pc += v1;
                    continue;
                }
            }
            Instruction::Jmp(i) => {
                pc += i;
                continue;
            }
        }
        pc += 1;
    }
}

fn part_two() -> i64 {
    let mut h: i64 = 0;

    let mut b: i64 = 67 * 100 + 100000;
    let c = b + 17000;

    loop {
        let mut f: i64 = 1;
        let mut d: i64 = 2;
        loop {
            let mut e = 2;
            while e != b {
                if b % d == 0 {
                    f = 0;
                }
                e = b;
            }
            d += 1;
            if d == b {
                break;
            }
        }

        if f == 0 {
            h += 1;
        }
        if b == c {
            break;
        }
        b += 17;
    }

    return h;
}

pub fn solve() {
    let mut file = File::open("2017/day23/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let cmds: Vec<Vec<String>> = contents
        .trim()
        .split('\n')
        .map(|v| v.split(' ').map(|i| i.to_string()).collect())
        .collect();
    let instrs = collect_instructions(cmds.clone());

    {
        let mut mul_ctr = 0;

        do_instructions(&instrs, &mut mul_ctr);

        println!("Part one: {}", mul_ctr);
        println!("Part two: {}", part_two());
    }
}
