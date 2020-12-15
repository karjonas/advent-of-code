extern crate common;

fn solve_both(input: &Vec<usize>, num_iterations: usize) -> usize {
    const UNSET: usize = std::usize::MAX;
    let mut last_position = Vec::new();
    let mut last_last_position = Vec::new();

    last_position.resize(num_iterations, UNSET);
    last_last_position.resize(num_iterations, UNSET);

    for idx in 0..input.len() {
        let curr_number = input[idx];
        last_last_position[curr_number] = last_position[curr_number];
        last_position[curr_number] = idx;
    }

    let mut last_spoken = *input.last().unwrap();
    let mut ctr = input.len();
    while ctr < num_iterations {
        let last_last_idx = last_last_position[last_spoken];

        if last_last_idx == std::usize::MAX {
            last_spoken = 0;
        } else {
            let last_idx = last_position[last_spoken];
            last_spoken = last_idx - last_last_idx;
        }

        last_last_position[last_spoken] = last_position[last_spoken];
        last_position[last_spoken] = ctr;
        ctr += 1;
    }
    return last_spoken;
}

pub fn solve() {
    let input = [1, 0, 16, 5, 17, 4].to_vec();
    println!("Part one: {}", solve_both(&input, 2020));
    println!("Part two: {}", solve_both(&input, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [0, 3, 6].to_vec();
        assert_eq!(solve_both(&input, 2020), 436);
    }
}
