extern crate common;

const PASSWORD_MIN: usize = 256310;
const PASSWORD_MAX: usize = 732736;

fn meets_criteria(password: usize, part_one: bool) -> bool {
    let chars: Vec<_> = password.to_string().chars().collect();
    let mut appearances = vec![0; 10];
    assert_eq!(chars.len(), 6);

    let mut prev_num = '0';
    for c in chars {
        appearances[(c as u8 - '0' as u8) as usize] += 1;
        if prev_num > c {
            return false;
        }
        prev_num = c;
    }

    for num in appearances {
        if (part_one && num >= 2) || num == 2 {
            return true;
        }
    }
    return false;
}

fn solve_internal(part_one: bool) -> usize {
    let mut counter = 0;
    for password in PASSWORD_MIN..(PASSWORD_MAX + 1) {
        counter += meets_criteria(password, part_one) as usize;
    }
    return counter;
}

pub fn solve() {
    println!("Part one: {}", solve_internal(true));
    println!("Part two: {}", solve_internal(false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(true, meets_criteria(122345, true));
        assert_eq!(true, meets_criteria(111123, true));
        assert_eq!(true, meets_criteria(111111, true));

        assert_eq!(false, meets_criteria(223450, true));
        assert_eq!(false, meets_criteria(123789, true));
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(true, meets_criteria(112233, false));
        assert_eq!(false, meets_criteria(123444, false));
        assert_eq!(true, meets_criteria(111122, false));
    }
}
