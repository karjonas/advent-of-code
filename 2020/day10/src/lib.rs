extern crate common;

use std::collections::HashMap;

fn parse_input(input: &String) -> Vec<usize> {
    let mut list: Vec<usize> = input.lines().map(|v| v.parse::<usize>().unwrap()).collect();
    list.insert(0, 0);
    list.sort_unstable();
    let joltage_max = list.last().unwrap() + 3;
    list.push(joltage_max);
    return list;
}

fn part_one(numbers: &Vec<usize>) -> usize {
    let mut num_one = 0;
    let mut num_three = 0;

    let mut curr_adapter = 0;
    for number in numbers {
        let diff = number - curr_adapter;
        if diff == 1 {
            num_one += 1;
        } else if diff == 3 {
            num_three += 1;
        }
        curr_adapter = *number;
    }

    return num_one * num_three;
}

fn part_two(numbers: &Vec<usize>) -> usize {
    let final_number = *numbers.last().unwrap();
    let mut hits: HashMap<usize, usize> = HashMap::new();

    hits.insert(numbers[0], 1);

    for idx in numbers {
        let curr_hits = *hits.get(idx).unwrap();

        for step in 1..4 {
            let idx_next = idx + step;
            if numbers.contains(&idx_next) {
                *hits.entry(idx_next).or_insert(0) += curr_hits;
            }
        }
    }

    return *hits.get(&final_number).unwrap();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let numbers = parse_input(&input);
    println!("Part one: {}", part_one(&numbers));
    println!("Part two: {}", part_two(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = ["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"].join("\n");
        let input_b = [
            "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45",
            "19", "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34",
            "10", "3",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 7 * 5);
        assert_eq!(part_one(&parse_input(&input_b)), 22 * 10);
        assert_eq!(part_two(&parse_input(&input)), 8);
        assert_eq!(part_two(&parse_input(&input_b)), 19208);
    }
}
