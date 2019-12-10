extern crate common;

use std::collections::HashMap;

type Point = (i32, i32);

fn dot(a: Point, b: Point) -> i32 {
    return a.0 * b.0 + a.1 * b.1;
}

fn length(a: Point) -> f32 {
    return ((a.0 * a.0 + a.1 * a.1) as f32).sqrt();
}

fn angle(a: Point, b: Point) -> f32 {
    return (dot(a, b) as f32 / (length(a) * length(b))).acos();
}

fn distance(a: Point, b: Point) -> f32 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    return ((x * x + y * y) as f32).sqrt();
}

fn is_occluding(src: Point, dest: Point, point: Point) -> bool {
    let dist_a = distance(src, point) + distance(point, dest);
    let dist_b = distance(src, dest);
    const EPSILON: f32 = 0.00001;
    return (dist_a - dist_b).abs() < EPSILON;
}

fn num_in_sight(asteroids: &Vec<Point>, asteroid_a: Point) -> usize {
    let mut ctr = 0;
    for asteroid_b in asteroids {
        // skip self
        if asteroid_a == asteroid_b.clone() {
            continue;
        }

        let mut any_occluding = false;
        for asteroid_c in asteroids {
            if asteroid_c.clone() == asteroid_b.clone() || asteroid_a == asteroid_c.clone() {
                continue;
            }
            if is_occluding(asteroid_a, asteroid_b.clone(), asteroid_c.clone()) {
                any_occluding = true;
                break;
            }
        }
        ctr += if any_occluding { 0 } else { 1 };
    }
    return ctr;
}

fn parse_input(input: &str) -> Vec<Point> {
    let data = input
        .split('\n')
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut asteroids = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }

    return asteroids;
}

fn solve_input(asteroids: &Vec<Point>) -> (i32, i32, usize) {
    let mut best_asteroid = (0, 0);
    let mut best_value = 0;
    for asteroid in asteroids.clone() {
        let result = num_in_sight(asteroids, asteroid);
        if result > best_value {
            best_asteroid = asteroid.clone();
            best_value = result;
        }
    }
    return (best_asteroid.0, best_asteroid.1, best_value);
}

fn solve_part_two(asteroids: &Vec<Point>, pos: Point) -> Vec<Point> {
    let mut occluded_by: HashMap<Point, Vec<Point>> = HashMap::new();
    let mut angles: Vec<(f32, Point)> = Vec::new();
    let num_asteroids = asteroids.len();
    for i in 0..num_asteroids {
        let asteroid = asteroids[i];
        occluded_by.entry(asteroid).or_insert(Vec::new());
        if asteroid == pos {
            continue;
        }
        for j in 0..num_asteroids {
            let asteroid_other = asteroids[j];
            if asteroid_other == pos || asteroid_other == asteroid {
                continue;
            }

            if is_occluding(pos, asteroid, asteroid_other) {
                occluded_by
                    .entry(asteroid)
                    .or_insert(Vec::new())
                    .push(asteroid_other);
            }
        }

        let dir = (asteroid.0 - pos.0, asteroid.1 - pos.1);
        let angle = angle((0, -1), dir);
        let angle_adjusted = if asteroid.0 < pos.0 {
            (2.0 * std::f32::consts::PI) - angle
        } else {
            angle
        };
        angles.push((angle_adjusted, asteroid));
    }

    angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut order: Vec<Point> = Vec::new();
    loop {
        if angles.is_empty() {
            break;
        };

        for (_angle, pos) in &angles {
            if occluded_by.get(pos).unwrap().is_empty() {
                order.push(pos.clone());
            }
        }

        for (_p, list) in &mut occluded_by {
            list.retain(|&x| !order.contains(&x));
        }

        angles.retain(|&x| !order.contains(&x.1));
    }

    return order;
}

pub fn solve() {
    let input = parse_input(common::read_file("2019/day10/input").as_str());
    let (x, y, v) = solve_input(&input);
    let p = solve_part_two(&input, (x, y))[199];
    let part2 = (p.0 * 100) + p.1;

    println!("Part one: {}", v);
    println!("Part two: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one_a() {
        let input = ".#..#\n.....\n#####\n....#\n...##\n";
        assert_eq!(solve_input(&parse_input(input)), (3, 4, 8));
    }
    #[test]
    fn test_samples_part_one_b() {
        let input = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";
        assert_eq!(solve_input(&parse_input(input)), (5, 8, 33));
    }
    #[test]
    fn test_samples_part_one_c() {
        let input = "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.\n";
        assert_eq!(solve_input(&parse_input(input)), (1, 2, 35));
    }

    #[test]
    fn test_samples_part_one_d() {
        let input =     ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..\n";
        assert_eq!(solve_input(&parse_input(input)), (6, 3, 41));
    }

    #[test]
    fn test_samples_part_one_e() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n";
        assert_eq!(solve_input(&parse_input(input)), (11, 13, 210));
    }

    #[test]
    fn test_samples_part_two_a() {
        let input = ".#....#####...#..\n##...##.#####..##\n##...#...#.#####.\n..#.....X...###..\n..#.#.....#....##\n";
        assert_eq!(
            solve_part_two(&parse_input(input), (8, 3)),
            [
                (8, 1),
                (9, 0),
                (9, 1),
                (10, 0),
                (9, 2),
                (11, 1),
                (12, 1),
                (11, 2),
                (15, 1),
                (12, 2),
                (13, 2),
                (14, 2),
                (15, 2),
                (12, 3),
                (16, 4),
                (15, 4),
                (10, 4),
                (4, 4),
                (2, 4),
                (2, 3),
                (0, 2),
                (1, 2),
                (0, 1),
                (1, 1),
                (5, 2),
                (1, 0),
                (5, 1),
                (6, 1),
                (6, 0),
                (7, 0),
                (8, 0),
                (10, 1),
                (14, 0),
                (16, 1),
                (13, 3),
                (14, 3)
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_samples_part_two_b() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n";
        let solution = solve_part_two(&parse_input(input), (11, 13));
        assert_eq!(solution[0], (11, 12));
        assert_eq!(solution[1], (12, 1));
        assert_eq!(solution[2], (12, 2));
        assert_eq!(solution[9], (12, 8));
        assert_eq!(solution[19], (16, 0));
        assert_eq!(solution[49], (16, 9));
        assert_eq!(solution[99], (10, 16));
        assert_eq!(solution[198], (9, 6));
        assert_eq!(solution[199], (8, 2));
        assert_eq!(solution[200], (10, 9));
        assert_eq!(solution[298], (11, 1));
    }

    #[test]
    fn test_samples_angle() {
        assert_eq!(angle((0, 1), (1, 0)), 0.5 * std::f32::consts::PI);
        assert_eq!(angle((0, 1), (0, -1)), std::f32::consts::PI);
        assert_eq!(angle((0, 1), (-1, 0)), 0.5 * std::f32::consts::PI);
    }
}
