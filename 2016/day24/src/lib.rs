use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    blocks: Vec<Vec<char>>, // [row][col]
    digits: Vec<(usize, usize)>,
    dists: Vec<Vec<Vec<usize>>>,
    dist_scalars: Vec<Vec<usize>>,
}

fn parse_map(contents: String) -> Map {
    let mut map = Map {
        width: 0,
        height: 0,
        blocks: Vec::new(),
        digits: Vec::new(),
        dists: Vec::new(),
        dist_scalars: Vec::new(),
    };

    let mut line_ctr = 0;
    for line in contents.lines() {
        let line_width = line.len();
        map.width = line_width;
        let mut col_ctr = 0;
        for c in line.chars() {
            match c.to_string().parse::<usize>() {
                Ok(d) => {
                    let new_len = std::cmp::max(map.digits.len(), d + 1);
                    map.digits.resize(new_len, (0, 0));
                    map.digits[d] = (line_ctr, col_ctr);
                }
                Err(_) => (),
            }
            col_ctr += 1;
        }
        let chars: Vec<char> = line.chars().collect();
        map.blocks.push(chars);
        line_ctr += 1;
    }

    map.height = line_ctr;
    calc_distances(&mut map);

    return map;
}

fn find_shortest_path(
    dists: &Vec<Vec<usize>>,
    curr: usize,
    visited: Vec<bool>,
    return_home: bool,
) -> usize {
    let mut shortest = std::usize::MAX;
    let mut done = true;
    for i in 0..dists.len() {
        if visited[i] || i == curr {
            continue;
        }
        let mut visited_i = visited.clone();
        visited_i[curr] = true;
        visited_i[i] = true;
        let d = dists[curr][i] + find_shortest_path(dists, i, visited_i, return_home);

        done = false;
        shortest = std::cmp::min(shortest, d);
    }

    if done {
        shortest = if return_home { dists[curr][0] } else { 0 };
    }

    return shortest;
}

fn calc_distances(map: &mut Map) {
    let mut digit = 0;
    while digit < map.digits.len() {
        let mut dists: Vec<Vec<usize>> = Vec::new();
        for _ in 0..map.height {
            let mut row: Vec<usize> = Vec::new();
            row.resize(map.width, std::usize::MAX);
            dists.push(row);
        }

        let mut stack: Vec<(usize, (usize, usize))> = Vec::new();
        let digit_pos = map.digits[digit];
        stack.push((0, digit_pos));

        while !stack.is_empty() {
            let (steps, (y, x)) = stack.pop().unwrap();

            if map.blocks[y][x] == '#' || dists[y][x] < std::usize::MAX {
                continue;
            }

            dists[y][x] = steps;

            if (x + 1) < map.width {
                stack.push((steps + 1, (y, x + 1)));
            }
            if x > 0 {
                stack.push((steps + 1, (y, x - 1)));
            }

            if (y + 1) < map.height {
                stack.push((steps + 1, (y + 1, x)));
            }
            if y > 0 {
                stack.push((steps + 1, (y - 1, x)));
            }

            stack.sort_by(|a, b| b.0.cmp(&a.0));
        }
        digit += 1;
        map.dists.push(dists);
    }

    map.dist_scalars.resize(map.digits.len(), Vec::new());

    let mut digit = 0;
    while digit < map.digits.len() {
        map.dist_scalars[digit].resize(map.digits.len(), std::usize::MAX);

        let mut digit1 = 0;
        while digit1 < map.digits.len() {
            let pos1 = map.digits[digit1];
            let dist = map.dists[digit][pos1.0][pos1.1];
            map.dist_scalars[digit][digit1] = dist;
            digit1 += 1;
        }
        digit += 1;
    }
}

pub fn solve() {
    let mut file = File::open("2016/day24/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let map = parse_map(contents);
    {
        let mut visited: Vec<bool> = Vec::new();
        visited.resize(map.dist_scalars.len(), false);
        let shortest = find_shortest_path(&map.dist_scalars, 0, visited.clone(), false);
        println!("Part 1: {:?}", shortest);
    }
    {
        let mut visited: Vec<bool> = Vec::new();
        visited.resize(map.dist_scalars.len(), false);
        let shortest = find_shortest_path(&map.dist_scalars, 0, visited.clone(), true);
        println!("Part 2: {:?}", shortest);
    }
}
