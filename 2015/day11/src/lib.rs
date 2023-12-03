extern crate common;

fn valid_password(password: &str) -> bool {
    let base = 'a' as u8;

    let v = password
        .chars()
        .map(|v| v as u8 - base)
        .collect::<Vec<u8>>();
    let length = v.len();

    let mut fail = true;
    for i in 0..(length - 2) {
        let first = v[i];
        if v[i + 1] == (first + 1) && v[i + 2] == (first + 2) {
            fail = false;
            break;
        }
    }

    if fail {
        return false;
    }

    for i in 0..length {
        if v[i] == ('i' as u8 - base) || v[i] == ('o' as u8 - base) || v[i] == ('l' as u8 - base) {
            return false;
        }
    }

    for i in 0..(length - 1) {
        let first = v[i];
        if first == v[i + 1] {
            for j in i + 2..(length - 1) {
                if v[j] == v[j + 1] && v[j] != first {
                    return true;
                }
            }
        }
    }

    return false;
}

fn next_valid_password(password: &str) -> String {
    let mut valid = false;
    let mut next = password.to_string();
    let len = password.len();

    let base = 'a' as u8;
    let end = 'z' as u8;

    while !valid {
        let mut chars = next.chars().map(|v| v as u8 - base).collect::<Vec<u8>>();

        for i_inv in 0..len {
            let i = len - 1 - i_inv;
            if chars[i] == (end - base) {
                chars[i] = 0;
            } else {
                chars[i] += 1;
                break;
            }
        }

        next = chars
            .iter()
            .map(|v| (*v + base) as char)
            .collect::<String>();

        valid = valid_password(next.as_str());
    }

    return next;
}

pub fn solve(_filepath: &str) {
    let p1 = next_valid_password("hxbxwxba");
    println!("Part one: {}", p1);
    println!("Part two: {}", next_valid_password(p1.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(valid_password("hijklmmn"), false);
        assert_eq!(valid_password("abbceffg"), false);
        assert_eq!(valid_password("abbcegjk"), false);
        assert_eq!(next_valid_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_valid_password("ghijklmn"), "ghjaabcc");
    }
}
