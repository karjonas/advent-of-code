extern crate common;

use std::collections::HashSet;

type Instruction = Vec<String>;
type Instructions = Vec<Instruction>;

const DIAGONALS: [&str; 4] = ["sw", "se", "nw", "ne"];
const ADJACENTS: [(i32, i32); 6] = [(1, -1), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1)];

fn parse_input(input: &String) -> Instructions {
    let mut output = Instructions::new();
    for line in input.lines() {
        let comma_separated = line
            .replace("se", ",se,")
            .replace("sw", ",sw,")
            .replace("nw", ",nw,")
            .replace("ne", ",ne,");

        let mut commands = Vec::new();
        for value in comma_separated.split(",") {
            if value.is_empty() {
                continue;
            }
            // Diagonals
            if DIAGONALS.contains(&value) {
                commands.push(String::from(value));
                continue;
            }
            // Sides
            for c in value.chars() {
                commands.push(String::from(c));
            }
        }
        output.push(commands);
    }

    return output;
}

fn init(input: &Instructions) -> HashSet<(i32, i32)> {
    let mut flipped = HashSet::new();
    for instruction in input {
        let mut pos = (0, 0);
        for direction in instruction {
            pos = match direction.as_str() {
                "ne" => (pos.0 + 1, pos.1 - 1),
                "e" => (pos.0 + 1, pos.1 + 0),
                "se" => (pos.0 + 0, pos.1 + 1),
                "sw" => (pos.0 - 1, pos.1 + 1),
                "w" => (pos.0 - 1, pos.1 + 0),
                "nw" => (pos.0 + 0, pos.1 - 1),
                _ => panic!("No match {}", direction),
            };
        }
        if flipped.contains(&pos) {
            flipped.remove(&pos);
        } else {
            flipped.insert(pos);
        }
    }
    return flipped;
}

fn part_one(input: &Instructions) -> usize {
    return init(&input).len();
}
fn part_two(input: &Instructions) -> usize {
    let mut flipped = init(&input);
    for _ in 0..100 {
        let mut flipped_next = HashSet::new();
        let mut positions = HashSet::new();
        for (x, y) in &flipped {
            for (d_x, d_y) in ADJACENTS.iter() {
                positions.insert((x + d_x, y + d_y));
            }
        }

        for (x, y) in &positions {
            let is_black = flipped.contains(&(*x, *y));
            let num_neighs = ADJACENTS
                .iter()
                .map(|(d_x, d_y)| flipped.contains(&(x + *d_x, y + *d_y)) as usize)
                .sum::<usize>();
            if is_black && (num_neighs == 0 || num_neighs > 2) {
                flipped_next.remove(&(*x, *y));
            } else if (!is_black && (num_neighs == 2)) || is_black {
                flipped_next.insert((*x, *y));
            }
        }

        flipped = flipped_next;
    }

    return flipped.len();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&parse_input(&input)));
    println!("Part two: {}", part_two(&parse_input(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .join("\n");

        assert_eq!(part_one(&parse_input(&input)), 10);
        assert_eq!(part_two(&parse_input(&input)), 2208);
    }
}
