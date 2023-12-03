extern crate common;

fn to_snafu(goal: i64) -> String {
    let mut index = 26 as i32;
    let mut curr_number = 0 as i64;
    let mut first = true;
    let mut snafu = String::new();

    while index >= 0 {
        // try all numbers and see if they put us closer to the goal
        let mut curr_c = '0';
        let mut curr_diff = curr_number.abs_diff(goal);
        let prev_value = curr_number;

        for (c, value) in [('0', 0), ('1', 1), ('2', 2), ('=', -2), ('-', -1)] {
            let number = prev_value + (5 as i64).pow(index as u32) * value;
            let diff = number.abs_diff(goal);
            if diff < curr_diff {
                curr_c = c;
                curr_number = number;
                curr_diff = diff;
            }
        }

        if !first || curr_c != '0' {
            snafu.push(curr_c);
            first = false;
        }

        index -= 1;
    }

    return snafu;
}

fn from_snafu(input: &str) -> i64 {
    let mut number = 0;
    let mut index = 0;
    for c in input.chars().rev() {
        let value = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => panic!(),
        };
        number += (5 as i64).pow(index) * value;
        index += 1;
    }
    return number;
}

fn part_one(input: &String) -> String {
    let sum = input.lines().fold(0, |sum, line| sum + from_snafu(line));
    return to_snafu(sum);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
        .to_string();
        assert_eq!(part_one(&input), "2=-1=0");
    }
}
