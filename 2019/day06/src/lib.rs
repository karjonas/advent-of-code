extern crate common;

use std::collections::HashMap;

fn count_children_recursive(
    parent: &String,
    children: &HashMap<String, Vec<String>>,
    depth: usize,
) -> usize {
    if !children.contains_key(parent) {
        return depth;
    }

    let mut count = depth;
    for child in children.get(parent).unwrap() {
        count += count_children_recursive(child, children, depth + 1);
    }
    return count;
}

fn parse_input(input_str: &str) -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let mut parents = HashMap::new();
    let mut children = HashMap::new();

    let lines: Vec<String> = input_str
        .split('\n')
        .map(|v| String::from(v))
        .collect::<Vec<_>>();

    for line in lines {
        let split: Vec<_> = line.split(')').map(|v| String::from(v)).collect();
        assert_eq!(split.len(), 2);
        let parent = split[0].clone();
        let child = split[1].clone();
        parents.insert(child.clone(), parent.clone());
        children.entry(parent).or_insert(Vec::new()).push(child);
    }
    return (parents, children);
}

fn calc_indirect(input_str: &str) -> usize {
    let (_parents, children) = parse_input(input_str);
    return count_children_recursive(&String::from("COM"), &children, 0);
}

fn calc_dist(input_str: &str) -> usize {
    let (parents, _children) = parse_input(input_str);
    let mut curr_parent = parents.get("YOU").unwrap().clone();
    let mut steps_you = 0;
    while curr_parent != "" {
        let mut san_parent = parents.get("SAN").unwrap().clone();
        let mut steps_san = 0;
        while san_parent != "" {
            if san_parent == curr_parent {
                return steps_you + steps_san;
            }
            san_parent = parents.get(&san_parent).unwrap_or(&String::new()).clone();
            steps_san += 1;
        }
        steps_you += 1;
        curr_parent = parents.get(&curr_parent).unwrap_or(&String::new()).clone();
    }

    return 0;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", calc_indirect(input.as_str()));
    println!("Part two: {}", calc_dist(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let map = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(calc_indirect(map), 42);
    }

    #[test]
    fn test_samples_part_two() {
        let map = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(calc_dist(map), 4);
    }
}
