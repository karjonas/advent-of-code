extern crate regex;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Particle {
    pos: (i32, i32, i32),
    pos_last: (i32, i32, i32),

    vel: (i32, i32, i32),
    acc: (i32, i32, i32),

    collided: bool,
}

pub fn solve() {
    let mut file = File::open("2017/day20/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let regex = Regex::new(r".*<(.*)>.*<(.*)>.*<(.*)>").unwrap();
    let mut ctr = 0;
    let mut best_idx = 0;
    let mut best_acc = 99999;

    let mut particles = Vec::<Particle>::new();

    for line in contents.lines() {
        let cap = regex.captures(line).unwrap();
        let p: Vec<_> = cap[1]
            .to_string()
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        let v: Vec<_> = cap[2]
            .to_string()
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        let a: Vec<_> = cap[3]
            .to_string()
            .split(",")
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        let a_v = a.iter().fold(0, |acc, &x| acc + x.abs());
        if a_v < best_acc {
            best_acc = a_v;
            best_idx = ctr;
        }

        ctr += 1;

        let part = Particle {
            pos: (p[0], p[1], p[2]),
            pos_last: (p[0], p[1], p[2]),

            vel: (v[0], v[1], v[2]),
            acc: (a[0], a[1], a[2]),
            collided: false,
        };

        particles.push(part);
    }

    let add = |p0: (i32, i32, i32), p1: (i32, i32, i32)| -> (i32, i32, i32) {
        (p0.0 + p1.0, p0.1 + p1.1, p0.2 + p1.2)
    };

    let dist = |p0: (i32, i32, i32), p1: (i32, i32, i32)| -> i32 {
        (p0.0 - p1.0).abs() + (p0.1 - p1.1).abs() + (p0.2 - p1.2).abs()
    };

    let mut num_particles;

    loop {
        let mut change = false;
        particles.retain(|ref particle| !particle.collided);
        num_particles = particles.len();

        for i in 0..num_particles {
            let ref mut particle = particles[i];
            particle.pos_last = particle.pos;
            particle.vel = add(particle.vel, particle.acc);
            particle.pos = add(particle.pos, particle.vel);
        }

        for i in 0..num_particles {
            let part_i = particles[i].clone();

            for j in i + 1..num_particles {
                let part_j = particles[j].clone();

                let dist_last = dist(part_i.pos_last, part_j.pos_last);
                let dist = dist(part_i.pos, part_j.pos);

                if dist < dist_last {
                    change = true;
                }

                if part_i.pos == part_j.pos {
                    particles[i].collided = true;
                    particles[j].collided = true;
                }
            }
        }

        if !change {
            break;
        }
    }

    println!("Part one: {:?}", best_idx);
    println!("Part two: {:?}", num_particles);
}
