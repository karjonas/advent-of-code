extern crate common;

const GOAL_SUM: usize = 150;

fn solve_internal(input: &String, goal_sum: usize) -> (usize, usize) {
    let numbers = input
        .lines()
        .map(|line| common::string_to_usize(line))
        .collect::<Vec<usize>>();

    let all_subs = common::subsequences(&numbers, 0, &Vec::new());

    let mut num_matched = 0;
    let mut num_matched_min = 0;
    let mut min_containers = std::usize::MAX;
    for perm in &all_subs {
        let sum = perm.iter().fold(0, |sum, v| sum + v);
        if sum == goal_sum {
            num_matched += 1;
            if perm.len() < min_containers {
                num_matched_min = 0;
                min_containers = perm.len();
            }

            if perm.len() == min_containers {
                num_matched_min += 1;
            }
        }
    }

    return (num_matched, num_matched_min);
}

pub fn solve() {
    let input = common::read_file("2015/day17/input");
    let (p1, p2) = solve_internal(&input, GOAL_SUM);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "20\n15\n10\n5\n5";
        assert_eq!(solve_internal(&input.to_string(), 25), (4, 3));
    }
}
