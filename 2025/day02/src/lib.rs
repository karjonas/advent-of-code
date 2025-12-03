fn parse_input(input: &String) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    for line in input.split(',') {
        let mut from_to = line.split('-').take(2);
        let from = from_to.next().unwrap().parse::<i64>().unwrap();
        let to = from_to.next().unwrap().parse::<i64>().unwrap();
        ranges.push((from, to));
    }
    return ranges;
}

fn round_up_to_next_power_of_ten(n: i64) -> i64 {
    let mut v = 1;
    while v < n {
        v *= 10;
    }
    return v;
}

fn part_one(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut count = 0;

    for (mut from, mut to) in ranges {
        let mut from_width = from.to_string().len() as u32;
        let mut to_width = to.to_string().len() as u32;
        if from_width % 2 == 1 {
            from = round_up_to_next_power_of_ten(from);
            from_width = from.to_string().len() as u32;
        }
        if to_width % 2 == 1 {
            to = 0;
            for _ in 0..(to_width - 1) {
                to *= 10;
                to += 9;
            }
            to_width = to.to_string().len() as u32;
        }

        let from_half = from / (10 as i64).pow(from_width / 2);
        let to_half = to / (10 as i64).pow(to_width / 2);

        if from > to {
            continue;
        }

        for i in from_half..(to_half + 1) {
            let curr_w = i.to_string().len() as u32;
            let expanded = i + i * (10 as i64).pow(curr_w);
            if expanded >= from && expanded <= to {
                count += expanded;
            }
        }
    }

    count
}

#[inline(always)]
fn digits_into_buf(mut n: i64, buf: &mut [u8; 20]) -> &[u8] {
    let mut i = buf.len();

    if n == 0 {
        buf[i - 1] = b'0';
        return &buf[i - 1..];
    }

    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    &buf[i..]
}

fn width_repeats(chars: &[u8], width: usize) -> bool {
    let mut offset = 0;
    while offset < chars.len() {
        for i in 0..width {
            if chars[i] != chars[offset + i] {
                return false;
            }
        }
        offset += width;
    }
    true
}

fn part_two(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut result = 0;

    for (from, to) in ranges {
        for value in *from..(*to + 1) {
            let mut buf = [0u8; 20];
            let chars = digits_into_buf(value, &mut buf);

            let num_chars = chars.len();
            let mut hit = false;
            for width in 1..num_chars {
                if num_chars % width != 0 {
                    continue;
                }

                if width_repeats(chars, width) {
                    hit = true;
                    break;
                }
            }

            if hit {
                result += value;
            }
        }
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
