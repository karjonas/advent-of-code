use std::collections::{HashMap, HashSet};

extern crate common;

const NORTH: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const EAST: usize = 3;

fn parse(input: &String) -> HashSet<(i32, i32)> {
    let mut occupied = HashSet::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                occupied.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }

    return occupied;
}

fn part_both(input: &String, part_one: bool) -> usize {
    let mut map = parse(input);
    let mut offsets: HashMap<(i32, i32), usize> = HashMap::new();
    let num_rounds = if part_one { 11 } else { std::usize::MAX };

    for round in 1..num_rounds {
        let mut map_next = HashSet::new();
        let mut offsets_next = HashMap::new();
        let mut proposed: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut proposed_count: HashMap<(i32, i32), usize> = HashMap::new();

        // first half round
        for pos in &map {
            let x = pos.0;
            let y = pos.1;
            let offset = *offsets.entry(*pos).or_insert(0);

            let ne = map.contains(&(x + 1, y - 1));
            let nw = map.contains(&(x - 1, y - 1));
            let n = map.contains(&(x, y - 1));
            let e = map.contains(&(x + 1, y));
            let se = map.contains(&(x + 1, y + 1));
            let s = map.contains(&(x, y + 1));
            let sw = map.contains(&(x - 1, y + 1));
            let w = map.contains(&(x - 1, y));

            // no move
            if !ne && !nw && !n && !e && !s && !se && !sw && !w {
                proposed.insert(*pos, *pos);
                *proposed_count.entry(*pos).or_insert(0) += 1;
                continue;
            }

            let mut pos_next = *pos;
            for i in 0..4 {
                match (i + offset) % 4 {
                    NORTH => {
                        if !(n || ne || nw) {
                            pos_next = (x, y - 1);
                            break;
                        }
                    }
                    SOUTH => {
                        if !(s || se || sw) {
                            pos_next = (x, y + 1);
                            break;
                        }
                    }
                    WEST => {
                        if !(w || nw || sw) {
                            pos_next = (x - 1, y);
                            break;
                        }
                    }
                    EAST => {
                        if !(e || ne || se) {
                            pos_next = (x + 1, y);
                            break;
                        }
                    }
                    _ => {}
                }
            }

            *proposed_count.entry(pos_next).or_insert(0) += 1;
            proposed.insert(*pos, pos_next);
        }

        // second half round
        for pos in &map {
            let mut dest = *proposed.get(pos).unwrap();
            let success = *proposed_count.get(&dest).unwrap() == 1;

            if !success {
                dest = *pos;
            }

            map_next.insert(dest);
            offsets_next.insert(dest, *offsets.get(pos).unwrap() + 1);
        }

        if !part_one && map == map_next {
            return round;
        }

        map = map_next;
        offsets = offsets_next;
    }

    // calc width/height
    let mut min = (std::i32::MAX, std::i32::MAX);
    let mut max = (std::i32::MIN, std::i32::MIN);

    for (x, y) in &map {
        min = (std::cmp::min(*x, min.0), std::cmp::min(*y, min.1));
        max = (std::cmp::max(*x, max.0), std::cmp::max(*y, max.1));
    }

    let count = (1 + max.0 - min.0) * (1 + max.1 - min.1) - map.len() as i32;
    count as usize
}

fn part_one(input: &String) -> usize {
    return part_both(input, true);
}

fn part_two(input: &String) -> usize {
    return part_both(input, false);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = ".....
..##.
..#..
.....
..##.
....."
            .to_string();
        assert_eq!(part_one(&input), 25);
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            .to_string();
        assert_eq!(part_one(&input), 110);
    }
}
