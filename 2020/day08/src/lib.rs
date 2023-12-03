extern crate common;

type Instruction = (String, i64);
type Program = Vec<Instruction>;

fn parse_input(input: &String) -> Program {
    let mut program = Program::new();
    for line in input.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let instruction = String::from(words[0]);
        let value = common::string_to_i64(words[1]);
        program.push((instruction, value));
    }

    return program;
}

fn run_program(program: &Program) -> (usize, bool) {
    let mut acc: i64 = 0;
    let mut ptr: i64 = 0;
    let mut visited = common::filled_vector(program.len(), false);

    while !visited[ptr as usize] {
        visited[ptr as usize] = true;
        let (instruction, value) = &program[ptr as usize];
        match instruction.as_str() {
            "acc" => {
                acc += value;
                ptr += 1
            }
            "jmp" => ptr += value,
            "nop" => ptr += 1,
            _ => panic!("Invalid instruction"),
        }
        if ptr as usize >= program.len() {
            return (acc as usize, true);
        }
    }

    return (acc as usize, false);
}

fn part_one(program: &Program) -> usize {
    return run_program(program).0;
}

fn part_two(program: &Program) -> usize {
    for i in 0..program.len() {
        let instruction = program[i].0.as_str();

        if instruction == "nop" || instruction == "jmp" {
            let mut program_mod = program.clone();

            match instruction {
                "jmp" => program_mod[i].0 = String::from("nop"),
                "nop" => program_mod[i].0 = String::from("jmp"),
                _ => panic!("Invalid instruction"),
            }

            let (result, success) = run_program(&program_mod);
            if success {
                return result;
            }
        }
    }

    panic!("No solution found");
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let program = parse_input(&input);
    println!("Part one: {}", part_one(&program));
    println!("Part two: {}", part_two(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 5);
        assert_eq!(part_two(&parse_input(&input)), 8);
    }
}
