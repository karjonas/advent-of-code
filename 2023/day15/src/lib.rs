#[macro_use]
extern crate scan_fmt;

fn parse_input(input: &String) -> Vec<String> {
    return input.split(',').map(|v| v.to_string()).collect();
}

fn hashed(s: &String) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value = value % 256;
    }
    return value;
}

fn part_one(sequence: &Vec<String>) -> usize {
    return sequence.iter().map(|s| hashed(s)).sum();
}

fn part_two(sequence: &Vec<String>) -> usize {
    let mut map: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    for s in sequence {
        let id = scan_fmt!(s, "{}-", String).unwrap_or(String::new());
        if !id.is_empty() && id != *s {
            let hash = hashed(&id);
            map[hash].retain(|(name, _value)| *name != id);
            continue;
        }

        let (id, num) = scan_fmt!(s, "{}={}", String, usize).unwrap();
        let hash = hashed(&id);
        let subidx = map[hash].iter().position(|(name, _value)| *name == id);
        match subidx {
            Some(value) => {
                map[hash][value] = (id, num);
            }
            None => {
                map[hash].push((id, num));
            }
        }
    }

    let mut sum = 0;
    let mut box_idx = 1;
    for vec in map {
        let mut idx = 1;
        for (_name, value) in vec {
            sum += box_idx * idx * value;
            idx += 1;
        }
        box_idx += 1;
    }

    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let sequence = parse_input(&input);

    println!("Part one: {}", part_one(&sequence));
    println!("Part two: {}", part_two(&sequence));
}
