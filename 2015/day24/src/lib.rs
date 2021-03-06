extern crate common;
extern crate itertools;

use itertools::Itertools;

fn calc_qe(numbers: &Vec<usize>, num_groups: usize) -> usize {
    let sum = numbers.iter().fold(0, |sum, v| sum + v);
    assert!(sum % num_groups == 0);

    let goal = sum / num_groups;
    let mut min_quantum = std::usize::MAX;
    for group_size in 1.. {
        for group in numbers.iter().combinations(group_size) {
            if group.iter().fold(0, |sum, v| sum + v.clone()) == goal {
                min_quantum =
                    std::cmp::min(group.iter().fold(1, |sum, v| sum * v.clone()), min_quantum);
            }
        }
        if min_quantum != std::usize::MAX {
            break;
        }
    }
    return min_quantum;
}

pub fn solve() {
    let input = common::read_file("2015/day24/input");
    let vec = input
        .lines()
        .map(|v| common::string_to_i64(v) as usize)
        .collect::<Vec<usize>>();

    println!("Part one: {}", calc_qe(&vec, 3));
    println!("Part two: {}", calc_qe(&vec, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11].to_vec();

        assert_eq!(calc_qe(&input, 3), 99);
    }
}
