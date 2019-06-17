use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day08/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<Vec<_>> = contents
        .split('\n')
        .map(|line| line.split(' ').collect())
        .collect();

    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max_intermediate = std::i32::MIN;

    for line in lines {
        let reg = line[0].to_string();
        let instr = line[1];
        let amount = line[2].parse::<i32>().unwrap();

        let cond_reg = line[4].to_string();
        let cond_type = line[5];
        let cond_val = line[6].parse::<i32>().unwrap();

        let reg_cond_val = registers.entry(cond_reg).or_insert(0).clone();
        let reg_ref = registers.entry(reg).or_insert(0);

        let mut valid = false;
        if cond_type == ">" {
            valid = reg_cond_val > cond_val;
        } else if cond_type == "<" {
            valid = reg_cond_val < cond_val;
        } else if cond_type == ">=" {
            valid = reg_cond_val >= cond_val;
        } else if cond_type == "<=" {
            valid = reg_cond_val <= cond_val;
        } else if cond_type == "==" {
            valid = reg_cond_val == cond_val;
        } else if cond_type == "!=" {
            valid = reg_cond_val != cond_val;
        } else {
            assert!(false);
        }

        if valid {
            if instr == "inc" {
                *reg_ref += amount;
            } else if instr == "dec" {
                *reg_ref -= amount;
            } else {
                assert!(false);
            }
        }

        max_intermediate = std::cmp::max(max_intermediate, *reg_ref);
    }

    let mut max_end = std::i32::MIN;
    for (_, v) in registers {
        max_end = std::cmp::max(v, max_end);
    }

    println!("Part one: {}", max_end);
    println!("Part two: {}", max_intermediate);
}
