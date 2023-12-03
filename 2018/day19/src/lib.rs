extern crate common;

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, Clone)]
struct InstructionExecutor {
    instr: Instruction,
    args: [usize; 3],
}

fn string_to_instruction(name: &str) -> Instruction {
    if name == "addr" {
        return Instruction::Addr;
    }
    if name == "addi" {
        return Instruction::Addi;
    }
    if name == "mulr" {
        return Instruction::Mulr;
    }
    if name == "muli" {
        return Instruction::Muli;
    }
    if name == "banr" {
        return Instruction::Banr;
    }
    if name == "bani" {
        return Instruction::Bani;
    }
    if name == "borr" {
        return Instruction::Borr;
    }
    if name == "bori" {
        return Instruction::Bori;
    }
    if name == "setr" {
        return Instruction::Setr;
    }
    if name == "seti" {
        return Instruction::Seti;
    }
    if name == "gtir" {
        return Instruction::Gtir;
    }
    if name == "gtri" {
        return Instruction::Gtri;
    }
    if name == "gtrr" {
        return Instruction::Gtrr;
    }
    if name == "eqir" {
        return Instruction::Eqir;
    }
    if name == "eqri" {
        return Instruction::Eqri;
    }
    if name == "eqrr" {
        return Instruction::Eqrr;
    }
    assert!(false);
    return Instruction::Addr;
}

fn parse_input(input: &String) -> (Vec<InstructionExecutor>, usize) {
    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();

    let ip = common::string_to_i64(lines[0].split_whitespace().collect::<Vec<_>>()[1]) as usize;

    let mut instructions = Vec::new();
    for i in 1..num_lines {
        let splitted = lines[i].split_whitespace().collect::<Vec<_>>();
        let istr = splitted[0];
        let args = [
            common::string_to_i64(splitted[1]) as usize,
            common::string_to_i64(splitted[2]) as usize,
            common::string_to_i64(splitted[3]) as usize,
        ];

        let instr = string_to_instruction(istr);

        instructions.push(InstructionExecutor {
            instr: instr,
            args: args,
        });
    }

    return (instructions, ip);
}

fn do_instruction(
    instruction: Instruction,
    args: &[usize; 3],
    input: &[usize; 6],
    ip: usize,
    ip_register: usize,
) -> ([usize; 6], usize) {
    let mut o = input.clone();
    o[ip_register] = ip;
    let a = args[0];
    let b = args[1];
    let c = args[2];
    let reg_a = if a < input.len() { o[a] } else { 0 };
    let reg_b = if b < input.len() { o[b] } else { 0 };
    match instruction {
        Instruction::Addr => o[c] = reg_a + reg_b,
        Instruction::Addi => o[c] = reg_a + b,
        Instruction::Mulr => o[c] = reg_a * reg_b,
        Instruction::Muli => o[c] = reg_a * b,
        Instruction::Banr => o[c] = reg_a & reg_b,
        Instruction::Bani => o[c] = reg_a & b,
        Instruction::Borr => o[c] = reg_a | reg_b,
        Instruction::Bori => o[c] = reg_a | b,
        Instruction::Setr => o[c] = reg_a,
        Instruction::Seti => o[c] = a,
        Instruction::Gtir => o[c] = if a > reg_b { 1 } else { 0 },
        Instruction::Gtri => o[c] = if reg_a > b { 1 } else { 0 },
        Instruction::Gtrr => o[c] = if reg_a > reg_b { 1 } else { 0 },
        Instruction::Eqir => o[c] = if a == reg_b { 1 } else { 0 },
        Instruction::Eqri => o[c] = if reg_a == b { 1 } else { 0 },
        Instruction::Eqrr => o[c] = if reg_a == reg_b { 1 } else { 0 },
    }
    let ip_next = o[ip_register] + 1;
    return (o, ip_next);
}

fn part_two() {
    let magic = 10551361 + 1;
    let mut result = 0;
    for i in 1..magic {
        for j in 1..magic {
            let p = i * j;
            if p > 10551361 {
                break;
            }
            if p == 10551361 {
                result += i;
            }
        }
    }
    println!("Part two: {}", result);
}

fn part_one(input: &String) {
    let (instrs, ipr) = parse_input(input);

    let num_instructions = instrs.len();
    let mut cip = 0;
    let mut state = [0, 0, 0, 0, 0, 0];
    loop {
        if cip >= num_instructions {
            break;
        }
        let executor = &instrs[cip];
        let (state_new, cip_new) =
            do_instruction(executor.instr.clone(), &executor.args, &state, cip, ipr);

        state = state_new;
        cip = cip_new;
    }

    println!("Part one: {:?}", state[0]);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    part_one(&input);
    part_two();
}
