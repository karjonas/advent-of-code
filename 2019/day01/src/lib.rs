extern crate common;

fn calc_required_mass(value: usize, recurse: bool) -> usize {
    let d = value / 3;
    let result = d - std::cmp::min(2, d);
    if recurse && result > 0 {
        return result + calc_required_mass(result, true);
    }
    return result;
}

fn solve_internal(input: String, recurse: bool) -> usize {
    return input
        .split('\n')
        .map(|s| calc_required_mass(common::string_to_usize(s), recurse))
        .sum();
}

pub fn solve(filepath: &str) {
    println!(
        "Part one: {}",
        solve_internal(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
            false
        )
    );

    println!(
        "Part two: {}",
        solve_internal(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
            true
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(calc_required_mass(12, false), 2);
        assert_eq!(calc_required_mass(14, false), 2);
        assert_eq!(calc_required_mass(1969, false), 654);
        assert_eq!(calc_required_mass(100756, false), 33583);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(calc_required_mass(14, true), 2);
        assert_eq!(calc_required_mass(1969, true), 966);
        assert_eq!(calc_required_mass(100756, true), 50346);
    }
}
