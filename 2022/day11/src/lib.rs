extern crate common;
extern crate scan_fmt;

use scan_fmt::scan_fmt_some;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<usize>,
    operator: String,
    value: String,
    test_div: usize,
    monkey_true: usize,
    monkey_false: usize,
}

fn parse_input(input: &String) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let lines: Vec<_> = input.lines().collect();

    for m_i in 0..(lines.len() + 1) / 7 {
        let i = m_i * 7;
        let items: VecDeque<usize> = lines[i + 1]
            .replace(",", "")
            .split_ascii_whitespace()
            .skip(2)
            .map(|v| v.parse().unwrap())
            .collect();

        let (operator, value) =
            scan_fmt_some!(lines[i + 2], "  Operation: new = old {} {}", String, String);
        let test_div = scan_fmt_some!(lines[i + 3], "  Test: divisible by {}", usize).unwrap();
        let monkey_true =
            scan_fmt_some!(lines[i + 4], "    If true: throw to monkey {}", usize).unwrap();
        let monkey_false =
            scan_fmt_some!(lines[i + 5], "    If false: throw to monkey {}", usize).unwrap();

        let monkey = Monkey {
            items: items,
            operator: operator.unwrap(),
            value: value.unwrap(),
            test_div: test_div,
            monkey_true: monkey_true,
            monkey_false: monkey_false,
        };
        monkeys.push(monkey);
    }

    return monkeys;
}

fn calc_worry(worry: usize, monkey: &Monkey) -> usize {
    let value = if monkey.value == "old" {
        worry
    } else {
        monkey.value.parse().unwrap()
    };

    if monkey.operator == "*" {
        return worry * value;
    } else if monkey.operator == "+" {
        return worry + value;
    }

    assert!(false);
    return 0;
}

fn part_both(input: &String, part_one: bool) -> usize {
    let mut monkeys = parse_input(input);
    let num_monkeys = monkeys.len();
    let mut inspected = vec![0; num_monkeys];
    let num_iters = if part_one { 20 } else { 10000 };
    let worry_div = if part_one { 3 } else { 1 };
    let yuge = monkeys.iter().fold(1, |prod, v| prod * v.test_div);

    for _ in 0..num_iters {
        for m_i in 0..num_monkeys {
            inspected[m_i] += monkeys[m_i].items.len();
            while !monkeys[m_i].items.is_empty() {
                let worry = monkeys[m_i].items.pop_front().unwrap();
                let worry_new = (calc_worry(worry, &monkeys[m_i]) / worry_div) % yuge;
                if worry_new % monkeys[m_i].test_div == 0 {
                    let to = monkeys[m_i].monkey_true;
                    monkeys[to].items.push_back(worry_new);
                } else {
                    let to = monkeys[m_i].monkey_false;
                    monkeys[to].items.push_back(worry_new);
                }
            }
        }
    }
    inspected.sort();

    return inspected[num_monkeys - 1] * inspected[num_monkeys - 2];
}

fn part_one(input: &String) -> usize {
    part_both(input, true)
}

fn part_two(input: &String) -> usize {
    part_both(input, false)
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
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            .to_string();
        assert_eq!(part_one(&input), 10605);
        assert_eq!(part_two(&input), 2713310158);
    }
}
