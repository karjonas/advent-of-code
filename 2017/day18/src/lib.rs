use std::collections::VecDeque;

#[derive(Debug)]
enum Instruction {
    Snd(char),
    Set0(char, i64),
    Set1(char, char),
    Add0(char, i64),
    Add1(char, char),
    Mul0(char, i64),
    Mul1(char, char),
    Mod0(char, i64),
    Mod1(char, char),
    Rcv(char),
    Jgz0(char, i64),
    Jgz1(char, char),
    Jmp(i64),
}

fn collect_instructions(cmds: Vec<Vec<String>>) -> Vec<Instruction> {
    let mut instrs = Vec::<Instruction>::new();

    for cmd in cmds.clone() {
        let action = cmd[0].clone();
        match action.as_ref() {
            "snd" => {
                assert!(cmd.len() == 2);
                let reg = cmd[1].chars().nth(0).unwrap();
                instrs.push(Instruction::Snd(reg));
            }
            "set" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Set0(reg, i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Set1(reg, reg1));
                    }
                }
            }
            "add" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Add0(reg, i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Add1(reg, reg1));
                    }
                }
            }
            "mul" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Mul0(reg, i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Mul1(reg, reg1));
                    }
                }
            }
            "mod" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();

                match val {
                    Ok(i) => {
                        instrs.push(Instruction::Mod0(reg, i));
                    }
                    Err(_) => {
                        let reg1 = cmd[2].chars().nth(0).unwrap();
                        instrs.push(Instruction::Mod1(reg, reg1));
                    }
                }
            }
            "rcv" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                instrs.push(Instruction::Rcv(reg));
            }
            "jgz" => {
                assert!(cmd[1].len() == 1);
                let reg = cmd[1].chars().nth(0).unwrap();
                let val = cmd[2].parse::<i64>();
                match val {
                    Ok(i) => {
                        if reg == '1' {
                            instrs.push(Instruction::Jmp(i));
                        } else {
                            instrs.push(Instruction::Jgz0(reg, i));
                        }
                    }
                    Err(_) => {
                        instrs.push(Instruction::Jgz1(reg, cmd[2].chars().nth(0).unwrap()));
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

fn get_reg(reg: char, regs: &Vec<i64>) -> i64 {
    return regs[(reg as u8 - 'a' as u8) as usize];
}

fn set_reg(reg: char, val: i64, regs: &mut Vec<i64>) {
    regs[(reg as u8 - 'a' as u8) as usize] = val;
}

fn do_instruction(
    pc: &mut i64,
    mut regs: &mut Vec<i64>,
    instrs: &Vec<Instruction>,
    send_queue: &mut VecDeque<i64>,
    recv_queue: &mut VecDeque<i64>,
    send_ctr: &mut usize,
) -> bool {
    let num_instrs = instrs.len() as i64;

    if *pc < 0 || *pc >= num_instrs {
        return false;
    }

    let inst = &instrs[*pc as usize];
    match inst {
        &Instruction::Snd(r) => {
            let v = get_reg(r, &regs);
            send_queue.push_back(v);
            *send_ctr += 1;
        }
        &Instruction::Set0(r, i) => {
            set_reg(r, i, &mut regs);
        }
        &Instruction::Set1(r0, r1) => {
            let v1 = get_reg(r1, &regs);
            set_reg(r0, v1, &mut regs);
        }
        &Instruction::Add0(r, i) => {
            let v = get_reg(r, &regs);
            set_reg(r, v + i, &mut regs);
        }
        &Instruction::Add1(r0, r1) => {
            let v0 = get_reg(r0, &regs);
            let v1 = get_reg(r1, &regs);
            set_reg(r0, v0 + v1, &mut regs);
        }
        &Instruction::Mul0(r, i) => {
            let v = get_reg(r, &regs);
            set_reg(r, v * i, &mut regs);
        }
        &Instruction::Mul1(r0, r1) => {
            let v0 = get_reg(r0, &regs);
            let v1 = get_reg(r1, &regs);
            set_reg(r0, v0 * v1, &mut regs);
        }
        &Instruction::Mod0(r0, i) => {
            let v0 = get_reg(r0, &regs);
            set_reg(r0, v0 % i, &mut regs);
        }
        &Instruction::Mod1(r0, r1) => {
            let v0 = get_reg(r0, &regs);
            let v1 = get_reg(r1, &regs);
            set_reg(r0, v0 % v1, &mut regs);
        }
        &Instruction::Rcv(r) => {
            if !recv_queue.is_empty() {
                let v = recv_queue.pop_front().unwrap();
                set_reg(r, v, &mut regs);
            } else {
                return false;
            }
        }
        &Instruction::Jgz0(r0, i) => {
            let v0 = get_reg(r0, &regs);
            if v0 > 0 {
                *pc += i;
                return true;
            }
        }
        &Instruction::Jgz1(r0, r1) => {
            let v0 = get_reg(r0, &regs);
            let v1 = get_reg(r1, &regs);
            if v0 > 0 {
                *pc += v1;
                return true;
            }
        }
        &Instruction::Jmp(i) => {
            *pc += i;
            return true;
        }
    }
    *pc += 1;
    return true;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let cmds: Vec<Vec<String>> = input
        .trim()
        .split('\n')
        .map(|v| v.split(' ').map(|i| i.to_string()).collect())
        .collect();
    let instrs = collect_instructions(cmds.clone());

    {
        let mut pc: i64 = 0;
        let mut regs = vec![0 as i64; 25];
        let mut queue_snd = VecDeque::<i64>::new();
        let mut queue_rcv = VecDeque::<i64>::new();
        let mut sndctr = 0;
        let mut last_snd = 0;
        loop {
            let running = do_instruction(
                &mut pc,
                &mut regs,
                &instrs,
                &mut queue_snd,
                &mut queue_rcv,
                &mut sndctr,
            );
            if !queue_snd.is_empty() {
                last_snd = queue_snd.pop_front().unwrap();
            }
            if !running {
                break;
            }
        }
        println!("Part one: {}", last_snd);
    }

    {
        let mut pc0: i64 = 0;
        let mut regs0 = vec![0 as i64; 25];
        let mut queue0 = VecDeque::<i64>::new();
        let mut sndctr0 = 0;

        let mut pc1: i64 = 0;
        let mut regs1 = vec![0 as i64; 25];
        let mut queue1 = VecDeque::<i64>::new();
        let mut sndctr1 = 0;

        set_reg('p', 0, &mut regs0);
        set_reg('p', 1, &mut regs1);

        loop {
            let running0 = do_instruction(
                &mut pc0,
                &mut regs0,
                &instrs,
                &mut queue0,
                &mut queue1,
                &mut sndctr0,
            );
            let running1 = do_instruction(
                &mut pc1,
                &mut regs1,
                &instrs,
                &mut queue1,
                &mut queue0,
                &mut sndctr1,
            );

            if !running0 && !running1 {
                break;
            }
        }

        println!("Part two: {}", sndctr1);
    }
}
