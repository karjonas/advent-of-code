extern crate common;
extern crate regex;

use regex::Regex;

const INPUT_FILE: &str = "2018/day10/input";

#[derive(Clone, Debug)]
struct Dot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn step_board(dots: &mut Vec<Dot>) {
    for dot in dots {
        let next = (
            dot.position.0 + dot.velocity.0,
            dot.position.1 + dot.velocity.1,
        );
        dot.position = next;
    }
}

fn print_board(dots: &Vec<Dot>, min_bounding_box: (usize, usize)) -> bool {
    let mut min = (std::i64::MAX, std::i64::MAX);
    let mut max = (std::i64::MIN, std::i64::MIN);

    for dot in dots {
        min = (
            std::cmp::min(min.0, dot.position.0),
            std::cmp::min(min.1, dot.position.1),
        );

        max = (
            std::cmp::max(max.0, dot.position.0),
            std::cmp::max(max.1, dot.position.1),
        );
    }

    if (max.0 - min.0) as usize > min_bounding_box.0
        || (max.1 - min.1) as usize > min_bounding_box.1
    {
        return false;
    }

    let default_row = common::filled_vector((1 + max.0 - min.0) as usize, '.');
    let mut board = common::filled_vector((1 + max.1 - min.1) as usize, default_row);

    for dot in dots {
        let x = (dot.position.0 - min.0) as usize;
        let y = (dot.position.1 - min.1) as usize;
        board[y][x] = '#';
    }

    for line in board {
        let s = line.iter().collect::<String>();
        println!("{}", s);
    }

    return true;
}

pub fn solve() {
    let input = common::read_file(INPUT_FILE);
    let re = Regex::new(r"position=<(.*), (.*)> velocity=<(.*), (.*)>").unwrap();

    let mut dots = Vec::new();

    for l in input.lines() {
        let cap = re.captures(l).unwrap();
        let to_i64 = |c: &str| return c.to_string().trim().parse::<i64>().unwrap();
        dots.push(Dot {
            position: (to_i64(&cap[1]), to_i64(&cap[2])),
            velocity: (to_i64(&cap[3]), to_i64(&cap[4])),
        });
    }

    let mut time = 0;

    println!("Part one:");
    while !print_board(&dots, (70, 15)) {
        step_board(&mut dots);
        time += 1;
    }

    println!("Part two: {:?}", time);
}
