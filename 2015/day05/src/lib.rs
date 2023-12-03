extern crate common;

fn is_nice_part_one(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<char>>();
    let bads = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];
    let mut contains_double = false;

    for i in 0..chars.len() - 1 {
        let c0 = chars[i];
        let c1 = chars[i + 1];

        if c0 == c1 {
            contains_double = true;
        }

        for bad in &bads {
            if &bad[..2] == [c0, c1] {
                return false;
            }
        }
    }

    if !contains_double {
        return false;
    }

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let num_vowels = chars
        .iter()
        .fold(0, |sum, v| sum + if vowels.contains(v) { 1 } else { 0 });

    return num_vowels >= 3;
}

fn is_nice_part_two(input: &str) -> bool {
    let chars = input.chars().collect::<Vec<char>>();
    let mut contains_repeat = false;

    for i in 0..chars.len() - 2 {
        let c0 = chars[i];
        let c1 = chars[i + 2];

        if c0 == c1 {
            contains_repeat = true;
            break;
        }
    }

    if !contains_repeat {
        return false;
    }

    for i in 0..chars.len() - 1 {
        let ca0 = chars[i];
        let ca1 = chars[i + 1];

        for j in i + 2..chars.len() - 1 {
            let cb0 = chars[j];
            let cb1 = chars[j + 1];

            if ca0 == cb0 && ca1 == cb1 {
                return true;
            }
        }
    }

    return false;
}

fn part_one(input: &String) -> usize {
    return input
        .lines()
        .fold(0, |sum, s| sum + if is_nice_part_one(s) { 1 } else { 0 });
}

fn part_two(input: &String) -> usize {
    return input
        .lines()
        .fold(0, |sum, s| sum + if is_nice_part_two(s) { 1 } else { 0 });
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
    fn test_samples_part_one() {
        assert_eq!(is_nice_part_one("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_part_one("aaa"), true);
        assert_eq!(is_nice_part_one("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_part_one("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_part_one("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(is_nice_part_two("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part_two("xxyxx"), true);
        assert_eq!(is_nice_part_two("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part_two("ieodomkazucvgmuy"), false);
    }
}
