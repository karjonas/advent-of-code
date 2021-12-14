extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

fn parse_input(input: &String) -> (String, HashMap<String, (String, String, char)>) {
    let lines: Vec<&str> = input.lines().collect();
    let start = String::from(lines[0]);
    let mut rules = HashMap::new();
    for line in &lines[2..] {
        let (s, to) = scan_fmt!(line, "{} -> {}", String, char).unwrap();
        let chars: Vec<_> = s.chars().collect();

        assert_eq!(rules.contains_key(&s), false);
        rules.insert(
            s,
            (
                [chars[0], to].iter().collect(),
                [to, chars[1]].iter().collect(),
                to
            ),
        );
    }

    return (start, rules);
}

fn solve_internal(input: &String, repeats: usize) -> usize {
    let (start, rules) = parse_input(input);
    let mut polymers = HashMap::new();
    let mut counters = HashMap::new();

    for i in 0..start.len() - 1 {
        let curr = String::from(&start[i..i + 2]);
        *polymers.entry(curr).or_insert(0) += 1;
    }

    for c in start.chars() {
        *counters.entry(c).or_insert(0) += 1;
    }

    for _i in 0..repeats {
        let mut polymers_next = HashMap::new();

        for (polymer, count) in &polymers {
            let (a, b, mid) = rules.get(polymer).unwrap().clone();
            *counters.entry(mid).or_insert(0) += *count;
            *polymers_next.entry(a).or_insert(0) += *count;
            *polymers_next.entry(b).or_insert(0) += *count;
        }

        polymers = polymers_next.clone();
    }

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();

    return max - min;
}

fn solve_internal_p1(input: &String) -> usize {
    return solve_internal(input, 10);
}

fn solve_internal_p2(input: &String) -> usize {
    return solve_internal(input, 40);
}

pub fn solve() {
    let input = common::read_file("2021/day14/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

        assert_eq!(solve_internal_p1(&String::from(input)), 1588);
        assert_eq!(solve_internal_p2(&String::from(input)), 2188189693529);
    }
}
