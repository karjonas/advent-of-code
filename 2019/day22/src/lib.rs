extern crate common;

use std::collections::VecDeque;

const NUM_CARDS: i128 = 119315717514047;
const NUM_SHUFFLES: i128 = 101741582076661;
const FINAL_POS: i128 = 2020;

fn modulo(a: i128, b: i128) -> i128 {
    return if a >= 0 { a % b } else { b + a % b };
}

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = gcd_extended(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    return (gcd, x, y);
}

fn modular_inverse(b: i128, n: i128) -> i128 {
    let (g, x, _y) = gcd_extended(b, n);
    return if g != 1 { -1 } else { modulo(x, n) };
}

fn modular_divide(a: i128, b: i128, n: i128) -> i128 {
    let a = modulo(a, n);
    let inv = modular_inverse(b, n);
    return if inv == -1 { -1 } else { (a * inv) % n };
}

fn modular_power(base: i128, exponent: i128, n: i128) -> i128 {
    assert!(exponent >= 0);
    if exponent == 0 {
        return if base == 0 { 0 } else { 1 };
    }

    let mut bit = 1;
    let mut power = modulo(base, n);
    let mut out = 1;
    while bit <= exponent {
        if exponent & bit > 0 {
            out = modulo(out * power, n);
        }
        power = modulo(power * power, n);
        bit <<= 1;
    }

    return out;
}

fn solve_part_one(input: String) -> usize {
    let result = run_commands(&input, &generate_deck(10007));

    for i in 0..result.len() {
        if result[i] == 2019 {
            return i;
        }
    }

    panic!("Could not find card 2019");
}

fn solve_part_two(input: String) -> usize {
    let mut a_tot = 1;
    let mut b_tot = 0;

    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        let mut a: i128 = 0;
        let mut b: i128 = 0;

        if words[0] == "cut" {
            let mut num = common::string_to_i64(words[1]) as i128;
            if num < 0 {
                num = num + NUM_CARDS;
            }
            a = 1;
            b = NUM_CARDS - num;
        } else if words[0] == "deal" && words[1] == "with" {
            let num = common::string_to_i64(words[3]) as i128;
            a = num;
            b = 0;
        } else if words[0] == "deal" && words[1] == "into" {
            a = -1;
            b = NUM_CARDS - 1;
        }
        a_tot = modulo(a * a_tot, NUM_CARDS);
        b_tot = modulo(a * b_tot + b, NUM_CARDS);
    }

    let full_a = modular_power(a_tot, NUM_SHUFFLES, NUM_CARDS);
    let full_b = modulo(
        b_tot
            * modular_divide(
                modular_power(a_tot, NUM_SHUFFLES, NUM_CARDS) - 1,
                a_tot - 1,
                NUM_CARDS,
            ),
        NUM_CARDS,
    );

    let start_pos = modulo(
        modular_divide(modulo(FINAL_POS - full_b, NUM_CARDS), full_a, NUM_CARDS),
        NUM_CARDS,
    );

    return start_pos as usize;
}

fn deal(input: VecDeque<i32>) -> VecDeque<i32> {
    let mut result = VecDeque::new();
    result.reserve(input.len());

    for v in input {
        result.push_front(v);
    }

    return result;
}

fn cut(input: VecDeque<i32>, n: i32) -> VecDeque<i32> {
    let mut result = VecDeque::new();
    result.reserve(input.len());

    if n < 0 {
        let start_idx = (input.len() as i32 + n) as usize;
        for idx in start_idx..input.len() {
            result.push_back(input[idx]);
        }
        for idx in 0..start_idx {
            result.push_back(input[idx]);
        }
    } else {
        for idx in (n as usize)..input.len() {
            result.push_back(input[idx]);
        }
        for idx in 0..(n as usize) {
            result.push_back(input[idx]);
        }
    }

    return result;
}

fn increment(input: VecDeque<i32>, n: usize) -> VecDeque<i32> {
    let mut result = VecDeque::new();
    let len = input.len();
    result.resize(len, -1);

    let mut index = 0;
    for v in input {
        result[index] = v;
        index = (index + n) % len;
    }

    return result;
}

fn generate_deck(num_cards: usize) -> VecDeque<i32> {
    let mut result = VecDeque::new();
    for i in 0..num_cards {
        result.push_back(i as i32);
    }
    return result;
}

fn run_commands(input: &String, deck: &VecDeque<i32>) -> VecDeque<i32> {
    let mut result = deck.clone();
    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        if words[0] == "deal" && words[1] == "into" {
            result = deal(result.clone());
        } else if words[0] == "deal" && words[1] == "with" {
            let num = common::string_to_i64(words[3]) as usize;
            result = increment(result.clone(), num);
        } else if words[0] == "cut" {
            let num = common::string_to_i64(words[1]) as i32;
            result = cut(result.clone(), num);
        }
    }
    return result;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", solve_part_one(input.clone()));
    println!("Part two: {}", solve_part_two(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let deck: VecDeque<i32> = VecDeque::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec());
        assert_eq!(
            deal(deck.clone()),
            VecDeque::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0].to_vec())
        );
        assert_eq!(
            cut(deck.clone(), 3),
            VecDeque::from([3, 4, 5, 6, 7, 8, 9, 0, 1, 2].to_vec())
        );
        assert_eq!(
            cut(deck.clone(), -4),
            VecDeque::from([6, 7, 8, 9, 0, 1, 2, 3, 4, 5].to_vec())
        );
        assert_eq!(
            increment(deck.clone(), 3),
            VecDeque::from([0, 7, 4, 1, 8, 5, 2, 9, 6, 3].to_vec())
        );
    }
    #[test]
    fn part_one_b() {
        let input_a: String = [
            "deal with increment 7",
            "deal into new stack",
            "deal into new stack",
        ]
        .join("\n");

        let input_b: String = ["cut 6", "deal with increment 7", "deal into new stack"].join("\n");

        let input_c: String =
            ["deal with increment 7", "deal with increment 9", "cut -2"].join("\n");

        let input_d: String = [
            "deal into new stack",
            "cut -2",
            "deal with increment 7",
            "cut 8",
            "cut -4",
            "deal with increment 7",
            "cut 3",
            "deal with increment 9",
            "deal with increment 3",
            "cut -1",
        ]
        .join("\n");

        assert_eq!(
            run_commands(&input_a, &generate_deck(10)),
            VecDeque::from([0, 3, 6, 9, 2, 5, 8, 1, 4, 7].to_vec())
        );
        assert_eq!(
            run_commands(&input_b, &generate_deck(10)),
            VecDeque::from([3, 0, 7, 4, 1, 8, 5, 2, 9, 6].to_vec())
        );
        assert_eq!(
            run_commands(&input_c, &generate_deck(10)),
            VecDeque::from([6, 3, 0, 7, 4, 1, 8, 5, 2, 9].to_vec())
        );
        assert_eq!(
            run_commands(&input_d, &generate_deck(10)),
            VecDeque::from([9, 2, 5, 8, 1, 4, 7, 0, 3, 6].to_vec())
        );
    }
}
