extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const INPUT: &str = "wtnhxymk";

fn to_hex_char(value: u8) -> char {
    assert!(value < 16);
    if value < 10 {
        return (value + '0' as u8) as char;
    } else {
        return ((value - 10) + 'a' as u8) as char;
    }
}

fn solve_internal(input: &str, part_two: bool) -> String {
    const NUM_CHARS: usize = 8;
    let mut hasher = Md5::new();
    let key = input.as_bytes();
    let mut hash = [0; 16];
    let mut hexes = [0; 32];
    let mut chars_collected = 0;
    let mut passwd_vec: Vec<char> = vec![' '; 8];

    for i in 0.. {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        hasher.result(&mut hash);

        for j in 0..4 {
            hexes[2 * j + 0] = hash[j] >> 4;
            hexes[2 * j + 1] = (hash[j] << 4) >> 4
        }

        if hexes[0] == 0 && hexes[1] == 0 && hexes[2] == 0 && hexes[3] == 0 && hexes[4] == 0 {
            if part_two {
                let pos = hexes[5] as usize;
                if pos < NUM_CHARS && passwd_vec[pos] == ' ' {
                    let hex_char = to_hex_char(hexes[6]);
                    passwd_vec[pos as usize] = hex_char;
                    chars_collected += 1;
                }
            } else {
                passwd_vec[chars_collected] = to_hex_char(hexes[5]);
                chars_collected += 1;
            }

            if chars_collected == NUM_CHARS {
                return passwd_vec.iter().collect::<String>();
            }
        }
        hasher.reset();
    }

    return String::new();
}

pub fn solve(_filepath: &str) {
    println!("Part one: {}", solve_internal(&INPUT, false));
    println!("Part two: {}", solve_internal(&INPUT, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(solve_internal("abc", false), "18f47a30".to_string());
    }
}
