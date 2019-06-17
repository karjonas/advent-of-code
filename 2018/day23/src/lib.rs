extern crate common;
extern crate regex;

use regex::Regex;

const INPUT_PATH: &str = "2018/day23/input";

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Copy, Clone)]
struct NanoBot {
    p: Point,
    r: usize,
}

fn parse_input() -> Vec<NanoBot> {
    let input = common::read_file(INPUT_PATH);
    let lines = input.lines().collect::<Vec<_>>();
    let mut ret = Vec::new();

    // Match: pos=<-9331508,90659413,35938523>, r=95038229
    let re = Regex::new(r"pos=<(.+),(.+),(.+)>, r=(\d+)").unwrap();

    for line in lines {
        let cap = re.captures(line.trim()).unwrap();
        let nb = NanoBot {
            p: Point {
                x: cap[1].parse::<i64>().unwrap(),
                y: cap[2].parse::<i64>().unwrap(),
                z: cap[3].parse::<i64>().unwrap(),
            },
            r: cap[4].parse::<usize>().unwrap(),
        };
        ret.push(nb);
    }
    return ret;
}

fn find_strongest_bot(bots: &Vec<NanoBot>) -> NanoBot {
    let mut best = bots[0].clone();

    for i in 0..bots.len() {
        if bots[i].r > best.r {
            best = bots[i].clone();
        }
    }

    return best;
}

fn distance(a: &NanoBot, b: &NanoBot) -> usize {
    let v = (a.p.x - b.p.x).abs() + (a.p.y - b.p.y).abs() + (a.p.z - b.p.z).abs();
    return v as usize;
}

fn num_bots_in_range(bot: &NanoBot, bots: &Vec<NanoBot>) -> usize {
    let mut ctr = 0;
    for bot_i in bots {
        if distance(&bot, &bot_i) <= bot.r {
            ctr += 1
        }
    }
    return ctr;
}

fn part_one() -> usize {
    let bots = parse_input();
    let best_bot = find_strongest_bot(&bots);
    return num_bots_in_range(&best_bot, &bots);
}

fn part_two() -> usize {
    let bots = parse_input();
    let (mut xmin, mut xmax) = bots.iter().fold((bots[0].p.x, bots[0].p.x), |total, next| {
        (
            std::cmp::min(total.0, next.p.x),
            std::cmp::max(total.1, next.p.x),
        )
    });
    let (mut ymin, mut ymax) = bots.iter().fold((bots[0].p.y, bots[0].p.y), |total, next| {
        (
            std::cmp::min(total.0, next.p.y),
            std::cmp::max(total.1, next.p.y),
        )
    });
    let (mut zmin, mut zmax) = bots.iter().fold((bots[0].p.z, bots[0].p.z), |total, next| {
        (
            std::cmp::min(total.0, next.p.z),
            std::cmp::max(total.1, next.p.z),
        )
    });

    let mut dist = 1;
    while dist < ((xmax - xmin) as usize) {
        dist = dist * 2;
    }

    loop {
        let mut target_count = 0;
        let mut best = (0, 0, 0);
        let mut best_val = std::usize::MAX;
        for x in (xmin..xmax + 1).step_by(dist) {
            for y in (ymin..ymax + 1).step_by(dist) {
                for z in (zmin..zmax + 1).step_by(dist) {
                    let mut count = 0;
                    for b in &bots {
                        let calc =
                            ((x - b.p.x).abs() + (y - b.p.y).abs() + (z - b.p.z).abs()) as usize;
                        if (calc as i64 - b.r as i64) / dist as i64 <= 0 {
                            count += 1;
                        }
                    }
                    if count > target_count {
                        target_count = count;
                        best_val = (x.abs() + y.abs() + z.abs()) as usize;
                        best = (x, y, z);
                    } else if count == target_count {
                        if ((x.abs() + y.abs() + z.abs()) as usize) < best_val {
                            best_val = (x.abs() + y.abs() + z.abs()) as usize;
                            best = (x, y, z);
                        }
                    }
                }
            }
        }

        if dist == 1 {
            return best_val;
        } else {
            xmin = best.0 - dist as i64;
            xmax = best.0 + dist as i64;
            ymin = best.1 - dist as i64;
            ymax = best.1 + dist as i64;;
            zmin = best.2 - dist as i64;
            zmax = best.2 + dist as i64;;
            dist = dist / 2;
        }
    }
}

pub fn solve() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
