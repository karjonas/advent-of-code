extern crate common;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
struct FileNode {
    size: usize,
    parent: String,
    children: HashSet<String>,
}

impl FileNode {
    fn new() -> Self {
        return Self {
            size: 0,
            parent: String::new(),
            children: HashSet::new(),
        };
    }
}

fn calc_size(path: &String, nodes: &mut HashMap<String, FileNode>) {
    if nodes[path].size != 0 {
        return;
    }
    let mut sum = 0;
    for child in nodes[path].children.clone() {
        calc_size(&child, nodes);
        sum += nodes[&child].size;
    }
    nodes.get_mut(path).unwrap().size = sum;
}

fn parse_directories(input: &String) -> HashMap<String, FileNode> {
    let mut nodes: HashMap<String, FileNode> = HashMap::new();
    let mut curr_dir = String::new();

    for line in input.lines() {
        // Add node
        nodes.entry(curr_dir.clone()).or_insert(FileNode::new());

        if line == "$ ls" {
            continue;
        }
        if line == "$ cd /" {
            curr_dir = "/".to_string();
            continue;
        }
        if line == "$ cd .." {
            curr_dir = nodes[&curr_dir].parent.clone();
            continue;
        }

        let words: Vec<_> = line.split_ascii_whitespace().collect();
        if words[0] == "$" && words[1] == "cd" {
            curr_dir = curr_dir + words[2] + "/";
            continue;
        }

        // collect dirs/files
        if words[0] == "dir" {
            let dir = curr_dir.clone() + words[1] + "/";
            let dirnode = nodes.entry(dir.clone()).or_insert(FileNode::new());
            dirnode.parent = curr_dir.clone();
            nodes.get_mut(&curr_dir).unwrap().children.insert(dir);
        } else {
            let size = words[0].parse::<usize>().unwrap();
            let file = curr_dir.clone() + words[1];
            nodes
                .get_mut(&curr_dir)
                .unwrap()
                .children
                .insert(file.clone());
            nodes.entry(file.clone()).or_insert(FileNode::new());
            nodes.get_mut(&file).unwrap().size = size;
            nodes.get_mut(&file).unwrap().parent = curr_dir.clone();
        }
    }

    // Go through nodes and calculate their sizes
    calc_size(&"/".to_string(), &mut nodes);

    return nodes;
}

fn part_one(nodes: &HashMap<String, FileNode>) -> usize {
    let mut sum_sub100k = 0;
    for node in nodes {
        if node.0.ends_with("/") && node.1.size <= 100000 {
            sum_sub100k += node.1.size;
        }
    }
    return sum_sub100k;
}

fn part_two(nodes: &HashMap<String, FileNode>) -> usize {
    const CEIL: usize = 40000000;
    let total = nodes["/"].size;
    let mut best_fit = std::usize::MAX;
    for node in nodes {
        if !node.0.ends_with("/") {
            continue;
        }
        let free = total - node.1.size;
        if free <= CEIL && node.1.size < best_fit {
            best_fit = node.1.size;
        }
    }
    return best_fit;
}

pub fn solve() {
    let input = common::read_file("2022/day07/input");
    let nodes = parse_directories(&input);
    println!("Part one: {}", part_one(&nodes));
    println!("Part two: {}", part_two(&nodes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .to_string();
        let nodes = parse_directories(&input);
        assert_eq!(part_one(&nodes), 95437);
        assert_eq!(part_two(&nodes), 24933642);
    }
}
