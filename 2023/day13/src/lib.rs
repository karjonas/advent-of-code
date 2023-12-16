fn parse_input(input: &String) -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::new();
    let mut pattern = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            result.push(pattern.clone());
            pattern.clear();
            continue;
        }
        pattern.push(line.chars().collect());
    }

    if !pattern.is_empty() {
        result.push(pattern)
    }

    return result;
}

fn flipped_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut pattern_flipped = Vec::new();
    for idx in 0..pattern[0].len() {
        let mut line_flipped = Vec::new();
        for line in pattern {
            line_flipped.push(line[idx]);
        }
        pattern_flipped.push(line_flipped);
    }
    return pattern_flipped;
}

fn find_reflection(pattern: &Vec<Vec<char>>, is_flipped: bool, ignore: usize) -> usize {
    let num = pattern.len();
    for i in 0..(num - 1) {
        let mut success = true;
        for offset in 0..num / 2 {
            if offset > i {
                break;
            }
            if offset + i + 1 >= num {
                break;
            }

            if pattern[i - offset] != pattern[i + 1 + offset] {
                success = false;
            }
        }

        let value = if !is_flipped { 100 * (i + 1) } else { i + 1 };

        if success && value != ignore {
            return value;
        }
    }

    return 0;
}

fn num_diffs(a: &Vec<char>, b: &Vec<char>) -> usize {
    assert!(a.len() == b.len());
    let mut sum = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            sum += 1;
        }
    }
    return sum;
}

fn get_smudged(pattern: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::new();
    let num_lines = pattern.len();

    for i in 0..num_lines {
        for j in i + 1..num_lines {
            if num_diffs(&pattern[i], &pattern[j]) == 1 {
                let mut smudged = pattern.clone();
                smudged[i] = smudged[j].clone();
                result.push(smudged);
            }
        }
    }

    return result;
}

fn smudged_value(pattern: &Vec<Vec<char>>, value_original: usize, is_flipped: bool) -> usize {
    let smudged_patterns = get_smudged(pattern);
    let mut found_value = 0;

    for smudged in smudged_patterns {
        let value = find_reflection(&smudged, is_flipped, value_original);
        if value > 0 {
            assert!(found_value == 0 || found_value == value);
            found_value = value;
        }
    }

    return found_value;
}

fn part_one(patterns: &Vec<Vec<Vec<char>>>) -> usize {
    let mut sum = 0;
    for pattern in patterns {
        let mut value = find_reflection(pattern, false, 0);
        value += find_reflection(&flipped_pattern(pattern), true, 0);
        assert!(value > 0);
        sum += value;
    }
    return sum;
}

fn part_two(patterns: &Vec<Vec<Vec<char>>>) -> usize {
    let mut sum = 0;
    for pattern in patterns {
        let mut value_original = find_reflection(pattern, false, 0);
        let flipped = flipped_pattern(pattern);
        if value_original == 0 {
            value_original = find_reflection(&flipped, true, 0);
        }
        let mut value = smudged_value(pattern, value_original, false);
        if value == 0 {
            value = smudged_value(&flipped, value_original, true);
        }
        assert!(value > 0);
        sum += value;
    }
    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let patterns = parse_input(&input);

    println!("Part one: {}", part_one(&patterns));
    println!("Part two: {}", part_two(&patterns));
}
