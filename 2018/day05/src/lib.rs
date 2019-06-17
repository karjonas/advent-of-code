extern crate common;

fn react(chars: Vec<char>) -> usize {
    let mut alive: Vec<bool> = common::filled_vector(chars.len(), true);
    let mut i = 0;

    while i < chars.len() {
        if !alive[i] {
            i += 1;
            continue;
        }

        // Find next alive character
        let mut j = i;
        for i_next in i + 1..chars.len() {
            if alive[i_next] {
                j = i_next;
                break;
            }
        }

        // None found, done
        if j == i {
            break;
        }

        // Characters match, mark dead
        if chars[i] != chars[j] && common::uppercase(chars[i]) == common::uppercase(chars[j]) {
            alive[i] = false;
            alive[j] = false;

            // Rewind counter to previous alive
            while i > 0 {
                i -= 1;
                if alive[i] {
                    break;
                }
            }
        } else {
            i += 1;
        }
    }

    return alive
        .iter()
        .fold(0, |sum, &val| if val { sum + 1 } else { sum });
}

pub fn solve() {
    let input = common::read_file("2018/day05/input").trim().to_string();
    let chars: Vec<char> = input.chars().collect();
    println!("Part one: {:?}", react(chars.clone()));

    let mut best_value = std::usize::MAX;
    for i in 0..26 {
        let c = ('a' as u8 + i as u8) as char;
        let c_lower = common::uppercase(c);
        let c_upper = common::uppercase(c);
        let chars_curr = chars
            .clone()
            .into_iter()
            .filter(|&x| x != c_lower && x != c_upper)
            .collect();
        let res = react(chars_curr);
        if res < best_value {
            best_value = res;
        }
    }

    println!("Part two: {:?}", best_value);
}
