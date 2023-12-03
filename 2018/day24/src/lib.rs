extern crate common;
extern crate regex;

use regex::Regex;

const DEBUG_PRINT: bool = false;

#[derive(Debug, Clone)]
struct Group {
    units: usize,
    hitpoints: usize,
    initiative: usize,
    attack: usize,

    attack_type: String,
    weaknesses: Vec<String>,
    immunities: Vec<String>,

    id: usize,
    target: i64,
}

#[derive(Debug, Clone)]
struct Battle {
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

fn parse_input(input: &String) -> Battle {
    let lines = input.lines().collect::<Vec<_>>();

    let re = Regex::new(r"(.*) \((.*)\) (.*)").unwrap();
    let re_immunes = Regex::new(r"(.*); (.*)").unwrap();

    let mut immune_system = Vec::new();
    let mut infection = Vec::new();

    let mut is_immune_system = true;
    for line in lines {
        if line == "Immune System:" {
            is_immune_system = true;
            continue;
        }
        if line == "Infection:" {
            is_immune_system = false;
            continue;
        }
        if line == "" {
            continue;
        }

        let mut basic_info = line.to_string();
        let mut immunes = Vec::new();
        let mut weaknesses = Vec::new();

        if line.find('(') != None {
            let mut add_immune_weak = |s: &str| {
                //let ss = s.clone();
                let vector = s.split(' ').map(|v| v.to_string()).collect::<Vec<_>>();
                let type_str = vector[0].clone();
                for i in 2..vector.len() {
                    if type_str == "immune" {
                        immunes.push(vector[i].clone());
                    } else {
                        weaknesses.push(vector[i].clone());
                    }
                }
            };
            let cap = re.captures(line).unwrap();
            let pre = cap[1].to_string();
            let mid = cap[2].to_string();
            let post = cap[3].to_string();

            basic_info = pre + " " + post.as_str();
            let extra_info = common::strip_characters(mid.as_str(), ",");

            if extra_info.find(';') != None {
                let cap_immunes = re_immunes.captures(extra_info.as_str()).unwrap();
                let first = cap_immunes[1].to_string();
                let second = cap_immunes[2].to_string();

                add_immune_weak(&first);
                add_immune_weak(&second);
            } else {
                add_immune_weak(&extra_info);
            }
        }

        let basic_info_vec = basic_info.split(' ').collect::<Vec<_>>();

        let units = common::string_to_i64(basic_info_vec[0]) as usize;
        let hitpoints = common::string_to_i64(basic_info_vec[4]) as usize;
        let attack = common::string_to_i64(basic_info_vec[12]) as usize;
        let attack_type = basic_info_vec[13];
        let initiative = common::string_to_i64(basic_info_vec[17]) as usize;

        let mut group = Group {
            units: units,
            hitpoints: hitpoints,
            initiative: initiative,
            attack: attack,

            attack_type: attack_type.to_string(),
            weaknesses: weaknesses,
            immunities: immunes,

            id: 0,
            target: 0,
        };

        if is_immune_system {
            group.id = immune_system.len() + 1;
            immune_system.push(group)
        } else {
            group.id = infection.len() + 1;
            infection.push(group)
        }
    }
    return Battle {
        immune_system: immune_system,
        infection: infection,
    };
}

fn sort_army_select_order(army: &mut Vec<Group>) {
    let mut army_cpy = army.clone();
    army.clear();

    while !army_cpy.is_empty() {
        let mut best_effective = 0;
        let mut best_initiative = 0;
        let mut best_idx = 0;

        for i in 0..army_cpy.len() {
            let effective = army_cpy[i].units * army_cpy[i].attack;
            if effective > best_effective
                || (effective == best_effective && army_cpy[i].initiative > best_initiative)
            {
                best_effective = effective;
                best_initiative = army_cpy[i].initiative;
                best_idx = i;
            }
        }

        army.push(army_cpy.remove(best_idx));
    }
}

fn print_status(battle_in: &Battle) {
    let mut battle = battle_in.clone();
    battle.immune_system.sort_by(|a, b| a.id.cmp(&b.id));
    battle.infection.sort_by(|a, b| a.id.cmp(&b.id));

    println!("Immune System:");
    for b in battle.immune_system {
        if b.units > 0 {
            println!("Group {} contains {} units", b.id, b.units);
        }
    }

    println!("Infection:");
    for b in battle.infection {
        if b.units > 0 {
            println!("Group {} contains {} units", b.id, b.units);
        }
    }
}

fn calculate_damage(attacker: &Group, target: &Group) -> usize {
    let base_damage = attacker.units * attacker.attack;

    if target.immunities.contains(&attacker.attack_type) {
        return 0;
    } else if target.weaknesses.contains(&attacker.attack_type) {
        return base_damage * 2;
    } else {
        return base_damage;
    }
}

fn select_attack(attacker: &mut Vec<Group>, target: &Vec<Group>, is_infection: bool) {
    let mut selected = common::filled_vector(target.len(), false);

    for i in 0..attacker.len() {
        // Skip dead
        if attacker[i].units == 0 {
            continue;
        }

        attacker[i].target = -1;

        let mut best_damage = 0;
        let mut best_defend_damage = 0;
        let mut best_defend_initiative = 0;
        let mut best_idx = -1;

        for j in 0..target.len() {
            if selected[j] {
                continue;
            }

            if target[j].units == 0 {
                continue;
            }

            let damage = calculate_damage(&attacker[i], &target[j]);
            let defend_damage = target[j].units * target[j].attack;
            let defend_initiative = target[j].initiative;

            if DEBUG_PRINT {
                println!(
                    "{} group {} would deal defending group {} {} damage",
                    if is_infection {
                        "Infection"
                    } else {
                        "Immune System"
                    },
                    attacker[i].id,
                    target[j].id,
                    damage
                );
            }
            let mut new_best = false;

            if damage == 0 {
                // no op
            } else if damage > best_damage {
                new_best = true;
            } else if damage == best_damage && defend_damage > best_defend_damage {
                new_best = true;
            } else if damage == best_damage
                && defend_damage == best_defend_damage
                && defend_initiative > best_defend_initiative
            {
                new_best = true;
            }

            if new_best {
                best_damage = damage;
                best_defend_damage = defend_damage;
                best_defend_initiative = defend_initiative;
                best_idx = j as i64;
            }
        }

        if best_idx != -1 {
            if DEBUG_PRINT {
                println!(
                    "{} group {} selected group {}",
                    if is_infection {
                        "Infection"
                    } else {
                        "Immune System"
                    },
                    attacker[i].id,
                    target[best_idx as usize].id
                );
            }
            selected[best_idx as usize] = true;
            attacker[i].target = target[best_idx as usize].id as i64;
        }
    }
}

fn deal_damage(attacker: &Group, defender: &mut Group) -> usize {
    let dmg = calculate_damage(attacker, defender);
    let units_killed = std::cmp::min(defender.units, dmg / defender.hitpoints);

    defender.units -= units_killed;
    return units_killed;
}

fn target_to_idx(target: usize, groups: &Vec<Group>) -> usize {
    for i in 0..groups.len() {
        if groups[i].id == target {
            return i;
        }
    }
    assert!(false);
    return 0;
}

fn attack(immune_system: &mut Vec<Group>, infection: &mut Vec<Group>) {
    let num_groups = immune_system.len() + infection.len();

    for initiative_inv in 0..num_groups {
        let initiative = num_groups - initiative_inv;

        for gis in immune_system.iter() {
            if gis.initiative == initiative && gis.units > 0 && gis.target != -1 {
                let idx = target_to_idx(gis.target as usize, &infection);
                let dmg = deal_damage(&gis, &mut infection[idx]);

                if DEBUG_PRINT {
                    println!(
                        "Immune System group {} attacks defending group {}, killing {} units",
                        gis.id, gis.target, dmg
                    );
                }
            }
        }

        for gin in infection.iter() {
            if gin.initiative == initiative && gin.units > 0 && gin.target != -1 {
                let idx = target_to_idx(gin.target as usize, &immune_system);
                let dmg = deal_damage(&gin, &mut immune_system[idx]);

                if DEBUG_PRINT {
                    println!(
                        "Infection group {} attacks defending group {}, killing {} units",
                        gin.id, gin.target, dmg
                    );
                }
            }
        }
    }
}

fn do_round(battle: &mut Battle) -> bool {
    let num_immunes = battle.immune_system.iter().fold(0, |sum, g| sum + g.units);
    let num_infection = battle.infection.iter().fold(0, |sum, g| sum + g.units);

    if num_immunes == 0 || num_infection == 0 {
        return false;
    }

    if DEBUG_PRINT {
        print_status(&battle);
    }
    sort_army_select_order(&mut battle.immune_system);
    sort_army_select_order(&mut battle.infection);

    if DEBUG_PRINT {
        println!();
    }
    select_attack(&mut battle.infection, &mut battle.immune_system, true);
    select_attack(&mut battle.immune_system, &mut battle.infection, false);

    if DEBUG_PRINT {
        println!();
    }
    attack(&mut battle.immune_system, &mut battle.infection);

    if DEBUG_PRINT {
        println!();
    }

    let num_immunes_next = battle.immune_system.iter().fold(0, |sum, g| sum + g.units);
    let num_infection_next = battle.infection.iter().fold(0, |sum, g| sum + g.units);

    return num_immunes != num_immunes_next || num_infection != num_infection_next;
}

fn part_one(battle_in: &Battle) {
    let mut battle = battle_in.clone();
    while do_round(&mut battle) {}

    let units_left = battle.immune_system.iter().fold(0, |sum, g| sum + g.units)
        + battle.infection.iter().fold(0, |sum, g| sum + g.units);
    println!("Part one: {}", units_left);
}

fn part_two(battle_in: &Battle) {
    let mut boost = 0;
    loop {
        let mut battle = battle_in.clone();
        for i in 0..battle.immune_system.len() {
            battle.immune_system[i].attack += boost;
        }

        while do_round(&mut battle) {}

        let num_immunes = battle.immune_system.iter().fold(0, |sum, g| sum + g.units);
        let num_infection = battle.infection.iter().fold(0, |sum, g| sum + g.units);

        if num_immunes > 0 && num_infection == 0 {
            println!("Part two: {}", num_immunes);
            return;
        }

        boost += 1;
    }
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let battle = parse_input(&input);

    part_one(&battle);
    part_two(&battle);
}
