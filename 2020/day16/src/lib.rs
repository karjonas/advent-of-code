extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Input {
    rules: HashMap<String, ((usize, usize), (usize, usize))>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
    used_number: Vec<bool>,
}

fn parse_input(input: &String) -> Input {
    let mut rules = HashMap::new();
    let mut ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    let mut used_number = Vec::new();

    let mut lines: VecDeque<_> = input.lines().collect();
    // Rules
    while lines.len() > 0 {
        let line = lines.pop_front().unwrap();
        if line.is_empty() {
            break;
        }

        let sides: Vec<_> = line.split(": ").collect();

        let name = sides[0];
        let (a0, a1, b0, b1) =
            scan_fmt!(sides[1], "{d}-{d} or {d}-{d}", usize, usize, usize, usize).unwrap();
        rules.insert(String::from(name), ((a0, a1), (b0, b1)));
        common::grow(&mut used_number, std::cmp::max(a1 + 1, b1 + 1), false);
        for i in a0..a1 + 1 {
            used_number[i] = true;
        }
        for i in b0..b1 + 1 {
            used_number[i] = true;
        }
    }

    // Ticket
    while lines.len() > 0 {
        let line = lines.pop_front().unwrap();

        if line.is_empty() {
            break;
        } else if line == "your ticket:" {
            continue;
        }

        ticket = line
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
    }

    // Nearby Tickets
    while lines.len() > 0 {
        let line = lines.pop_front().unwrap();

        if line.is_empty() {
            break;
        } else if line == "nearby tickets:" {
            continue;
        }

        nearby_tickets.push(
            line.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect(),
        );
    }

    return Input {
        rules: rules,
        ticket: ticket,
        nearby_tickets: nearby_tickets,
        used_number: used_number,
    };
}

fn part_one(input: &Input) -> usize {
    return input
        .nearby_tickets
        .iter()
        .map(|nearby_ticket| {
            nearby_ticket
                .iter()
                .filter(|nearby| {
                    **nearby >= input.used_number.len() || !input.used_number[**nearby]
                })
                .sum::<usize>()
        })
        .sum();
}

fn part_two(input: &Input) -> usize {
    let mut nearby_ok = input.nearby_tickets.clone();
    nearby_ok.retain(|nearby_ticket| {
        for nearby in nearby_ticket {
            if *nearby >= input.used_number.len() || !input.used_number[*nearby] {
                return false;
            }
        }
        return true;
    });

    let mut rule_matches = HashMap::new();
    let num_rules = input.rules.len();
    for (rule, ((a0, a1), (b0, b1))) in &input.rules {
        for i in 0..num_rules {
            let mut success = true;
            for nearby in &nearby_ok {
                if (nearby[i] < *a0 || nearby[i] > *a1) && (nearby[i] < *b0 || nearby[i] > *b1) {
                    success = false;
                    break;
                }
            }
            if success {
                rule_matches.entry(rule).or_insert_with(Vec::new).push(i);
            }
        }
    }

    let mut rule_decided = HashMap::new();

    while !rule_matches.is_empty() {
        let mut matched_idx = 0;
        let mut matched_rule = String::new();
        for (rule, numbers) in &rule_matches {
            if numbers.len() == 1 {
                matched_idx = numbers[0];
                matched_rule = (*rule).clone();
                break;
            }
        }

        rule_matches.remove(&matched_rule);
        rule_decided.insert(matched_rule, matched_idx);

        for (_rule, numbers) in &mut rule_matches {
            numbers.retain(|v| *v != matched_idx);
        }
    }

    return [
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
    ]
    .iter()
    .fold(1, |acc, rule_name| {
        acc * input.ticket[*rule_decided.get(&String::from(*rule_name)).unwrap()]
    });
}

pub fn solve(filepath: &str) {
    let input = parse_input(
        &std::fs::read_to_string(filepath)
            .unwrap()
            .trim()
            .to_string(),
    );

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .join("\n");

        assert_eq!(part_one(&parse_input(&input)), 71);
    }
}
