extern crate common;
#[macro_use]
extern crate scan_fmt;

type Instruction = (char, i64);

fn parse_input(input: &String) -> Vec<Instruction> {
    return input
        .lines()
        .map(|line| scan_fmt!(line, "{[A-Z]}{d}", char, i64).unwrap())
        .collect();
}

fn part_one(instructions: &Vec<Instruction>) -> usize {
    const NUM_DIRS: usize = 4;
    let mut facing = 1; // 0,1,2,3 N, E, S, W
    let mut y = 0;
    let mut x = 0;

    for (action, value) in instructions.clone() {
        match action {
            'N' => y += value,
            'E' => x += value,
            'S' => y -= value,
            'W' => x -= value,
            'L' => facing = (facing + NUM_DIRS - value as usize / 90) % NUM_DIRS,
            'R' => facing = (facing + NUM_DIRS + value as usize / 90) % NUM_DIRS,
            'F' => match facing {
                0 => y += value,
                1 => x += value,
                2 => y -= value,
                3 => x -= value,
                _ => panic!("invalid facing"),
            },
            _ => panic!("Invalid instruction"),
        }
    }

    return (y.abs() + x.abs()) as usize;
}

fn part_two(instructions: &Vec<Instruction>) -> usize {
    let mut pos = (0, 0);
    let mut way_pos = (10, 1);

    for (action, value) in instructions.clone() {
        match action {
            'N' => way_pos.1 += value,
            'E' => way_pos.0 += value,
            'S' => way_pos.1 -= value,
            'W' => way_pos.0 -= value,
            'L' => {
                for _i in 0..value / 90 {
                    way_pos = (-way_pos.1, way_pos.0);
                }
            }
            'R' => {
                for _i in 0..value / 90 {
                    way_pos = (way_pos.1, -way_pos.0);
                }
            }
            'F' => pos = (pos.0 + way_pos.0 * value, pos.1 + way_pos.1 * value),
            _ => panic!("Invalid instruction"),
        }
    }

    return (pos.0.abs() + pos.1.abs()) as usize;
}

pub fn solve() {
    let input = common::read_file("2020/day12/input");
    let grid = parse_input(&input);
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = ["F10", "N3", "F7", "R90", "F11"].join("\n");
        assert_eq!(part_one(&parse_input(&input)), 25);
        assert_eq!(part_two(&parse_input(&input)), 286);
    }
}
