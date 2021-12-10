extern crate common;

use std::collections::HashMap;

fn solve_internal_p1(input: &String) -> usize {
    let openers = ['(', '[', '{', '<'];
    let closers = [')', ']', '}', '>'];
    let penalty = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let matching = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut sum = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            if openers.contains(&c) {
                stack.push(c);
            } else if closers.contains(&c) {
                let expected = *matching.get(stack.last().unwrap()).unwrap();
                if expected != c {
                    sum += penalty.get(&c).unwrap();
                    break;
                }
                stack.pop();
            }
        }
    }
    return sum;
}

fn solve_internal_p2(input: &String) -> usize {
    let openers = ['(', '[', '{', '<'];
    let closers = [')', ']', '}', '>'];
    let costs = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let matching = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut to_match: Vec<String> = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        let mut failed = false;
        for c in line.chars() {
            if openers.contains(&c) {
                stack.push(c);
            } else if closers.contains(&c) {
                let expected = *matching.get(stack.last().unwrap()).unwrap();
                if expected != c {
                    failed = true;
                    break;
                }
                stack.pop();
            }
        }

        if !failed {
            to_match.push(stack.into_iter().rev().collect());
        }
    }

    let mut sums = Vec::new();
    for line in to_match {
        let mut sum = 0;
        for c in line.chars() {
            sum = 5 * sum + costs.get(&c).unwrap();
        }
        sums.push(sum);
    }
    sums.sort();

    return sums[sums.len() / 2];
}

pub fn solve() {
    let input = common::read_file("2021/day10/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(solve_internal_p1(&String::from(input)), 26397);
        assert_eq!(solve_internal_p2(&String::from(input)), 288957);
    }
}
