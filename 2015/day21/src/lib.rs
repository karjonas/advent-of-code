extern crate common;

const BOSS_HIT_POINTS: i64 = 109;
const BOSS_DAMAGE: i64 = 8;
const BOSS_ARMOR: i64 = 2;

#[derive(Debug, Clone, PartialEq)]
struct Item {
    name: String,
    cost: i64,
    damage: i64,
    armor: i64,
}

fn get_weapons() -> Vec<Item> {
    return vec![
        Item {
            name: "Dagger".to_string(),
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            name: "Shortsword".to_string(),
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            name: "Warhammer".to_string(),
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            name: "Longsword".to_string(),
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            name: "Greataxe".to_string(),
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];
}

fn get_rings() -> Vec<Item> {
    return vec![
        Item {
            name: "Damage +1".to_string(),
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            name: "Damage +2".to_string(),
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            name: "Damage +3".to_string(),
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            name: "Defense +1".to_string(),
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            name: "Defense +2".to_string(),
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            name: "Defense +3".to_string(),
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];
}

fn get_armors() -> Vec<Item> {
    return vec![
        Item {
            name: "Leather".to_string(),
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            name: "Chainmail".to_string(),
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            name: "Splintmail".to_string(),
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            name: "Bandedmail".to_string(),
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            name: "Platemail".to_string(),
            cost: 102,
            damage: 0,
            armor: 5,
        },
        Item {
            name: "Empty".to_string(),
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ];
}

fn get_possible_combinations() -> Vec<Vec<Item>> {
    let mut combinations = Vec::new();
    let weapons = get_weapons();
    let armors = get_armors();
    let rings = get_rings();

    for weapon in &weapons {
        for armor in &armors {
            // No ring combo
            combinations.push(vec![weapon.clone(), armor.clone()]);
            for ring_a in &rings {
                // One ring combo
                combinations.push(vec![weapon.clone(), armor.clone(), ring_a.clone()]);
                for ring_b in &rings {
                    if ring_a == ring_b {
                        continue;
                    }
                    // Two ring combo
                    combinations.push(vec![
                        weapon.clone(),
                        armor.clone(),
                        ring_a.clone(),
                        ring_b.clone(),
                    ]);
                }
            }
        }
    }

    return combinations;
}

fn fight(
    hero_hitpoints: i64,
    hero_damage: i64,
    hero_armor: i64,
    boss_hitpoints: i64,
    boss_damage: i64,
    boss_armor: i64,
) -> bool {
    let mut hp_hero = hero_hitpoints;
    let mut hp_boss = boss_hitpoints;

    while hp_boss > 0 || hp_hero > 0 {
        hp_boss -= std::cmp::max(1, hero_damage - boss_armor);

        if hp_boss <= 0 {
            break;
        }

        hp_hero -= std::cmp::max(1, boss_damage - hero_armor);
    }

    return hp_hero > 0;
}

pub fn solve(_filepath: &str) {
    let mut best_cost = std::i64::MAX;
    let mut worst_cost = 0;
    let combinations = get_possible_combinations();
    for comb in &combinations {
        let hero_damage = comb.iter().fold(0, |sum, item| sum + item.damage);
        let hero_armor = comb.iter().fold(0, |sum, item| sum + item.armor);
        let cost = comb.iter().fold(0, |sum, item| sum + item.cost);

        let win = fight(
            100,
            hero_damage,
            hero_armor,
            BOSS_HIT_POINTS,
            BOSS_DAMAGE,
            BOSS_ARMOR,
        );

        if win {
            best_cost = std::cmp::min(best_cost, cost);
        } else {
            worst_cost = std::cmp::max(worst_cost, cost);
        }
    }

    println!("Part one: {}", best_cost);
    println!("Part two: {}", worst_cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(fight(8, 5, 5, 12, 7, 2), true);
    }
}
