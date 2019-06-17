extern crate common;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i64),
    Jie(char, i64),
    Jio(char, i64),
}

fn string_to_instruction(s: &Vec<String>) -> Instruction {
    let name = s[0].clone();
    if name == "hlf" {
        let c = s[1].chars().collect::<Vec<_>>()[0];
        return Instruction::Hlf(c);
    }
    if name == "tpl" {
        let c = s[1].chars().collect::<Vec<_>>()[0];
        return Instruction::Tpl(c);
    }
    if name == "inc" {
        let c = s[1].chars().collect::<Vec<_>>()[0];
        return Instruction::Inc(c);
    }
    if name == "jmp" {
        let n = common::string_to_i64(s[1].as_str());
        return Instruction::Jmp(n);
    }
    if name == "jie" {
        let c = s[1].chars().collect::<Vec<_>>()[0];
        let n = common::string_to_i64(s[2].as_str());
        return Instruction::Jie(c, n);
    }
    if name == "jio" {
        let c = s[1].chars().collect::<Vec<_>>()[0];
        let n = common::string_to_i64(s[2].as_str());
        return Instruction::Jio(c, n);
    }
    assert!(false);
    return Instruction::Hlf('a');
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();

    let mut instructions = Vec::new();
    for i in 0..num_lines {
        let splitted = common::strip_characters(&lines[i], ",")
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        instructions.push(string_to_instruction(&splitted));
    }

    return instructions;
}

fn do_instruction(instruction: Instruction, registers: &mut HashMap<char, usize>, ip: i64) -> i64 {
    match instruction {
        Instruction::Hlf(c) => {
            let v = registers.get(&c).unwrap().clone() / 2;
            registers.insert(c, v);
        }
        Instruction::Tpl(c) => {
            let v = registers.get(&c).unwrap().clone() * 3;
            registers.insert(c, v);
        }
        Instruction::Inc(c) => {
            let v = registers.get(&c).unwrap().clone() + 1;
            registers.insert(c, v);
        }
        Instruction::Jmp(v) => return ip + v,
        Instruction::Jie(c, v) => {
            let even = 0 == registers.get(&c).unwrap().clone() % 2;
            if even {
                return ip + v;
            }
        }
        Instruction::Jio(c, v) => {
            let is_one = 1 == registers.get(&c).unwrap().clone();
            if is_one {
                return ip + v;
            }
        }
    };

    return ip + 1;
}

fn run(input: &str, start_val_a: usize) -> usize {
    let instrs = parse_input(&input);
    let num_instrs = instrs.len() as i64;
    let mut ip = 0 as i64;
    let mut hm = HashMap::new();
    hm.insert('a', start_val_a);
    hm.insert('b', 0);

    while ip < num_instrs {
        ip = do_instruction(instrs[ip as usize].clone(), &mut hm, ip);
    }

    return hm.get(&'b').unwrap().clone();
}

pub fn solve() {
    let input = common::read_file("2015/day23/input");
    println!("Part one: {}", run(input.as_str(), 0));
    println!("Part two: {}", run(input.as_str(), 1));
}
