extern crate common;
#[macro_use]
extern crate scan_fmt;

fn parse(input: &String) -> Vec<(i64, i64, i64, i64, i64)> {
    let mut beacons = Vec::new();
    for line in input.lines() {
        let (x0, y0, x1, y1) = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();

        let dist = (x0 - x1).abs() + (y0 - y1).abs();
        beacons.push((x0, y0, x1, y1, dist));
    }

    return beacons;
}

fn part_one(input: &String, y_test: i64) -> usize {
    let beacons = parse(input);

    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    for &(x0, _, _, _, dist) in &beacons {
        min_x = std::cmp::min(min_x, x0 - dist);
        max_x = std::cmp::max(max_x, x0 + dist);
    }

    let mut sum = 0;
    for x_test in (min_x - 1)..(max_x + 1) {
        let mut in_range = false;
        for (x0, y0, x1, y1, dist) in &beacons {
            if (*x0, *y0) == (x_test, y_test) || (*x1, *y1) == (x_test, y_test) {
                in_range = false;
                break;
            }

            let dist_pos = (x_test - x0).abs() + (y0 - y_test).abs();
            if dist_pos <= *dist {
                in_range = true;
            }
        }

        if in_range {
            sum += 1;
        }
    }

    return sum;
}

fn is_ok(
    pos: &(i64, i64),
    beacon: &(i64, i64, i64, i64, i64),
    beacons: &Vec<(i64, i64, i64, i64, i64)>,
) -> bool {
    for beacon_other in beacons {
        if beacon == beacon_other {
            continue;
        }

        let dist = (beacon_other.0 - pos.0).abs() + (beacon_other.1 - pos.1).abs();

        if dist <= beacon_other.4 {
            return false;
        }
    }

    return true;
}

fn part_two(input: &String) -> usize {
    let beacons = parse(input);

    for beacon in &beacons {
        let pos = (beacon.0, beacon.1);
        let radius = beacon.4;

        for i in 0..radius + 1 {
            let x0 = pos.0 - radius - 1 + i; // 0..mid
            let x1 = pos.0 + i; // mid..end
            let y0 = pos.1 - i; // mid..up
            let y1 = pos.1 + i; // mid..down
            let y2 = pos.1 - radius - 1 + i; // up..down
            let y3 = pos.1 + radius - i; // down..up

            for pos in [(x0, y0), (x0, y1), (x1, y2), (x1, y3)] {
                if pos.0 > 0
                    && pos.1 > 0
                    && pos.0 < 4000000
                    && pos.1 < 4000000
                    && is_ok(&pos, beacon, &beacons)
                {
                    return (pos.0 * 4000000 + pos.1) as usize;
                }
            }
        }
    }

    return 0;
}

pub fn solve() {
    let input = common::read_file("2022/day15/input");
    println!("Part one: {}", part_one(&input, 2000000));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            .to_string();
        assert_eq!(part_one(&input, 10), 26);
    }
}
