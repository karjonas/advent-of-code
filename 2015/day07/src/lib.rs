extern crate common;

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Gate {
    DirectValue(u16, String),
    DirectSource(String, String),
    And(String, String, String),
    AndValue(u16, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

fn parse_circuit(input: &str) -> Vec<Gate> {
    let mut result = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();

        if words[0] == "NOT" {
            result.push(Gate::Not(words[1].to_string(), words[3].to_string()));
        } else if words[1] == "LSHIFT" {
            result.push(Gate::LShift(
                words[0].to_string(),
                common::string_to_i64(words[2]) as u16,
                words[4].to_string(),
            ));
        } else if words[1] == "RSHIFT" {
            result.push(Gate::RShift(
                words[0].to_string(),
                common::string_to_i64(words[2]) as u16,
                words[4].to_string(),
            ));
        } else if common::is_number(words[0]) && words[1] == "AND" {
            result.push(Gate::AndValue(
                common::string_to_i64(words[0]) as u16,
                words[2].to_string(),
                words[4].to_string(),
            ));
        } else if words[1] == "AND" {
            result.push(Gate::And(
                words[0].to_string(),
                words[2].to_string(),
                words[4].to_string(),
            ));
        } else if words[1] == "OR" {
            result.push(Gate::Or(
                words[0].to_string(),
                words[2].to_string(),
                words[4].to_string(),
            ));
        } else if common::is_number(words[0]) && words[1] == "->" {
            result.push(Gate::DirectValue(
                common::string_to_i64(words[0]) as u16,
                words[2].to_string(),
            ));
        } else if words[1] == "->" {
            result.push(Gate::DirectSource(
                words[0].to_string(),
                words[2].to_string(),
            ));
        } else {
            assert!(false);
        }
    }
    return result;
}

fn solve_circuit(gates: &Vec<Gate>, values: &mut HashMap<String, u16>) -> HashMap<String, u16> {
    let num_gates = gates.len();
    let mut any_change = true;
    let constants = values.clone();

    while any_change {
        let values_prev = values.clone();

        for i in 0..num_gates {
            for (k, v) in &constants {
                values.insert(k.clone(), *v);
            }
            match gates[i] {
                Gate::DirectValue(ref num, ref dst) => {
                    values.insert(dst.to_string(), *num);
                }
                Gate::DirectSource(ref src0, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0);
                    }
                }
                Gate::And(ref src0, ref src1, ref dst) => {
                    if values.contains_key(src0) && values.contains_key(src1) {
                        let v0 = *values.get(src0).unwrap();
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), v0 & v1);
                    }
                }
                Gate::AndValue(ref v0, ref src1, ref dst) => {
                    if values.contains_key(src1) {
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), *v0 & v1);
                    }
                }
                Gate::Or(ref src0, ref src1, ref dst) => {
                    if values.contains_key(src0) && values.contains_key(src1) {
                        let v0 = *values.get(src0).unwrap();
                        let v1 = *values.get(src1).unwrap();
                        values.insert(dst.to_string(), v0 | v1);
                    }
                }
                Gate::LShift(ref src0, ref num, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0 << *num);
                    }
                }
                Gate::RShift(ref src0, ref num, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), v0 >> *num);
                    }
                }
                Gate::Not(ref src0, ref dst) => {
                    if values.contains_key(src0) {
                        let v0 = *values.get(src0).unwrap();
                        values.insert(dst.to_string(), !v0);
                    }
                }
            }
        }
        any_change = values_prev != *values;
    }

    return values.clone();
}

fn part_one(input: &str) -> usize {
    let gates = parse_circuit(input);
    let hm = solve_circuit(&gates, &mut HashMap::new());
    return *hm.get("a").unwrap() as usize;
}

fn part_two(input: &str) -> usize {
    let gates = parse_circuit(input);
    let value_a = *solve_circuit(&gates, &mut HashMap::new()).get("a").unwrap();
    let mut hm = HashMap::new();
    hm.insert("b".to_string(), value_a);
    let hm = solve_circuit(&gates, &mut hm);
    return *hm.get("a").unwrap() as usize;
}

pub fn solve(filepath: &str) {
    println!(
        "Part one: {}",
        part_one(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string()
                .as_str()
        )
    );
    println!(
        "Part two: {}",
        part_two(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string()
                .as_str()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";
        let hm = solve_circuit(&parse_circuit(input), &mut HashMap::new());

        assert_eq!(*hm.get("d").unwrap(), 72 as u16);
        assert_eq!(*hm.get("e").unwrap(), 507 as u16);
        assert_eq!(*hm.get("f").unwrap(), 492 as u16);
        assert_eq!(*hm.get("g").unwrap(), 114 as u16);
        assert_eq!(*hm.get("h").unwrap(), 65412 as u16);
        assert_eq!(*hm.get("i").unwrap(), 65079 as u16);
        assert_eq!(*hm.get("x").unwrap(), 123 as u16);
        assert_eq!(*hm.get("y").unwrap(), 456 as u16);
    }

    #[test]
    fn test_samples_part_two() {}
}
