extern crate common;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Point {
    data: [i64; 4],
    constellation: usize,
}

fn dist(a: &Point, b: &Point) -> usize {
    let mut d = 0;
    for i in 0..4 {
        d += (a.data[i] - b.data[i]).abs() as usize;
    }
    return d;
}

fn parse_input(input: &String) -> Vec<Point> {
    let mut output = Vec::new();
    let re = Regex::new(r"(.*),(.*),(.*),(.*)").unwrap();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let data = [
            common::string_to_i64(&cap[1]),
            common::string_to_i64(&cap[2]),
            common::string_to_i64(&cap[3]),
            common::string_to_i64(&cap[4]),
        ];
        output.push(Point {
            data: data,
            constellation: 0,
        });
    }

    return output;
}

fn find_constellations(points: &mut Vec<Point>) -> usize {
    let num_points = points.len();
    let mut connected = common::filled_vector(num_points, Vec::new());

    for i in 0..num_points {
        points[i].constellation = i;
        for j in i + 1..num_points {
            if dist(&points[i], &points[j]) <= 3 {
                connected[i].push(j);
                connected[j].push(i);
            }
        }
    }

    let mut any_change = true;
    while any_change {
        any_change = false;

        for i in 0..num_points {
            let mut min_id = points[i].constellation;
            for j in &connected[i] {
                min_id = std::cmp::min(min_id, points[*j].constellation);
            }
            any_change = any_change || (min_id != points[i].constellation);
            points[i].constellation = min_id;
        }
    }

    let mut constellations = HashSet::new();
    for p in points.iter() {
        constellations.insert(p.constellation);
    }

    return constellations.len();
}

fn part_one(input: &String) {
    let mut points = parse_input(input);
    println!("Part one: {}", find_constellations(&mut points));
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    part_one(&input);
}
