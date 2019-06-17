extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const INPUT: &str = "qzyelonm";
use std::collections::VecDeque;

struct Hash {
    data: [char; 32],
    fives: Vec<char>,
}

fn hash_input(input: &str, idx: usize, num_repeats: usize) -> Hash {
    let start: String = input.to_string() + idx.to_string().as_str();

    let mut hasher = Md5::new();
    let mut curr_hash = start.clone();

    for _ in 0..num_repeats + 1 {
        hasher.input_str(curr_hash.as_str());
        curr_hash = hasher.result_str();
        hasher.reset();
    }

    let mut hexes = ['0'; 32];
    let mut ctr = 0;
    for c in curr_hash.chars() {
        hexes[ctr] = c;
        ctr += 1;
    }

    let mut fives = Vec::new();

    for x in 0..(hexes.len() - 4) {
        let v = hexes[x];
        if v == hexes[x + 1] && v == hexes[x + 2] && v == hexes[x + 3] && v == hexes[x + 4] {
            fives.push(v);
        }
    }

    return Hash {
        data: hexes,
        fives: fives,
    };
}

fn any_fives_in_deque(v: char, deque: &VecDeque<Hash>) -> bool {
    for hash_s in deque {
        if hash_s.fives.contains(&v) {
            return true;
        }
    }

    return false;
}

fn solve_internal(num_repeats: usize) -> usize {
    let mut last_thousand_hashes: VecDeque<Hash> = VecDeque::new();

    // Fill first thousand
    for i in 0..1001 {
        last_thousand_hashes.push_back(hash_input(INPUT, i, num_repeats));
    }

    let mut pwd_ctr = 0;
    for i in 0.. {
        let hash_front = last_thousand_hashes.pop_front().unwrap();
        {
            let hash = hash_front.data;

            for x in 0..(hash.len() - 2) {
                if hash[x] == hash[x + 1] && hash[x] == hash[x + 2] {
                    if any_fives_in_deque(hash[x], &last_thousand_hashes) {
                        pwd_ctr += 1;
                        if pwd_ctr == 64 {
                            return i;
                        }
                    }
                    break;
                }
            }
        }

        let hash_s = hash_input(INPUT, 1001 + i, num_repeats);
        last_thousand_hashes.push_back(hash_s);
    }
    return 0;
}

pub fn solve() {
    println!("Part 1: {}", solve_internal(0));
    println!("Part 2: {}", solve_internal(2016));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        {
            let hash = hash_input("abc", 0, 0);
            let s = hash.data.iter().collect::<String>();
            assert_eq!(s, "577571be4de9dcce85a041ba0410f29f");
        }
        {
            let hash = hash_input("abc", 0, 1);
            let s = hash.data.iter().collect::<String>();
            assert_eq!(s, "eec80a0c92dc8a0777c619d9bb51e910");
        }
    }
}
