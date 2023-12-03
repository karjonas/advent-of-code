use std::cmp::Ordering;

extern crate common;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    value: i64,
    children: Vec<Node>,
}

fn is_number(c: char) -> bool {
    c as u8 >= '0' as u8 && c as u8 <= '9' as u8
}

fn parse_node(input: &Vec<char>, mut idx: usize) -> (Vec<Node>, usize) {
    let mut children = Vec::new();

    while idx < input.len() {
        if input[idx] == ']' {
            idx += 1;
            break;
        }
        if input[idx] == '[' {
            let (children_sub, idx_new) = parse_node(input, idx + 1);
            let node = Node {
                value: -1,
                children: children_sub,
            };
            children.push(node);
            idx = idx_new;
            continue;
        }
        if is_number(input[idx]) {
            let mut value = 0;
            while is_number(input[idx]) {
                let number = (input[idx] as u8 - '0' as u8) as i64;
                value = number + value * 10;
                idx += 1;
            }
            let node = Node {
                value: value,
                children: Vec::new(),
            };
            children.push(node);
        }
        if input[idx] == ',' {
            idx += 1;
            continue;
        }
    }

    return (children, idx);
}

fn node_to_string(node: &Node) -> String {
    let mut s = String::new();
    if node.value != -1 {
        s.push_str(node.value.to_string().as_str());
    } else {
        s.push('[');
        let num_children = node.children.len();
        for i in 0..num_children {
            let child = &node.children[i];
            s += node_to_string(child).as_str();
            if i < num_children - 1 {
                s += ",";
            }
        }
        s.push(']');
    }
    return s;
}

fn is_right_order_node(left: &mut Node, right: &mut Node) -> Ordering {
    let nums = (left.children.len(), right.children.len());
    if left.value >= 0 && right.value >= 0 {
        if left == right {
            return Ordering::Equal;
        }
        if left < right {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }

    if nums.0 > 0 && nums.1 > 0 {
        for i in 0..std::cmp::min(nums.0, nums.1) {
            let result = is_right_order_node(
                &mut left.children[i].clone(),
                &mut right.children[i].clone(),
            );
            if result == Ordering::Less {
                return Ordering::Less;
            } else if result == Ordering::Greater {
                return Ordering::Greater;
            }
        }

        // list neutral, check length
        if nums.0 == nums.1 {
            return Ordering::Equal;
        } else if nums.0 < nums.1 {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    if left.value >= 0 {
        left.children.push(Node {
            value: left.value,
            children: Vec::new(),
        });
        left.value = -1;
        return is_right_order_node(left, right);
    }

    if right.value >= 0 {
        right.children.push(Node {
            value: right.value,
            children: Vec::new(),
        });
        right.value = -1;
        return is_right_order_node(left, right);
    }

    if nums.0 == 0 && nums.1 > 0 {
        return Ordering::Greater;
    } else if nums.1 == 0 && nums.0 > 0 {
        return Ordering::Less;
    } else if nums == (0, 0) {
        return Ordering::Equal;
    }

    assert!(false);
    return Ordering::Equal;
}

fn is_right_order(left: &str, right: &str) -> bool {
    let (nodes_left, _) = parse_node(&left.chars().collect(), 0);
    let (nodes_right, _) = parse_node(&right.chars().collect(), 0);
    let node_left = nodes_left[0].clone();
    let node_right = nodes_right[0].clone();

    assert_eq!(String::from(left), node_to_string(&node_left));
    assert_eq!(String::from(right), node_to_string(&node_right));

    let result = is_right_order_node(&mut node_left.clone(), &mut node_right.clone());
    return result == Ordering::Greater;
}

fn part_one(input: &String) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let num_lines = lines.len();
    let mut sum = 0;
    for pair_i in 0..(num_lines + 1) / 3 {
        let i = pair_i * 3;
        let j = i + 1;

        let result = is_right_order(lines[i], lines[j]);
        if result {
            sum += 1 + pair_i;
        }
    }

    return sum;
}

fn part_two(input: &String) -> usize {
    let mut lines_sorted = Vec::new();
    {
        let mut lines: Vec<_> = input.lines().collect();
        lines.push("");
        lines.push("[[2]]");
        lines.push("[[6]]");
        let num_lines = lines.len();
        for pair_i in 0..(num_lines + 1) / 3 {
            let i = pair_i * 3;
            let j = i + 1;

            let (nodes_left, _) = parse_node(&lines[i].chars().collect(), 0);
            let (nodes_right, _) = parse_node(&lines[j].chars().collect(), 0);
            let node_left = nodes_left[0].clone();
            let node_right = nodes_right[0].clone();
            lines_sorted.push(node_left);
            lines_sorted.push(node_right);
        }
    }

    lines_sorted.sort_by(|a, b| is_right_order_node(&mut b.clone(), &mut a.clone()));

    for i in 0..lines_sorted.len() {
        if node_to_string(&lines_sorted[i]) != "[[2]]" {
            continue;
        }
        for j in i + 1..lines_sorted.len() {
            if node_to_string(&lines_sorted[j]) == "[[6]]" {
                return (i + 1) * (j + 1);
            }
        }
    }

    return 0;
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
        assert!(is_right_order("[1,1,3,1,1]", "[1,1,5,1,1]"));
        assert!(is_right_order("[[1],[2,3,4]]", "[[1],4]"));
        assert!(!is_right_order("[9]", "[[8,7,6]]"));
        assert!(is_right_order("[[4,4],4,4]", "[[4,4],4,4,4]"));
        assert!(!is_right_order("[7,7,7,7]", "[7,7,7]"));
        assert!(is_right_order("[]", "[3]"));
        assert!(!is_right_order("[[[]]]", "[[]]"));
        assert!(!is_right_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
        assert!(!is_right_order("[[9,9,9],2]", "[[1,1],3]"));
        assert!(is_right_order("[[],[],[[[3,6,6,1,5],9],[[7,2],[],[9,2,4],[7,8,10,0,3]],4,[[6,9,5,2]]],[6,6,[[4,10]]]]", "[[],[6]]"));
        assert!(!is_right_order(
            "[[0,[0,10,7],[0,4,[4,7,9,10],10],2,7]]",
            "[[[],10],[],[],[[8,10,[5,8,1],4]]]"
        ));
        assert!(!is_right_order(
            "[[0,2,2],[[[9],[8,5],[1,8,2,6,1]],[3,3]],[5,5,0,[9,3,[],[2,8,6],[]]],[4,[[8,6],[8,6,8,2,10],8,[6,10,5,0,8]],[],[[],2,1,[2,9,2,3],5],[]]]"
,            "[[[[],[6]],[],6,9],[[]],[]]"

        ));

        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            .to_string();
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 140);
    }
}
