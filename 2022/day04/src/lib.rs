extern crate common;
#[macro_use]
extern crate scan_fmt;

fn solve_internal(input: &String, part_one: bool) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let (a0, a1, b0, b1) = scan_fmt!(line, "{}-{},{}-{}", usize, usize, usize, usize).unwrap();
        let part_one_ok = (a0 <= b0 && a1 >= b1) || (b0 <= a0 && b1 >= a1);
        let part_two_ok = a0 <= b1 && b0 <= a1;
        if part_one && part_one_ok || !part_one && part_two_ok {
            sum += 1;
        }
    }
    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", solve_internal(&input, true));
    println!("Part two: {}", solve_internal(&input, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input_simple = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(solve_internal(&input_simple, true), 2);
        assert_eq!(solve_internal(&input_simple, false), 4);
    }
}
