fn calc_score_recursive(
    chars: &Vec<char>,
    start_idx: usize,
    depth: usize,
) -> (usize, usize, usize) {
    let mut score = depth;
    let mut canceled = 0;
    let mut i = start_idx;
    while i < chars.len() {
        let curr_char = chars[i];
        if curr_char == '<' {
            let mut j = i + 1;
            while j < chars.len() {
                let char_j = chars[j];
                if char_j == '>' {
                    i = j + 1;
                    break;
                } else if char_j == '!' {
                    j += 2;
                } else {
                    canceled += 1;
                    j += 1;
                }
            }
        } else if curr_char == '{' {
            let (s, idx_next, c) = calc_score_recursive(&chars, i + 1, depth + 1);
            score += s;
            canceled += c;
            i = idx_next;
        } else if curr_char == '}' {
            return (score, i + 1, canceled);
        } else {
            i += 1;
        }
    }

    return (score, i, canceled);
}

fn calc_score(chars: &Vec<char>) -> (usize, usize) {
    let (score, _, canceled) = calc_score_recursive(chars, 0, 0);
    return (score, canceled);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let chars: Vec<_> = input.chars().collect();
    let (score, canceled) = calc_score(&chars);

    println!("Part one: {}", score);
    println!("Part two: {}", canceled);
}
