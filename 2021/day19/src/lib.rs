extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn parse_input(input: &String) -> Vec<Vec<Point>> {
    let mut all = Vec::new();
    let mut curr_vec = Vec::new();
    for line in input.lines() {
        if line.contains("scanner") || line.is_empty() {
            if !curr_vec.is_empty() {
                all.push(curr_vec.clone());
            }
            curr_vec.clear();
            continue;
        }

        let (x, y, z) = scan_fmt!(line, "{d},{d},{d}", i32, i32, i32).unwrap();
        curr_vec.push(Point { x: x, y: y, z: z });
    }
    all.push(curr_vec);
    return all;
}

fn get_rotation_by_index(v: Point, index: usize) -> Point {
    return match index {
        0 => Point {
            x: v.z,
            y: v.y,
            z: -v.x,
        },
        1 => Point {
            x: -v.z,
            y: -v.y,
            z: -v.x,
        },
        2 => Point {
            x: -v.z,
            y: -v.x,
            z: v.y,
        },
        3 => Point {
            x: -v.z,
            y: v.x,
            z: -v.y,
        },
        4 => Point {
            x: -v.z,
            y: v.y,
            z: v.x,
        },
        5 => Point {
            x: -v.y,
            y: -v.z,
            z: v.x,
        },
        6 => Point {
            x: -v.y,
            y: -v.x,
            z: -v.z,
        },
        7 => Point {
            x: -v.y,
            y: v.x,
            z: v.z,
        },
        8 => Point {
            x: -v.y,
            y: v.z,
            z: -v.x,
        },
        9 => Point {
            x: -v.x,
            y: -v.z,
            z: -v.y,
        },
        10 => Point {
            x: -v.x,
            y: -v.y,
            z: v.z,
        },
        11 => Point {
            x: -v.x,
            y: v.y,
            z: -v.z,
        },
        12 => Point {
            x: -v.x,
            y: v.z,
            z: v.y,
        },
        13 => Point {
            x: v.x,
            y: -v.z,
            z: v.y,
        },
        14 => Point {
            x: v.x,
            y: -v.y,
            z: -v.z,
        },
        15 => Point {
            x: v.x,
            y: v.y,
            z: v.z,
        },
        16 => Point {
            x: v.x,
            y: v.z,
            z: -v.y,
        },
        17 => Point {
            x: v.y,
            y: -v.z,
            z: -v.x,
        },
        18 => Point {
            x: v.y,
            y: -v.x,
            z: v.z,
        },
        19 => Point {
            x: v.y,
            y: v.x,
            z: -v.z,
        },
        20 => Point {
            x: v.y,
            y: v.z,
            z: v.x,
        },
        21 => Point {
            x: v.z,
            y: -v.y,
            z: v.x,
        },
        22 => Point {
            x: v.z,
            y: -v.x,
            z: -v.y,
        },
        23 => Point {
            x: v.z,
            y: v.x,
            z: v.y,
        },
        _ => Point { x: 0, y: 0, z: 0 },
    };
}

fn overlaps(a: &Vec<Point>, b: &Vec<Point>, offset: Point, orientation: usize) -> bool {
    let mut num_hits = 0;
    for point_a in a {
        for point_b in b {
            let b_adjusted = offset + get_rotation_by_index(*point_b, orientation);
            if *point_a == b_adjusted {
                num_hits += 1;
                if num_hits >= 12 {
                    return true;
                }
                break;
            }
        }
    }

    return false;
}

fn find_overlaps(a: &Vec<Point>, b: &Vec<Point>) -> Option<(Point, usize)> {
    for i in 0..a.len() {
        let point_a = a[i];
        for j in 0..b.len() {
            let point_b = b[j];
            for orientation in 0..24 {
                let offset = point_a - get_rotation_by_index(point_b, orientation);
                if overlaps(a, b, offset, orientation) {
                    return Some((offset, orientation));
                }
            }
        }
    }

    return None;
}

fn orient_points(points: &Vec<Point>, orientation: usize, offset: Point) -> Vec<Point> {
    let mut result = Vec::new();
    for p in points {
        result.push(offset + get_rotation_by_index(*p, orientation));
    }
    return result;
}

fn orient_beacons(input: &String) -> (usize, Vec<Point>) {
    let mut scanners = parse_input(input);
    let mut offsets = Vec::new();
    offsets.resize(scanners.len(), Point { x: 0, y: 0, z: 0 });
    let mut orientation = Vec::new();
    orientation.resize(scanners.len(), 0);
    let mut is_oriented = Vec::new();
    is_oriented.resize(scanners.len(), false);
    is_oriented[0] = true;

    loop {
        let mut done = true;
        for i in 0..scanners.len() - 1 {
            for j in i + 1..scanners.len() {
                if is_oriented[i] == is_oriented[j] {
                    continue;
                }
                done = false;
                let idx_a = if is_oriented[i] { i } else { j };
                let idx_b = if is_oriented[i] { j } else { i };
                let overlaps = find_overlaps(&scanners[idx_a], &scanners[idx_b]);

                if overlaps.is_some() {
                    let (offset, orientation) = overlaps.unwrap();
                    offsets[idx_b] = offset;
                    scanners[idx_b] = orient_points(&scanners[idx_b], orientation, offset);
                    is_oriented[idx_b] = true;
                }
            }
        }
        if done {
            break;
        }
    }
    let mut beacons = Vec::new();
    for scanner in scanners {
        for point in scanner {
            beacons.push(point);
        }
    }
    beacons.sort();
    beacons.dedup();

    return (beacons.len(), offsets);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let (num_beacons, positions) = orient_beacons(&input);

    let mut max = 0;
    for i in 0..positions.len() {
        let p0 = positions[i];
        for j in i + 1..positions.len() {
            let p1 = positions[j];
            let mag = (p0.x - p1.x).abs() + (p0.y - p1.y).abs() + (p0.z - p1.z).abs();
            max = std::cmp::max(max, mag);
        }
    }

    println!("Part one: {}", num_beacons);
    println!("Part two: {}", max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        assert_eq!(orient_beacons(&input.to_string()).0, 79);
    }
}
