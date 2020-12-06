extern crate common;

use std::collections::HashSet;

pub fn solve() {
    let input = common::read_file("2020/day06/input");

    let mut curr_group_intersect = HashSet::new();
    let mut curr_group: HashSet<char> = HashSet::new();

    let mut groups_intersect = Vec::new();
    let mut groups = Vec::new();

    let mut first = true;
    for line in input.lines() {
        if line.is_empty() {
            groups_intersect.push(curr_group_intersect.clone());
            groups.push(curr_group.clone());
            first = true;
        } else {
            let line_answer = line.chars().collect::<HashSet<char>>();
            if first {
                curr_group_intersect = line_answer.clone();
                curr_group = line_answer;
                first = false;
            } else {
                curr_group_intersect = curr_group_intersect
                    .intersection(&line_answer)
                    .map(|v| *v)
                    .collect();
                curr_group.extend(line_answer.iter());
            }
        }
    }
    groups_intersect.push(curr_group_intersect);
    groups.push(curr_group);

    let part_one: usize = groups.iter().map(|v| v.len()).sum();
    let part_two: usize = groups_intersect.iter().map(|v| v.len()).sum();

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
