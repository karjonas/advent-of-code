use std::collections::HashSet;

extern crate common;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct RobotCosts {
    ore: u16,             // ore
    clay: u16,            // ore
    obsidian: (u16, u16), // ore, clay
    geode: (u16, u16),    // ore, obsidian
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
struct Inventory {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Inventory {
    pub fn empty() -> Inventory {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn increase(&mut self, robot: &Inventory) {
        self.ore += robot.ore;
        self.clay += robot.clay;
        self.obsidian += robot.obsidian;
        self.geode += robot.geode;
    }
}

fn parse(input: &String) -> Vec<RobotCosts> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let words: Vec<_> = line
            .split_ascii_whitespace()
            .map(|word| word.parse::<u16>().unwrap_or(0))
            .collect();
        let costs = RobotCosts {
            ore: words[6],
            clay: words[12],
            obsidian: (words[18], words[21]),
            geode: (words[27], words[30]),
        };
        blueprints.push(costs);
    }
    return blueprints;
}

fn part_both(input: &String, part_two: bool) -> usize {
    let blueprints = parse(input);
    let mut sum = if part_two { 1 } else { 0 };
    let mut id = 1;
    for blueprint in blueprints {
        let mut states = HashSet::new();
        // A state is a pair of (robots, material)
        states.insert((
            Inventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            Inventory::empty(),
        ));

        let mut best_geodes = 0;

        // For each minute
        let num_minutes = if part_two { 32 } else { 24 };
        for minute in 0..num_minutes {
            let mut states_next = HashSet::new();

            // for each state we generate new state where we eiter do nothing or build something
            for (robots, materials) in states {
                // thumb rule for culling bad paths
                if materials.geode + 5 < best_geodes {
                    continue;
                }

                if minute == num_minutes - 1 {
                    // Cannot build during last, just increase and check best
                    let mut materials_next = materials.clone();
                    materials_next.increase(&robots);
                    best_geodes = std::cmp::max(best_geodes, materials_next.geode);
                    continue;
                }

                best_geodes = std::cmp::max(best_geodes, materials.geode);

                // assume only one robot is built
                // build ore
                if blueprint.ore <= materials.ore {
                    let mut robots_next = robots.clone();
                    let mut materials_next = materials.clone();
                    robots_next.ore += 1;
                    materials_next.ore -= blueprint.ore;
                    materials_next.increase(&robots);
                    states_next.insert((robots_next, materials_next));
                }

                // build clay
                if blueprint.clay <= materials.ore {
                    let mut robots_next = robots.clone();
                    let mut materials_next = materials.clone();
                    robots_next.clay += 1;
                    materials_next.ore -= blueprint.clay;
                    materials_next.increase(&robots);
                    states_next.insert((robots_next, materials_next));
                }

                // build obsidian
                if blueprint.obsidian.0 <= materials.ore && blueprint.obsidian.1 <= materials.clay {
                    let mut robots_next = robots.clone();
                    let mut materials_next = materials.clone();
                    robots_next.obsidian += 1;
                    materials_next.ore -= blueprint.obsidian.0;
                    materials_next.clay -= blueprint.obsidian.1;
                    materials_next.increase(&robots);
                    states_next.insert((robots_next, materials_next));
                }

                // build geode
                if blueprint.geode.0 <= materials.ore && blueprint.geode.1 <= materials.obsidian {
                    let mut robots_next = robots.clone();
                    let mut materials_next = materials.clone();
                    robots_next.geode += 1;
                    materials_next.ore -= blueprint.geode.0;
                    materials_next.obsidian -= blueprint.geode.1;
                    materials_next.increase(&robots);
                    states_next.insert((robots_next, materials_next));
                }

                let wait_ore = materials.ore < blueprint.ore;
                let wait_clay = materials.ore < blueprint.clay;
                let wait_obsidian = robots.clay > 0
                    && (materials.ore < blueprint.obsidian.0
                        || materials.clay < blueprint.obsidian.1);
                let wait_geode = robots.obsidian > 0
                    && (materials.ore < blueprint.geode.0
                        || materials.obsidian < blueprint.geode.1);

                // do nothing
                if wait_ore || wait_clay || wait_obsidian || wait_geode {
                    let mut materials_next = materials.clone();
                    materials_next.increase(&robots);
                    states_next.insert((robots, materials_next));
                }
            }

            states = states_next;
        }

        if part_two {
            sum = sum * best_geodes;
            if id == 3 {
                break;
            }
        } else {
            sum += best_geodes * id;
        }
        id += 1;
    }
    sum as usize
}

fn part_one(input: &String) -> usize {
    part_both(input, false)
}

fn part_two(input: &String) -> usize {
    part_both(input, true)
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input0 = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string();

        assert_eq!(part_one(&input0), 33);
    }
}
