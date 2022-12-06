extern crate common;
use std::collections::HashSet;

fn find_marker(input: &String, num_distinct: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for i in num_distinct - 1..chars.len() {
        let mut distinct = HashSet::new();
        for j in i + 1 - num_distinct..i + 1 {
            distinct.insert(chars[j]);
        }
        if distinct.len() == num_distinct {
            return i + 1;
        }
    }
    return 0;
}

fn find_marker_p1(input: &String) -> usize {
    return find_marker(input, 4);
}

fn find_marker_p2(input: &String) -> usize {
    return find_marker(input, 14);
}

pub fn solve() {
    let input = common::read_file("2022/day06/input");
    println!("Part one: {}", find_marker_p1(&input));
    println!("Part two: {}", find_marker_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(
            find_marker_p1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            5
        );
        assert_eq!(
            find_marker_p1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            6
        );
        assert_eq!(
            find_marker_p1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
        assert_eq!(
            find_marker_p1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );

        assert_eq!(
            find_marker_p2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            19
        );
        assert_eq!(
            find_marker_p2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            23
        );
        assert_eq!(
            find_marker_p2(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            23
        );
        assert_eq!(
            find_marker_p2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            29
        );
        assert_eq!(
            find_marker_p2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            26
        );
    }
}
