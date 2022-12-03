extern crate common;

fn char_to_value(c_char: char) -> usize {
    let c = c_char as usize;
    const A_LOW: usize = 'a' as usize;
    const A_UP: usize = 'A' as usize;
    if c > A_LOW {
        return 1 + c - A_LOW;
    } else {
        return 27 + c - A_UP;
    }
}

fn solve_part_one(input: &String) -> usize {
    let mut accum = 0;
    for line_str in input.lines() {
        let line: Vec<char> = line_str.chars().collect();
        let n = line.len();
        let v0: Vec<char> = line[0..n / 2].iter().cloned().collect();
        let v1: Vec<char> = line[n / 2..n].iter().cloned().collect();
        for c in v0 {
            if v1.contains(&c) {
                accum += char_to_value(c);
                break;
            }
        }
    }
    return accum;
}

fn solve_part_two(input: &String) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut accum = 0;
    while i < lines.len() {
        for c in lines[i].chars() {
            if lines[i + 1].contains(c) && lines[i + 2].contains(c) {
                accum += char_to_value(c);
                break;
            }
        }
        i += 3;
    }
    return accum;
}

pub fn solve() {
    let input = common::read_file("2022/day03/input");
    println!("Part one: {}", solve_part_one(&input));
    println!("Part two: {}", solve_part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input_simple = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        assert_eq!(solve_part_one(&input_simple), 157);
        assert_eq!(solve_part_two(&input_simple), 70);
    }
}
