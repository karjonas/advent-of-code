extern crate common;

struct PuzzleInput {
    departure: usize,
    times: Vec<usize>,
}

fn parse_input(input: &String) -> PuzzleInput {
    let lines: Vec<_> = input.lines().collect();
    let departure = lines[0].parse::<usize>().unwrap();
    let times = lines[1]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap_or(0))
        .collect();
    return PuzzleInput {
        departure: departure,
        times: times,
    };
}

fn part_one(input: &PuzzleInput) -> usize {
    let mut result = std::usize::MAX;
    let mut best = std::usize::MAX;
    for time in &input.times {
        if *time == 0 {
            continue;
        }
        // NOTE: will not work if wait is zero
        let wait = time - input.departure % time;
        if wait < best {
            result = wait * time;
            best = wait;
        }
    }

    return result;
}

fn part_two(input: &PuzzleInput) -> usize {
    let mut modulii = Vec::new();
    let mut residues = Vec::new();
    for i in 0..input.times.len() {
        let time = input.times[i];
        if time == 0 {
            continue;
        }
        modulii.push(time as i128);
        residues.push((time - (i % time)) as i128);
    }

    return common::chinese_remainder(&residues.as_slice(), &modulii.as_slice()).unwrap() as usize;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let times = parse_input(&input);

    println!("Part one: {}", part_one(&times));
    println!("Part two: {}", part_two(&times));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = ["939", "7,13,x,x,59,x,31,19"].join("\n");

        assert_eq!(part_one(&parse_input(&input)), 295);
        assert_eq!(part_two(&parse_input(&input)), 1068781);
    }

    #[test]
    fn test_samples_p2() {
        assert_eq!(part_two(&parse_input(&String::from("0\n17,x,13,19"))), 3417);
        assert_eq!(
            part_two(&parse_input(&String::from("0\n67,7,59,61"))),
            754018
        );
        assert_eq!(
            part_two(&parse_input(&String::from("0\n67,x,7,59,61"))),
            779210
        );
        assert_eq!(
            part_two(&parse_input(&String::from("0\n67,7,x,59,61"))),
            1261476
        );
        assert_eq!(
            part_two(&parse_input(&String::from("0\n1789,37,47,1889"))),
            1202161486
        );
    }
}
