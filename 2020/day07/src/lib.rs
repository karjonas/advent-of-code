extern crate common;

use std::collections::HashMap;

type NodeMap = HashMap<String, Vec<(usize, String)>>;

fn parse_input(input: &String) -> NodeMap {
    let mut nodes = NodeMap::new();

    for line in input.lines() {
        let name = line.split(" ").take(2).collect::<Vec<_>>().join(" ");

        if line.contains("contain no other bags.") {
            nodes.insert(name, Vec::new());
            continue;
        }

        let bags = line
            .replace((name.clone() + " bags contain ").as_str(), "")
            .replace(".", "")
            .split(", ")
            .map(|v| String::from(v))
            .collect::<Vec<_>>();

        let mut children = Vec::new();
        for bag in bags {
            let words = bag.split_whitespace().collect::<Vec<_>>();
            let amount = common::string_to_usize(words[0]);
            let child_name = words[1..3].join(" ");
            children.push((amount, child_name));
        }

        nodes.insert(name, children);
    }

    return nodes;
}

fn find_recursive(nodes: &NodeMap, needle: &String, bags: &Vec<(usize, String)>) -> bool {
    for (_, bag) in bags {
        if bag == needle || find_recursive(nodes, needle, nodes.get(bag).unwrap()) {
            return true;
        };
    }
    return false;
}

fn part_one(nodes: &NodeMap) -> usize {
    return nodes
        .iter()
        .map(|(_name, children)| {
            find_recursive(nodes, &"shiny gold".to_string(), &children) as usize
        })
        .sum::<usize>();
}

fn count_recursive(nodes: &NodeMap, bags: &Vec<(usize, String)>) -> usize {
    return bags
        .iter()
        .map(|(amount, bag)| amount * count_recursive(nodes, &nodes.get(bag).unwrap()))
        .sum::<usize>()
        + 1;
}

fn part_two(nodes: &NodeMap) -> usize {
    return count_recursive(nodes, &nodes.get("shiny gold").unwrap()) - 1;
}

pub fn solve() {
    let input = common::read_file("2020/day07/input");
    let map = parse_input(&input);
    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 4);
    }

    #[test]
    fn test_samples_part_two() {
        let input = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .join("\n");
        assert_eq!(part_two(&parse_input(&input)), 126);
    }
}
