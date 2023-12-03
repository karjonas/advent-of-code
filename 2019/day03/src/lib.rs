extern crate common;

#[derive(Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
    horizontal: bool,
}

#[derive(Debug)]
struct Trail {
    lines: Vec<Line>,
}

const POINT_INF: Point = Point {
    x: std::i64::MAX / 2,
    y: std::i64::MAX / 2,
};

fn intersection(l_a: &Line, l_b: &Line) -> Point {
    let mut p = POINT_INF;

    let a_horiz = l_a.horizontal;

    let ax0 = std::cmp::min(l_a.a.x, l_a.b.x);
    let ax1 = std::cmp::max(l_a.a.x, l_a.b.x);

    let ay0 = std::cmp::min(l_a.a.y, l_a.b.y);
    let ay1 = std::cmp::max(l_a.a.y, l_a.b.y);

    let bx0 = std::cmp::min(l_b.a.x, l_b.b.x);
    let bx1 = std::cmp::max(l_b.a.x, l_b.b.x);

    let by0 = std::cmp::min(l_b.a.y, l_b.b.y);
    let by1 = std::cmp::max(l_b.a.y, l_b.b.y);

    let b_horiz = l_b.horizontal;

    if a_horiz && !b_horiz && (ax0 < bx0 && bx0 < ax1) && (by0 < ay0 && ay0 < by1) {
        p = Point { x: bx0, y: ay0 };
    }
    if !a_horiz && b_horiz && (ay0 < by0 && by0 < ay1) && (bx0 < ax0 && ax0 < bx1) {
        p = Point { x: ax0, y: by0 };
    }

    return p;
}

fn intersect_dist(l_a: &Line, p: &Point) -> i64 {
    let mut dist = std::i64::MAX;
    let a_horiz = l_a.horizontal;

    let ax0 = std::cmp::min(l_a.a.x, l_a.b.x);
    let ax1 = std::cmp::max(l_a.a.x, l_a.b.x);

    let ay0 = std::cmp::min(l_a.a.y, l_a.b.y);
    let ay1 = std::cmp::max(l_a.a.y, l_a.b.y);

    if a_horiz && (ax0 < p.x && p.x < ax1) && ay0 == p.y {
        dist = (l_a.a.x - p.x).abs();
    }
    if !a_horiz && (ay0 < p.y && p.y < ay1) && ax0 == p.x {
        dist = (l_a.a.y - p.y).abs();
    }

    return dist;
}

fn trail_intersections(a: &Trail, b: &Trail) -> Vec<Point> {
    let mut intersections = Vec::new();
    for line_a in &a.lines {
        for line_b in &b.lines {
            let inter = intersection(line_a, line_b);
            if inter != POINT_INF {
                intersections.push(inter);
            }
        }
    }
    return intersections;
}

fn parse_input(input: &str) -> Vec<Trail> {
    let mut trails = Vec::new();
    for trail_s in input.split("\n") {
        let mut trail = Trail { lines: Vec::new() };
        let mut x: i64 = 0;
        let mut y: i64 = 0;

        for step in trail_s.split(",") {
            let dir = String::from(step).chars().next().unwrap();
            let num = common::string_to_i64(common::strip_characters(step, "LRUD").as_str());

            let mut xn = x;
            let mut yn = y;
            let mut horizontal = false;

            if dir == 'L' {
                xn -= num;
                horizontal = true;
            } else if dir == 'R' {
                xn += num;
                horizontal = true;
            } else if dir == 'U' {
                yn += num;
            } else if dir == 'D' {
                yn -= num;
            } else {
                panic!("Invalid char");
            }

            let line = Line {
                a: Point { x: x, y: y },
                b: Point { x: xn, y: yn },
                horizontal: horizontal,
            };

            x = xn;
            y = yn;

            trail.lines.push(line);
        }
        trails.push(trail);
    }

    return trails;
}

fn run_input(input: &str) -> i64 {
    let trails = parse_input(input);
    assert_eq!(trails.len(), 2);
    let mut p = POINT_INF;

    let inters = trail_intersections(&trails[0], &trails[1]);
    for inter in inters {
        if inter.x.abs() + inter.y.abs() < p.x.abs() + p.y.abs() {
            p = inter;
        }
    }

    return p.x.abs() + p.y.abs();
}

fn dist_to_point(a: &Trail, p: &Point) -> i64 {
    let mut dist = 0;
    for line in &a.lines {
        let dist_intersect = intersect_dist(&line, &p);
        if dist_intersect != std::i64::MAX {
            return dist + dist_intersect;
        }
        dist += (line.a.x - line.b.x).abs();
        dist += (line.a.y - line.b.y).abs();
    }

    return std::i64::MAX;
}

fn run_input_pt2(input: &str) -> i64 {
    let trails = parse_input(input);
    assert_eq!(trails.len(), 2);
    let mut inters: Vec<Point> = Vec::new();

    inters.append(&mut trail_intersections(&trails[0], &trails[1]));

    let mut closest = std::i64::MAX;

    for p in inters {
        let d0 = dist_to_point(&trails[0], &p);
        let d1 = dist_to_point(&trails[1], &p);
        assert_ne!(d0, std::i64::MAX);
        assert_ne!(d1, std::i64::MAX);

        closest = std::cmp::min(d0 + d1, closest);
    }

    return closest;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", run_input(input.as_str()));
    println!("Part two: {}", run_input_pt2(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(
            run_input("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
        assert_eq!(
            run_input(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(run_input_pt2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
        assert_eq!(
            run_input_pt2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
        assert_eq!(
            run_input_pt2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
