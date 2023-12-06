fn parse_input(input: &String) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    let mut numbers = Vec::new();
    for word in input.split_whitespace() {
        match word.parse::<i32>() {
            Ok(number) => numbers.push(number),
            _ => {}
        }
    }
    assert!(0 == numbers.len() % 2);
    for i in 0..numbers.len() / 2 {
        result.push((numbers[i], numbers[i + numbers.len() / 2]));
    }

    return result;
}

fn solve_equation(t: f64, d: f64) -> usize {
    let x0 = (t + (t * t - 4.0 * d).sqrt()) * 0.5;
    let x1 = (t - (t * t - 4.0 * d).sqrt()) * 0.5;
    let x_start = x0.min(x1).ceil() as usize;
    let x_stop = x0.max(x1).floor() as usize;
    return 1 + x_stop - x_start;
}

fn part_one(pairs: &Vec<(i32, i32)>) -> usize {
    let mut sum = 1;
    for (time, distance) in pairs {
        sum = sum * solve_equation(*time as f64, *distance as f64);
    }
    return sum;
}

fn part_two(pairs: &Vec<(i32, i32)>) -> usize {
    let mut time_s = String::new();
    let mut distance_s = String::new();
    for (time_i, distance_i) in pairs {
        time_s += time_i.to_string().as_str();
        distance_s += distance_i.to_string().as_str();
    }
    let time = time_s.parse::<usize>().unwrap();
    let distance = distance_s.parse::<usize>().unwrap();

    return solve_equation(time as f64, distance as f64);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let pairs = parse_input(&input);

    println!("Part one: {}", part_one(&pairs));
    println!("Part two: {}", part_two(&pairs));
}
