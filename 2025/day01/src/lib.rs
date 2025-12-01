fn parse_input(input: &String) -> Vec<i32> {
    let mut actions: Vec<i32> = Vec::new();
    for line in input.lines() {
        let dir: char = (&line[0..1]).parse::<char>().unwrap();
        let mut number: i32 = (&line[1..]).parse::<i32>().unwrap();
        if dir == 'L' {
            number *= -1;
        }
        actions.push(number);
    }
    return actions;
}

fn part_one(actions: &Vec<i32>) -> i32 {
    let mut point: i32 = 50;
    let mut num_zero = 0;
    for number in actions {
        point += number;
        point = point.rem_euclid(100);
        if point == 0 {
            num_zero += 1;
        }
    }

    num_zero
}

fn part_two(actions: &Vec<i32>) -> i32 {
    let mut point: i32 = 50;
    let mut num_zero = 0;

    for &rot in actions {
        let dir = if rot < 0 { -1 } else { 1 };
        let mut steps = rot.abs();

        if steps >= 100 {
            let rounds = steps / 100;
            num_zero += rounds;
            steps -= 100 * rounds;
        }

        if dir > 0 && point != 0 && steps >= (100 - point) {
            num_zero += 1;
        }

        if dir < 0 && point != 0 && steps >= point {
            num_zero += 1;
        }

        point = (point + dir * steps).rem_euclid(100);
    }

    num_zero
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
