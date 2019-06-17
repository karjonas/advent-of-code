extern crate common;

use std::collections::HashMap;

const INPUT: &str = "2015/day16/input";

fn parse_input() -> Vec<HashMap<String, usize>> {
    let input = common::read_file(INPUT);
    let mut sues = Vec::new();
    for line in input.lines() {
        let mut props = HashMap::new();
        // Sue 2: akitas: 10, perfumes: 10, children: 5
        let words = common::strip_characters(line, ",:")
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        props.insert(
            words[2].clone(),
            common::string_to_i64(words[3].as_str()) as usize,
        );
        props.insert(
            words[4].clone(),
            common::string_to_i64(words[5].as_str()) as usize,
        );
        props.insert(
            words[6].clone(),
            common::string_to_i64(words[7].as_str()) as usize,
        );

        sues.push(props.clone());
    }
    return sues;
}

fn props_exact() -> HashMap<String, usize> {
    let mut props = HashMap::new();
    props.insert("children".to_string(), 3);
    props.insert("samoyeds".to_string(), 2);
    props.insert("akitas".to_string(), 0);
    props.insert("vizslas".to_string(), 0);
    props.insert("cars".to_string(), 2);
    props.insert("perfumes".to_string(), 1);
    return props;
}

fn props_less_than() -> HashMap<String, usize> {
    let mut props = HashMap::new();
    props.insert("pomeranians".to_string(), 3);
    props.insert("goldfish".to_string(), 5);
    return props;
}

fn props_greater_than() -> HashMap<String, usize> {
    let mut props = HashMap::new();
    props.insert("cats".to_string(), 7);
    props.insert("trees".to_string(), 3);
    return props;
}

fn part_one() -> usize {
    let sues = parse_input();
    let mut goal_props = props_exact();
    for (prop, number) in props_greater_than() {
        goal_props.insert(prop, number);
    }
    for (prop, number) in props_less_than() {
        goal_props.insert(prop, number);
    }

    let mut ctr = 1;
    for sue in sues {
        let mut matched = true;
        for (prop, number) in &sue {
            if goal_props.get(&prop.clone()).unwrap() != number {
                matched = false;
                break;
            }
        }

        if matched {
            break;
        }

        ctr += 1;
    }

    return ctr;
}

fn part_two() -> usize {
    let sues = parse_input();
    let goal_exact = props_exact();
    let goal_less = props_less_than();
    let goal_greater = props_greater_than();

    let mut ctr = 1;
    for sue in sues {
        let mut matched = true;
        for (prop, number) in &sue {
            let mut matched_curr = false;
            match goal_exact.get(&prop.clone()) {
                Some(value) => {
                    matched_curr = matched_curr || value == number;
                }
                None => (),
            }
            match goal_less.get(&prop.clone()) {
                Some(value) => {
                    matched_curr = matched_curr || number < value;
                }
                None => (),
            }
            match goal_greater.get(&prop.clone()) {
                Some(value) => {
                    matched_curr = matched_curr || number > value;
                }
                None => (),
            }

            if !matched_curr {
                matched = false;
                break;
            }
        }

        if matched {
            break;
        }

        ctr += 1;
    }

    return ctr;
}

pub fn solve() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
