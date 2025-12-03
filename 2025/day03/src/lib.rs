fn parse_input(input: &String) -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let mut line_numbers = Vec::new();
        for c in line.chars() {
            let digit = c as u8 as i64 - 48;
            assert!(digit >= 0 && digit <= 9);
            line_numbers.push(digit);
        }
        result.push(line_numbers);
    }
    return result;
}

fn part_one(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;
    for line in lines {
        let mut start_idx = 0;
        let mut start_num = 0;
        for i in 0..(line.len() - 1) {
            if line[i] > start_num {
                start_num = line[i];
                start_idx = i;
            }
        }
        let mut end_num = 0;
        for i in (start_idx + 1)..line.len() {
            if line[i] > end_num {
                end_num = line[i];
            }
        }
        let jolt = start_num * 10 + end_num;
        result += jolt;
    }
    result
}

fn part_two(lines: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;
    for line in lines {
        let mut start_idx = 0;
        let mut digits_remaining = 12;
        let mut jolt = 0;
        for _ in 0..12 {
            let mut best_digit = 0;
            for i in start_idx..(line.len() - digits_remaining + 1) {
                if line[i] > best_digit {
                    best_digit = line[i];
                    start_idx = i + 1;
                }
            }

            jolt = 10 * jolt + best_digit;
            digits_remaining -= 1;
        }
        result += jolt;
    }
    result
}

pub fn solve(filepath: &str) {
    let input = parse_input(
        &std::fs::read_to_string(filepath)
            .unwrap()
            .trim_end_matches('\n')
            .to_string(),
    );

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
