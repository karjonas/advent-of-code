extern crate common;
#[macro_use]
extern crate scan_fmt;

fn solve_internal_p1(input: &Vec<String>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    for line in input {
        let (dir, val) = scan_fmt!(line, "{} {d}", String, i64).unwrap();

        if dir == "forward" {
            x += val;
        } else if dir == "up" {
            y -= val;
        } else if dir == "down" {
            y += val;
        }
    }

    return x * y;
}

fn solve_internal_p2(input: &Vec<String>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in input {
        let (dir, val) = scan_fmt!(line, "{} {d}", String, i64).unwrap();

        if dir == "forward" {
            x += val;
            y += aim * val;
        } else if dir == "up" {
            aim -= val;
        } else if dir == "down" {
            aim += val;
        }
    }

    return x * y;
}

pub fn solve(filepath: &str) {
    let input: Vec<String> = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string()
        .lines()
        .map(|s| String::from(s))
        .collect();

    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2"),
        ]
        .to_vec();

        assert_eq!(solve_internal_p1(&input), 150);
        assert_eq!(solve_internal_p2(&input), 900);
    }
}
