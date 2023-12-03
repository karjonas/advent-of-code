fn generate_dragon_code(input: String, min_length: usize) -> Vec<bool> {
    let mut output: Vec<bool> = input
        .chars()
        .map(|c| if c == '1' { true } else { false })
        .collect();

    while output.len() < min_length {
        let nums = output.len();
        output.push(false);
        for i in 0..nums {
            let v = !output[nums - i - 1];
            output.push(v);
        }
    }
    output.resize(min_length, false);
    return output;
}

fn calculate_checksum(input: Vec<bool>) -> Vec<bool> {
    let mut output: Vec<bool> = input;

    while output.len() % 2 == 0 {
        let mut next: Vec<bool> = Vec::new();
        for i in 0..output.len() / 2 {
            next.push(output[2 * i] == output[2 * i + 1]);
        }
        output = next;
    }

    return output;
}

pub fn solve(_filepath: &str) {
    {
        let c = generate_dragon_code("10001110011110000".to_string(), 272);
        let cv = calculate_checksum(c);
        let mut s = String::new();
        for v in cv {
            s.push(if v { '1' } else { '0' });
        }

        println!("Part 1: {}", s);
    }

    {
        let c = generate_dragon_code("10001110011110000".to_string(), 35651584);
        let cv = calculate_checksum(c);
        let mut s = String::new();
        for v in cv {
            s.push(if v { '1' } else { '0' });
        }

        println!("Part 2: {}", s);
    }
}
