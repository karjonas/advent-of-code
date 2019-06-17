extern crate common;
extern crate unescape;

use unescape::unescape;

fn part_one(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let unescaped = unescape(line).unwrap();
        let len_prev = line.len();
        let len_after = unescaped.chars().fold(0, |sum, _v| sum + 1);
        let sum_curr = 2 + len_prev - len_after;
        sum += sum_curr;
    }
    return sum;
}

fn part_two(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let len_prev = line.chars().fold(0, |sum, _v| sum + 1);
        let escaped = format!("{:?}", line);
        let len_after = escaped.chars().fold(0, |sum, _v| sum + 1);
        sum += len_after - len_prev;
    }
    return sum;
}

pub fn solve() {
    println!(
        "Part one: {}",
        part_one(common::read_file("2015/day08/input").as_str())
    );
    println!(
        "Part two: {}",
        part_two(common::read_file("2015/day08/input").as_str())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(part_one("\"\""), 2);
        assert_eq!(part_one("\"abc\""), 2);
        assert_eq!(part_one("\"aaa\\\"aaa\""), 3);
        assert_eq!(part_one("\"\\x27\""), 5);

        assert_eq!(part_one("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), 12);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(part_two("\"\""), 4);
        assert_eq!(part_two("\"abc\""), 4);
        assert_eq!(part_two("\"aaa\\\"aaa\""), 6);
        assert_eq!(part_two("\"\\x27\""), 5);

        assert_eq!(part_two("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\""), 19);
    }

}
