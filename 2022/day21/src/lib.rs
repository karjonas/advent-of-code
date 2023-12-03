use std::collections::HashMap;

extern crate common;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Job {
    left: String,
    right: String,
    operator: char,
    number: i64,
}

fn parse(input: &String) -> HashMap<String, Job> {
    let mut jobs = HashMap::new();
    for line in input.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let n = words.len();
        let name = words[0].to_string().replace(":", "");
        let number = words[1].to_string().parse::<i64>().unwrap_or(0);
        let mut left = String::new();
        let mut operator = ' ';
        let mut right = String::new();
        if n > 2 {
            left = words[1].to_string();
            operator = words[2].chars().next().unwrap();
            right = words[3].to_string();
        }

        jobs.insert(
            name,
            Job {
                left: left,
                right: right,
                operator: operator,
                number: number,
            },
        );
    }

    return jobs;
}

fn get_value(name: &String, jobs: &HashMap<String, Job>) -> (i64, bool) {
    let job = jobs.get(name).unwrap();

    if job.operator == ' ' {
        return (job.number, name == "humn");
    }

    let (left, left_human) = get_value(&job.left, jobs);
    let (right, right_human) = get_value(&job.right, jobs);

    let value = match job.operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => 0,
    };

    return (value, left_human || right_human);
}

fn calc_human_value(name: &String, goal: i64, jobs: &HashMap<String, Job>) -> i64 {
    let job = jobs.get(name).unwrap();
    if name == "humn" {
        return goal;
    }

    assert_ne!(job.operator, ' ');

    let (left, left_human) = get_value(&job.left, jobs);
    let (right, right_human) = get_value(&job.right, jobs);

    assert_ne!(left_human, right_human);

    let goal_next;
    let name_next;

    if left_human {
        goal_next = match job.operator {
            '+' => goal - right, // left + right = goal
            '-' => goal + right, // left - right = goal
            '*' => goal / right, // left * right = goal
            '/' => right * goal, // left / right = goal
            _ => 0,
        };
        name_next = job.left.clone();
    } else {
        goal_next = match job.operator {
            '+' => goal - left, // left + right = goal
            '-' => left - goal, // left - right = goal
            '*' => goal / left, // left * right = goal
            '/' => left / goal, // left / right = goal
            _ => 0,
        };
        name_next = job.right.clone();
    };

    return calc_human_value(&name_next, goal_next, jobs);
}

fn part_one(input: &String) -> i64 {
    let jobs = parse(input);
    let (v, _is_human) = get_value(&"root".to_string(), &jobs);
    return v;
}

fn part_two(input: &String) -> i64 {
    let jobs = parse(input);
    let root = jobs.get(&"root".to_string()).unwrap();
    let left = root.left.clone();
    let right = root.right.clone();

    let (left_number, is_left_human) = get_value(&left, &jobs);
    let (right_number, _is_right_human) = get_value(&right, &jobs);

    return if is_left_human {
        calc_human_value(&left, right_number, &jobs)
    } else {
        calc_human_value(&right, left_number, &jobs)
    };
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
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            .to_string();
        assert_eq!(part_one(&input), 152);
        assert_eq!(part_two(&input), 301);
    }
}
