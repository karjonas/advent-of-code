extern crate common;
extern crate regex;

use regex::Regex;
use std::cmp;

#[derive(Debug)]
struct Square {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

pub fn solve() {
    let input = common::read_file("2018/day03/input");

    let mut max_width = 0;
    let mut max_height = 0;
    let mut squares: Vec<Square> = Vec::new();

    // Match: #1316 @ 493,779: 12x18
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for line in input.lines() {
        let cap = re.captures(line.trim()).unwrap();
        let id = cap[1].parse::<usize>().unwrap();
        let (x, y) = (
            cap[2].parse::<usize>().unwrap(),
            cap[3].parse::<usize>().unwrap(),
        );
        let (w, h) = (
            cap[4].parse::<usize>().unwrap(),
            cap[5].parse::<usize>().unwrap(),
        );

        squares.push(Square { id, x, y, w, h });

        max_width = cmp::max(max_width, x + w + 1);
        max_height = cmp::max(max_height, y + h + 1);

        // println!("#{} @ {},{}: {}x{}", id, x, y, w, h);
    }

    let mut grid = common::zeroed_vector(max_width * max_height);

    for sq in &squares {
        for x in sq.x..(sq.x + sq.w) {
            for y in sq.y..(sq.y + sq.h) {
                let grid_pos = max_width * y + x;
                grid[grid_pos] += 1;
            }
        }
    }

    let part_one = grid
        .iter()
        .fold(0, |sum, &pos| sum + if pos > 1 { 1 } else { 0 });

    let mut part_two = 0;
    for sq in &squares {
        let mut found = true;
        for x in sq.x..(sq.x + sq.w) {
            for y in sq.y..(sq.y + sq.h) {
                let grid_pos = max_width * y + x;
                if grid[grid_pos] > 1 {
                    found = false;
                    break;
                }
            }
            if !found {
                break;
            }
        }

        if found {
            part_two = sq.id;
            break;
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
