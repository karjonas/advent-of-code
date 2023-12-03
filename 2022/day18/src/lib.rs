use std::collections::HashSet;

extern crate common;

fn parse(input: &String) -> Vec<(i32, i32, i32)> {
    let mut cubes = Vec::new();
    for line in input.lines() {
        let v3: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        cubes.push((v3[0], v3[1], v3[2]));
    }
    return cubes;
}

fn part_one(input: &String) -> usize {
    let cubes = parse(input);
    let mut sum = 0;
    for i in 0..cubes.len() {
        let mut num_neighs = 0;
        let cube_i = cubes[i];
        for j in 0..cubes.len() {
            let cube_j = cubes[j];
            if j == i {
                continue;
            }
            let dist = (cube_i.0 - cube_j.0).abs()
                + (cube_i.1 - cube_j.1).abs()
                + (cube_i.2 - cube_j.2).abs();

            assert_ne!(dist, 0);
            num_neighs += (dist == 1) as usize;
        }
        assert!(num_neighs <= 6);
        sum += 6 - num_neighs;
    }

    return sum;
}

fn part_two(input: &String) -> usize {
    let cubes = parse(input);
    let mut bounds = vec![(1, 1, 1), (-1, -1, -1)];

    for (x, y, z) in &cubes {
        bounds[0].0 = std::cmp::min(bounds[0].0, *x - 1);
        bounds[0].1 = std::cmp::min(bounds[0].1, *y - 1);
        bounds[0].2 = std::cmp::min(bounds[0].2, *z - 1);
        bounds[1].0 = std::cmp::max(bounds[1].0, *x + 1);
        bounds[1].1 = std::cmp::max(bounds[1].1, *y + 1);
        bounds[1].2 = std::cmp::max(bounds[1].2, *z + 1);
    }

    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();
    to_visit.push(bounds[0]);
    let mut sum = 0;

    // flood fill from corner and count all cubes that are neighbours
    // to get surface area
    while !to_visit.is_empty() {
        let cube = to_visit.pop().unwrap();
        if visited.contains(&cube) {
            continue;
        }
        visited.insert(cube);

        for nei in [
            (cube.0, cube.1, cube.2 - 1),
            (cube.0, cube.1, cube.2 + 1),
            (cube.0, cube.1 - 1, cube.2),
            (cube.0, cube.1 + 1, cube.2),
            (cube.0 - 1, cube.1, cube.2),
            (cube.0 + 1, cube.1, cube.2),
        ] {
            // skip out of bounds
            if nei.0 < bounds[0].0
                || nei.1 < bounds[0].1
                || nei.2 < bounds[0].2
                || nei.0 > bounds[1].0
                || nei.1 > bounds[1].1
                || nei.2 > bounds[1].2
            {
                continue;
            }

            // if cube, increase count, else visit next
            if cubes.contains(&nei) {
                sum += 1;
            } else {
                to_visit.push(nei);
            }
        }
    }

    sum
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
        let input0 = "1,1,1\n2,1,1".to_string();
        let input1 = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            .to_string();

        assert_eq!(part_one(&input0), 10);
        assert_eq!(part_one(&input1), 64);
        assert_eq!(part_two(&input1), 58);
    }
}
