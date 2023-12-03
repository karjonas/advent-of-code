extern crate common;
extern crate regex;

use regex::Regex;

use std::collections::HashMap;

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

const ALL_INSTRUCTIONS: [Instruction; 16] = [
    Instruction::Addr,
    Instruction::Addi,
    Instruction::Mulr,
    Instruction::Muli,
    Instruction::Banr,
    Instruction::Bani,
    Instruction::Borr,
    Instruction::Bori,
    Instruction::Setr,
    Instruction::Seti,
    Instruction::Gtir,
    Instruction::Gtri,
    Instruction::Gtrr,
    Instruction::Eqir,
    Instruction::Eqri,
    Instruction::Eqrr,
];

#[derive(Debug, Clone)]
struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
    possible_instructions: Vec<Instruction>,
}

fn parse_input(input: &String) -> (Vec<Sample>, Vec<[usize; 4]>) {
    let lines = input.lines().collect::<Vec<_>>();

    let re = Regex::new(r".*\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut samples = Vec::new();

    let mut start_idx = 0;
    loop {
        // Before: [0, 3, 0, 3]
        // 9 0 0 1
        // After:  [0, 0, 0, 3]
        //
        if start_idx >= lines.len() || lines[start_idx] == "" {
            break;
        }

        let line_before = lines[start_idx + 0];
        let line_result = lines[start_idx + 1];
        let line_after = lines[start_idx + 2];

        let before: [usize; 4];
        let mut instruction: [usize; 4] = [0; 4];
        let after: [usize; 4];

        {
            let cap = re.captures(line_before).unwrap();
            before = [
                common::string_to_i64(&cap[1]) as usize,
                common::string_to_i64(&cap[2]) as usize,
                common::string_to_i64(&cap[3]) as usize,
                common::string_to_i64(&cap[4]) as usize,
            ];
        }
        {
            let res = line_result.split_whitespace().collect::<Vec<_>>();
            for i in 0..4 {
                instruction[i] = common::string_to_i64(res[i]) as usize;
            }
        }
        {
            let cap = re.captures(line_after).unwrap();
            after = [
                common::string_to_i64(&cap[1]) as usize,
                common::string_to_i64(&cap[2]) as usize,
                common::string_to_i64(&cap[3]) as usize,
                common::string_to_i64(&cap[4]) as usize,
            ];
        }
        samples.push(Sample {
            before: before,
            instruction: instruction,
            after: after,
            possible_instructions: Vec::new(),
        });
        start_idx += 4;
    }

    start_idx += 2;
    let mut program = Vec::new();
    loop {
        if start_idx >= lines.len() {
            break;
        }

        let mut res_v = [0, 0, 0, 0];
        let res = lines[start_idx].split_whitespace().collect::<Vec<_>>();
        for i in 0..4 {
            res_v[i] = common::string_to_i64(res[i]) as usize;
        }
        program.push(res_v);
        start_idx += 1;
    }
    return (samples, program);
}

fn do_instruction(instruction: Instruction, args: &[usize; 4], input: &[usize; 4]) -> [usize; 4] {
    let mut o = input.clone();
    let a = args[1] as usize;
    let b = args[2] as usize;
    let c = args[3] as usize;
    let reg_a = input[a as usize] as usize;
    let reg_b = input[b as usize] as usize;
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
    return o;
}

fn instruction_compatible(
    instruction: Instruction,
    args: &[usize; 4],
    input: &[usize; 4],
    output: &[usize; 4],
) -> bool {
    let input_us = [
        input[0] as usize,
        input[1] as usize,
        input[2] as usize,
        input[3] as usize,
    ];
    let res = do_instruction(instruction, args, &input_us);

    for i in 0..4 {
        if res[i] != output[i] as usize {
            return false;
        }
    }

    return true;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let (mut samples, program) = parse_input(&input);
    for i in 0..samples.len() {
        for instr in ALL_INSTRUCTIONS.iter() {
            if instruction_compatible(
                instr.clone(),
                &samples[i].instruction,
                &samples[i].before,
                &samples[i].after,
            ) {
                samples[i].possible_instructions.push(instr.clone());
            }
        }
    }

    let mut solution: HashMap<usize, Vec<Instruction>> = HashMap::new();

    let mut ctr = 0;
    for sample in samples.iter() {
        let idx = sample.instruction[0];
        let vecs = solution
            .entry(idx)
            .or_insert(sample.possible_instructions.clone())
            .clone();
        let filtered = vecs
            .iter()
            .filter(|v| sample.possible_instructions.contains(v))
            .map(|v| v.clone())
            .collect::<Vec<_>>()
            .clone();
        solution.insert(idx, filtered);

        if sample.possible_instructions.len() >= 3 {
            ctr += 1
        }
    }

    println!("Part one: {}", ctr);
    loop {
        let mut done = true;
        let mut to_remove = Vec::new();
        for (_k, v) in &solution {
            if v.len() == 1 {
                to_remove.push(v[0].clone());
            }
        }

        for (k, v) in solution.clone() {
            if v.len() == 1 {
                continue;
            }

            done = false;

            let v_new = v
                .iter()
                .filter(|v| !to_remove.contains(v))
                .map(|v| v.clone())
                .collect::<Vec<_>>();
            solution.insert(k, v_new);
        }
        if done {
            break;
        }
    }

    let mut instruction_map: HashMap<usize, Instruction> = HashMap::new();
    for (k, v) in &solution {
        instruction_map.insert(*k, v[0].clone());
    }

    let mut state: [usize; 4] = [0, 0, 0, 0];
    for row in program {
        let instruction = instruction_map.get(&row[0]).unwrap().clone();
        state = do_instruction(instruction, &row, &state);
    }
    println!("Part two: {}", state[0]);
}
