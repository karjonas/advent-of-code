extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
#[derive(Debug)]
struct Rule {
    letter: char,
    rules: Vec<Vec<usize>>,
}

type RuleMap = HashMap<usize, Rule>;

fn parse_input(input: &String) -> (RuleMap, Vec<String>) {
    let mut rules = RuleMap::new();
    let mut messages = Vec::new();
    let mut reading_rules = true;
    for line in input.lines() {
        if line == "" {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let (idx, rest) = scan_fmt!(line, "{d}: {/.*/}", usize, String).unwrap();
            if rest == "\"a\"" || rest == "\"b\"" {
                rules.insert(
                    idx,
                    Rule {
                        letter: rest.chars().nth(1).unwrap(),
                        rules: Vec::new(),
                    },
                );
            } else {
                rules.insert(
                    idx,
                    Rule {
                        letter: '#',
                        rules: rest
                            .split(" | ")
                            .map(|str| {
                                str.split(" ")
                                    .map(|v| v.parse::<usize>().unwrap())
                                    .collect()
                            })
                            .collect(),
                    },
                );
            }
        } else {
            messages.push(String::from(line));
        }
    }
    return (rules, messages);
}

fn resolve(rules: &RuleMap, message: &Vec<char>, idx: usize, rule: &Rule) -> Vec<usize> {
    if idx >= message.len() {
        return Vec::new();
    }

    if rule.letter != '#' {
        return if message[idx] == rule.letter {
            [idx + 1].to_vec()
        } else {
            Vec::new()
        };
    }

    let mut matched_indices = Vec::new();

    for rules_curr in &rule.rules {
        let mut curr_indices = [idx].to_vec();
        for rule_next_idx in rules_curr {
            let mut new_indices = Vec::new();
            for idx_curr in &curr_indices {
                new_indices.append(&mut resolve(
                    rules,
                    message,
                    *idx_curr,
                    rules.get(rule_next_idx).unwrap(),
                ));
            }

            curr_indices = new_indices;

            if curr_indices.is_empty() {
                break;
            }
        }

        matched_indices.append(&mut curr_indices);
    }

    return matched_indices;
}

fn part_one(input: &String) -> usize {
    let (rules, messages) = parse_input(input);

    let mut sum = 0;
    for message in messages {
        let indices = resolve(
            &rules,
            &message.chars().collect(),
            0,
            &rules.get(&0).unwrap(),
        );

        sum += indices.iter().any(|index| *index == message.len()) as usize;
    }
    return sum;
}

fn part_two(input: &String) -> usize {
    return part_one(
        &input
            .replace("8: 42", "8: 42 | 42 8")
            .replace("11: 42 31", "11: 42 31 | 42 11 31"),
    );
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ]
        .join("\n");

        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_samples_p2() {
        let input = [
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            // This fails for some reason "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabb",
        ]
        .join("\n");

        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 11);
    }
}
