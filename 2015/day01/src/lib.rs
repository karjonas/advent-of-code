extern crate common;

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(input.as_str()));
    println!("Part two: {}", part_two(input.as_str()));
}

fn part_one(input: &str) -> i64 {
    return input
        .to_string()
        .chars()
        .fold(0, |sum, char_i| sum + if char_i == '(' { 1 } else { -1 });
}

fn part_two(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();

    let mut level: i64 = 0;
    for pos in 0..chars.len() {
        level += if chars[pos] == '(' { 1 } else { -1 };
        if level == -1 {
            return pos + 1;
        }
    }

    assert!(false);
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("()()"), 0);
        assert_eq!(part_one("((("), 3);
        assert_eq!(part_one("(()(()("), 3);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one("())"), -1);
        assert_eq!(part_one("))("), -1);
        assert_eq!(part_one(")))"), -3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}
