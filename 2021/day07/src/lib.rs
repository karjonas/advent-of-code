extern crate common;

fn solve_internal(input: &String, part_two: bool) -> usize {
    let numbers: Vec<usize> = input
        .split(",")
        .map(|v| common::string_to_usize(v))
        .collect();

    let min = numbers.iter().fold(0, |acc, x| std::cmp::min(acc, *x));
    let max = numbers.iter().fold(0, |acc, x| std::cmp::max(acc, *x));

    let mut costs = vec![0];
    for i in 1..max - min + 1 {
        if part_two {
            costs.push(i + costs[i - 1]);
        } else {
            costs.push(i);
        }
    }

    let mut best = std::usize::MAX;
    for pos in min..max {
        let mut curr = 0;
        for &num in &numbers {
            let dist = std::cmp::max(pos, num) - std::cmp::min(pos, num);
            curr += costs[dist];
        }
        best = std::cmp::min(curr, best);
    }

    return best;
}

fn solve_internal_p1(input: &String) -> usize {
    return solve_internal(input, false);
}

fn solve_internal_p2(input: &String) -> usize {
    return solve_internal(input, true);
}

pub fn solve() {
    let input = common::read_file("2021/day07/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(solve_internal(&String::from(input), false), 37);
        assert_eq!(solve_internal(&String::from(input), true), 168);
    }
}
