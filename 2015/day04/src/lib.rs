extern crate common;
extern crate crypto;
extern crate rayon;

use crypto::digest::Digest;
use crypto::md5::Md5;
use rayon::prelude::*;

const INPUT: &str = "yzbqklnj";

fn hash_input(input: &str, index: usize) -> [u8; 16] {
    let mut hasher = Md5::new();
    let key = input.as_bytes();
    let mut output = [0; 16];

    hasher.input(key);
    hasher.input(index.to_string().as_bytes());
    hasher.result(&mut output);

    return output;
}

fn solve_internal(input: &str, num_zeroes: usize) -> usize {
    const STEP: usize = 2000;
    let mut ipt = [0; STEP];
    let mut curr_index = 0;
    loop {
        for i in 0..STEP {
            ipt[i] = curr_index + i;
        }

        let nums = ipt
            .par_iter()
            .map(|num| hash_input(input, *num).clone())
            .collect::<Vec<[u8; 16]>>();

        for i in curr_index..curr_index + STEP {
            //  let output = hash_input(input, i);
            let output = nums[i - curr_index];

            let mut success = true;
            for j in 0..num_zeroes {
                let even = j % 2 == 0;
                let hex_char = if even {
                    output[j / 2] >> 4
                } else {
                    (output[j / 2] << 4) >> 4
                };

                if hex_char != 0 {
                    success = false;
                    break;
                }
            }

            if success {
                return i;
            }
        }

        curr_index += STEP;
    }
}

pub fn solve() {
    println!("Part one: {}", solve_internal(INPUT, 5));
    println!("Part two: {}", solve_internal(INPUT, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(solve_internal("abcdef", 5), 609043);
    }
}
