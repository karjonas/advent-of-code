extern crate common;

use std::collections::HashSet;

fn parse_input(input: &String) -> HashSet<usize> {
    return input
        .lines()
        .map(|v| {
            usize::from_str_radix(
                &v.to_string()
                    .replace("B", "1")
                    .replace("F", "0")
                    .replace("R", "1")
                    .replace("L", "0"),
                2,
            )
            .unwrap()
        })
        .collect();
}

fn part_one(seats: &HashSet<usize>) -> usize {
    return *seats.iter().max().unwrap();
}

fn part_two(seats: &HashSet<usize>) -> usize {
    // Skip some seats at beginning
    for i in 48..(128 * 8) {
        if !seats.contains(&i) {
            return i;
        }
    }

    panic!("No solution found");
}

pub fn solve() {
    let input = common::read_file("2020/day05/input");
    let seats = parse_input(&input);
    println!("Part one: {}", part_one(&seats));
    println!("Part two: {}", part_two(&seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input_a = String::from("BFFFBBFRRR"); // : row 70, column 7, seat ID 567.
        let input_b = String::from("FFFBBBFRRR"); // : row 14, column 7, seat ID 119.
        let input_c = String::from("BBFFBBFRLL"); // : row 102, column 4, seat ID 820.
        assert_eq!(part_one(&parse_input(&input_a)), 567);
        assert_eq!(part_one(&parse_input(&input_b)), 119);
        assert_eq!(part_one(&parse_input(&input_c)), 820);
    }
}
