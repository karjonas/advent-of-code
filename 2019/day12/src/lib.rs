extern crate common;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn abs(value: [i64; 3]) -> i64 {
    return value[0].abs() + value[1].abs() + value[2].abs();
}

fn potential(moon: Moon) -> i64 {
    return abs(moon.pos);
}

fn kinetic(moon: Moon) -> i64 {
    return abs(moon.vel);
}

fn total(moon: Moon) -> i64 {
    return potential(moon) * kinetic(moon);
}

fn total_energy(moons: &Vec<Moon>) -> i64 {
    return moons.iter().fold(0, |sum, moon| sum + total(moon.clone()));
}

fn simulate_step(moons: &mut Vec<Moon>) {
    simulate_step_idx(moons, 0);
    simulate_step_idx(moons, 1);
    simulate_step_idx(moons, 2);
}

fn simulate_step_idx(moons: &mut Vec<Moon>, idx: usize) {
    // Do gravity
    for i in 0..moons.len() {
        for j in (i + 1)..moons.len() {
            let v0 = moons[i].pos;
            let v1 = moons[j].pos;

            if v0[idx] < v1[idx] || v0[idx] > v1[idx] {
                moons[i].vel[idx] += if v0[idx] > v1[idx] { -1 } else { 1 };
                moons[j].vel[idx] += if v0[idx] > v1[idx] { 1 } else { -1 };
            }
        }
    }

    // Do velocity
    for i in 0..moons.len() {
        moons[i].pos[idx] += moons[i].vel[idx];
    }
}

fn parse_input(input: String) -> Vec<Moon> {
    let mut moons = Vec::new();

    for line in input.lines() {
        let v = line
            .split(',')
            .map(|s| {
                common::string_to_i64(
                    common::strip_characters(&String::from(s), "<xyz=> ").as_str(),
                )
            })
            .collect::<Vec<i64>>();

        assert_eq!(v.len(), 3);

        let m = Moon {
            pos: [v[0], v[1], v[2]],
            vel: [0, 0, 0],
        };
        moons.push(m);
    }

    return moons;
}

pub fn solve(filepath: &str) {
    let part_one;
    let part_two;
    {
        let mut moons = parse_input(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
        );
        for _i in 0..1000 {
            simulate_step(&mut moons);
        }
        part_one = total_energy(&moons);
    }

    {
        let moons_start = parse_input(
            std::fs::read_to_string(filepath)
                .unwrap()
                .trim()
                .to_string(),
        );
        let mut repeats = [0, 0, 0];

        for i in 0..3 {
            let mut ctr = 0;
            let mut moons = moons_start.clone();
            loop {
                simulate_step_idx(&mut moons, i);

                ctr += 1;
                if moons == moons_start {
                    break;
                }
            }
            repeats[i] = ctr;
        }

        part_two = common::lcm(
            repeats[2] as i128,
            common::lcm(repeats[0] as i128, repeats[1] as i128),
        );
    }
    println!("Part one: {:?}", part_one);
    println!("Part one: {:?}", part_two);
}
