extern crate common;

type Input = (usize, usize);

fn parse_input(input: &String) -> Input {
    let numbers: Vec<_> = input.lines().map(|v| v.parse::<usize>().unwrap()).collect();
    return (numbers[0], numbers[1]);
}

fn encrypt_subject(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    return value;
}

fn find_loop_size(key: usize) -> usize {
    let mut value = 1;
    for loop_size in 1.. {
        value = (value * 7) % 20201227;

        if value == key {
            return loop_size;
        }
    }
    panic!("No loop size found");
}

fn part_one(input: &Input) -> usize {
    return encrypt_subject(input.0, find_loop_size(input.1));
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&parse_input(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
        assert_eq!(encrypt_subject(17807724, 8), 14897079);
        assert_eq!(encrypt_subject(5764801, 11), 14897079);
    }
}
