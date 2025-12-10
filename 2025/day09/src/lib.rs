extern crate geo;
use geo::{ContainsProperly, LineString, Polygon};

fn parse_input(input: &String) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    for line in input.lines() {
        let mut splits = line.split(',');
        let f0 = splits.next().unwrap().parse::<usize>().unwrap();
        let f1 = splits.next().unwrap().parse::<usize>().unwrap();
        points.push((f0, f1));
    }
    return points;
}

fn part_one(points: &Vec<(usize, usize)>) -> usize {
    let n = points.len();
    let mut biggest = 0;
    for i in 0..n {
        let (x0, y0) = points[i];
        for j in i + 1..n {
            let (x1, y1) = points[j];
            let area = (1 + x0.abs_diff(x1)) * (1 + y0.abs_diff(y1));
            biggest = std::cmp::max(biggest, area);
        }
    }

    biggest
}

fn part_two(points: &Vec<(usize, usize)>) -> usize {
    let n = points.len();
    let mut biggest = 0;

    let mut pointsf = Vec::new();
    for (x, y) in points {
        pointsf.push((*x as f64, *y as f64));
    }

    let polygon = Polygon::new(LineString::from(pointsf), vec![]);

    for i in 0..n {
        let (x0, y0) = points[i];
        for j in i + 1..n {
            let (x1, y1) = points[j];
            let area = (1 + x0.abs_diff(x1)) * (1 + y0.abs_diff(y1));

            // Create the four line segments of the rectangle
            let x_min = std::cmp::min(x0, x1) as f64 + 0.25;
            let x_max = std::cmp::max(x0, x1) as f64 - 0.25;
            let y_min = std::cmp::min(y0, y1) as f64 + 0.25;
            let y_max = std::cmp::max(y0, y1) as f64 - 0.25;

            let area_polygon = Polygon::new(
                LineString::from(vec![
                    (x_min, y_min),
                    (x_max, y_min),
                    (x_max, y_max),
                    (x_min, y_max),
                ]),
                vec![],
            );

            if polygon.contains_properly(&area_polygon) && area > biggest {
                biggest = area;
            }
        }
    }

    biggest
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let points = parse_input(&input);
    println!("Part one: {}", part_one(&points));
    println!("Part two: {}", part_two(&points));
}
