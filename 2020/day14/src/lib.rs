extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

fn parse_input(input: &String) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let words = line.split(" = ").collect::<Vec<_>>();
        if words[0] == "mask" {
            instructions.push(Instruction::Mask(String::from(words[1])));
        } else {
            let address = scan_fmt!(line, "mem[{d}]", usize).unwrap();
            let value = words[1].parse::<usize>().unwrap();
            instructions.push(Instruction::Mem(address, value));
        }
    }
    return instructions;
}

fn enumerate_addresses(input: &String, output: &mut Vec<usize>) {
    if !input.contains("X") {
        let value_bits = usize::from_str_radix(input.as_str(), 2).unwrap();
        output.push(value_bits);
        return;
    }

    enumerate_addresses(&input.replacen("X", "1", 1), output);
    enumerate_addresses(&input.replacen("X", "0", 1), output);
}

fn part_one(input: &Vec<Instruction>) -> usize {
    let mut memory = HashMap::new();
    let mut mask_active_bits = 0;
    let mut mask_value_bits = 0;
    for instruction in input {
        match instruction {
            Instruction::Mask(text) => {
                mask_active_bits = usize::from_str_radix(
                    String::from(text)
                        .replace("0", "1")
                        .replace("X", "0")
                        .as_str(),
                    2,
                )
                .unwrap();

                mask_value_bits =
                    usize::from_str_radix(text.replace("X", "0").as_str(), 2).unwrap();
            }
            Instruction::Mem(address, value) => {
                let value_new = (value & !mask_active_bits) | mask_value_bits;
                memory.insert(address, value_new);
            }
        };
    }

    return memory.iter().map(|(_, v)| v).sum();
}

fn part_two(input: &Vec<Instruction>) -> usize {
    let mut memory = HashMap::new();
    let mut mask_active_bits = 0;
    let mut masks = Vec::new();
    for instruction in input {
        match instruction {
            Instruction::Mask(text) => {
                masks.clear();
                enumerate_addresses(text, &mut masks);
                mask_active_bits =
                    usize::from_str_radix(String::from(text).replace("X", "1").as_str(), 2)
                        .unwrap();
            }
            Instruction::Mem(address, value) => {
                for mask_value_bits in &masks {
                    let address_new = (address & !mask_active_bits) | mask_value_bits;
                    memory.insert(address_new, value);
                }
            }
        };
    }

    return memory.iter().map(|(_, v)| *v).sum();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let times = parse_input(&input);

    println!("Part one: {}", part_one(&times));
    println!("Part two: {}", part_two(&times));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .join("\n");

        assert_eq!(part_one(&parse_input(&input)), 165);
    }

    #[test]
    fn test_samples_p2() {
        let input = [
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .join("\n");

        assert_eq!(part_two(&parse_input(&input)), 208);
    }
}
