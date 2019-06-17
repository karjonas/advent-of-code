extern crate common;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    endurance: usize,
    rest: usize,
}

fn parse(input: &String) -> Vec<Reindeer> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
        let r = Reindeer {
            name: words[0].to_string(),
            speed: common::string_to_usize(words[3]),
            endurance: common::string_to_usize(words[6]),
            rest: common::string_to_usize(words[13]),
        };

        ret.push(r);
    }

    return ret;
}

fn solve_internal(reindeers: &Vec<Reindeer>, end_time: usize) -> (usize, usize) {
    let mut best_dist = 0;
    let mut dist_travelled =
        common::filled_vector(reindeers.len(), common::filled_vector(end_time, 0));

    for r_i in 0..reindeers.len() {
        let r = &reindeers[r_i];
        let mut travelled = 0;
        let mut flying = true;
        let mut rest_left = 0;
        let mut endurance_left = r.endurance;

        for t_i in 0..end_time {
            if flying {
                travelled += r.speed;
                endurance_left -= 1;
            } else {
                rest_left -= 1;
            }

            if flying && endurance_left == 0 {
                flying = false;
                rest_left = r.rest;
            } else if !flying && rest_left == 0 {
                flying = true;
                endurance_left = r.endurance;
            }

            dist_travelled[r_i][t_i] = travelled;
        }

        best_dist = std::cmp::max(best_dist, travelled);
    }

    let mut points = common::filled_vector(reindeers.len(), 0);
    for t_i in 0..end_time {
        let mut best_value = 0;
        for r_i in 0..reindeers.len() {
            if dist_travelled[r_i][t_i] > best_value {
                best_value = dist_travelled[r_i][t_i];
            }
        }

        for r_i in 0..reindeers.len() {
            if dist_travelled[r_i][t_i] == best_value {
                points[r_i] += 1;
            }
        }
    }

    let best_points = points.iter().fold(0, |sum, v| std::cmp::max(sum, *v));
    return (best_dist, best_points);
}

pub fn solve() {
    let input = common::read_file("2015/day14/input");
    let (p1, p2) = solve_internal(&parse(&input.to_string()), 2503);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(
            solve_internal(&parse(&input.to_string()), 1000),
            (1120, 689)
        );
    }
}
