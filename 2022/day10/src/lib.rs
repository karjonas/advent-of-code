use std::iter::FromIterator;

extern crate common;

fn part_one(input: &String) -> i64 {
    let mut ctr = 20;
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;

    for line in input.lines() {
        cycle += 1;
        ctr += 1;

        if ctr == 40 {
            sum += x * cycle;
            ctr = 0;
        }

        let words: Vec<_> = line.split_whitespace().collect();
        if words[0] == "noop" {
            continue;
        } else if words[0] == "addx" {
            cycle += 1;
            ctr += 1;

            if ctr == 40 {
                sum += x * cycle;
                ctr = 0;
            }
            let v: i64 = words[1].parse().unwrap();
            x += v;
        } else {
            assert!(false);
        }
    }

    return sum;
}

fn part_two(input: &String) -> String {
    let mut crt = vec![vec!['.'; 40]; 6];
    let mut cycle = 0;
    let mut x: i64 = 1;
    let mut row = 0;
    let mut col = 0;

    for line in input.lines() {
        if cycle % 40 == 0 && cycle != 0 {
            row += 1;
            col = 0;
        }
        cycle += 1;
        let c = if (x - col).abs() < 2 { '#' } else { '.' };
        crt[row][col as usize] = c;
        col += 1;

        let words: Vec<_> = line.split_whitespace().collect();
        if words[0] == "noop" {
            continue;
        } else if words[0] == "addx" {
            if cycle % 40 == 0 {
                row += 1;
                col = 0;
            }
            cycle += 1;
            let c = if (x - col).abs() < 2 { '#' } else { '.' };
            crt[row][col as usize] = c;
            col += 1;
            let v: i64 = words[1].parse().unwrap();
            x += v;
        } else {
            assert!(false);
        }
    }

    return crt.iter().fold(String::new(), |acc, v| {
        acc + String::from_iter(v.iter()).as_str() + "\n"
    });
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", part_one(&input));
    println!("Part two:\n{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            .to_string();
        assert_eq!(part_one(&input), 13140);
        let part_two_solution = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(part_two(&input), part_two_solution);
    }
}
