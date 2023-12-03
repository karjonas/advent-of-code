extern crate common;

fn find_number(mut nums: Vec<i64>, width: usize, keep_most: bool) -> i64 {
    for i in 0..width {
        if nums.len() <= 1 {
            break;
        }

        let mut num_ones = 0;
        let mut num_zeroes = 0;
        let bit_idx = width - i - 1;

        for num in &nums {
            let bit_set = 0x1 & num >> bit_idx == 0x1;
            num_ones += if bit_set { 1 } else { 0 };
            num_zeroes += if !bit_set { 1 } else { 0 };
        }

        let mut ctr = 0;
        let keep_set = if keep_most {
            num_ones >= num_zeroes
        } else {
            num_ones < num_zeroes
        };

        while ctr < nums.len() {
            let num = nums[ctr];
            let bit_set = 0x1 & num >> bit_idx == 0x1;

            if bit_set != keep_set {
                nums.swap_remove(ctr);
            } else {
                ctr += 1;
            }
        }
    }

    return nums[0];
}

fn solve_internal_p1(input: &String, width: usize) -> i64 {
    let nums = input
        .lines()
        .map(|v| i32::from_str_radix(v, 2).unwrap())
        .collect::<Vec<_>>();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..width {
        let mut num_ones = 0;
        let mut num_zeroes = 0;
        for num in &nums {
            let bit_set = 0x1 & num >> i == 0x1;
            num_ones += if bit_set { 1 } else { 0 };
            num_zeroes += if !bit_set { 1 } else { 0 };
        }

        if num_ones > num_zeroes {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    return gamma * epsilon;
}

fn solve_internal_p2(input: &String, width: usize) -> i64 {
    let nums = input
        .lines()
        .map(|v| i64::from_str_radix(v, 2).unwrap())
        .collect::<Vec<_>>();

    let oxygen = find_number(nums.clone(), width, true);
    let carbon = find_number(nums.clone(), width, false);

    return oxygen * carbon;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", solve_internal_p1(&input, 12));
    println!("Part two: {}", solve_internal_p2(&input, 12));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(solve_internal_p1(&String::from(input), 5), 198);
        assert_eq!(solve_internal_p2(&String::from(input), 5), 230);
    }
}
