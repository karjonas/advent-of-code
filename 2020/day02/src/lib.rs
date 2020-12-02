extern crate common;
#[macro_use]
extern crate scan_fmt;

type PasswordT = (i64, i64, char, String);

fn parse_passwords() -> Vec<PasswordT> {
    return common::read_file("2020/day02/input")
        .lines()
        .map(|line| scan_fmt!(line, "{d}-{d} {}: {}", i64, i64, char, String).unwrap())
        .collect();
}

fn solve_first(passwords: &Vec<PasswordT>) -> usize {
    let mut count = 0;
    for (min, max, letter, password) in passwords.clone() {
        let num_chars = password.matches(letter).count() as i64;
        if min <= num_chars && num_chars <= max {
            count += 1;
        }
    }
    return count;
}

fn solve_second(passwords: &Vec<PasswordT>) -> usize {
    let mut count = 0;
    for (min, max, letter, password) in passwords.clone() {
        let letter_min = password.chars().nth(min as usize - 1).unwrap();
        let letter_max = password.chars().nth(max as usize - 1).unwrap();

        if (letter_min == letter) ^ (letter_max == letter) {
            count += 1;
        }
    }
    return count;
}

pub fn solve() {
    let passwords = parse_passwords();
    println!("Part one: {:?}", solve_first(&passwords));
    println!("Part two: {:?}", solve_second(&passwords));
}
