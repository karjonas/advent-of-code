extern crate common;

use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &String) -> HashMap<String, Vec<String>> {
    let mut cavern = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split('-').collect();
        let a = String::from(tokens[0]);
        let b = String::from(tokens[1]);
        let entry_a = cavern.entry(a.clone()).or_insert(Vec::new());
        entry_a.push(b.clone());
        let entry_b = cavern.entry(b).or_insert(Vec::new());
        entry_b.push(a);
    }

    return cavern;
}

fn check_cave_p1(visited_lower: &HashSet<String>, cave: &String) -> bool {
    return is_uppercase(cave) || !visited_lower.contains(cave);
}

fn is_uppercase(input: &String) -> bool {
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            return false;
        }
    }
    return true;
}

fn check_cave_p2(visited_lower: &HashSet<String>, double_visit: bool, cave: &String) -> bool {
    if is_uppercase(cave) {
        return true;
    }
    if !double_visit && visited_lower.contains(cave) && cave != "start" {
        return true;
    }
    return !visited_lower.contains(cave);
}

fn flood(
    pos: &String,
    visited_lower: &mut HashSet<String>,
    mut double_visit: bool,
    cave: &HashMap<String, Vec<String>>,
    part_two: bool,
) -> usize {
    if pos == "end" {
        return 1;
    }

    if (!part_two && !check_cave_p1(visited_lower, pos))
        || (part_two && !check_cave_p2(visited_lower, double_visit, pos))
    {
        return 0;
    }

    if !is_uppercase(pos) {
        if visited_lower.contains(pos) {
            double_visit = true;
        } else {
            visited_lower.insert(pos.clone());
        }
    }
    let mut sum = 0;
    for next in cave.get(pos).unwrap() {
        let mut visited_next = visited_lower.clone();
        sum += flood(next, &mut visited_next, double_visit, cave, part_two);
    }

    return sum;
}

fn solve_internal(input: &String, part_two: bool) -> usize {
    let cave = parse_input(input);
    let result = flood(
        &String::from("start"),
        &mut HashSet::new(),
        false,
        &cave,
        part_two,
    );
    return result;
}

pub fn solve() {
    let input = common::read_file("2021/day12/input");
    println!("Part one: {}", solve_internal(&input, false));
    println!("Part two: {}", solve_internal(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let input2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let input3 = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!(solve_internal(&String::from(input), false), 10);
        assert_eq!(solve_internal(&String::from(input2), false), 19);
        assert_eq!(solve_internal(&String::from(input3), false), 226);

        assert_eq!(solve_internal(&String::from(input), true), 36);
        assert_eq!(solve_internal(&String::from(input2), true), 103);
        assert_eq!(solve_internal(&String::from(input3), true), 3509);
    }
}
