extern crate common;

const HERO_HIT_POINTS: i64 = 50;
const HERO_MANA: i64 = 500;
const BOSS_HIT_POINTS: i64 = 51;
const BOSS_DAMAGE: i64 = 9;

#[derive(Debug, Clone, PartialEq)]
enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, PartialEq)]
struct Spell {
    spell_type: SpellType,
    mana_charge: usize,
    damage: usize,
    duration: usize,
    armor: usize,
    health_charge: usize,
    cost: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Game {
    hero_hitpoints: i64,
    hero_mana: i64,

    boss_hitpoints: i64,
    boss_damage: i64,

    active_spells: Vec<Spell>,

    spent_mana: usize,
    best_mana: usize,

    hard_mode: bool,

    history: Vec<Spell>,
}

const SPELLS: [Spell; 5] = [
    Spell {
        spell_type: SpellType::Poison,
        mana_charge: 0,
        damage: 3,
        duration: 6,
        armor: 0,
        health_charge: 0,
        cost: 173,
    },
    Spell {
        spell_type: SpellType::Shield,
        mana_charge: 0,
        damage: 0,
        duration: 6,
        armor: 7,
        health_charge: 0,
        cost: 113,
    },
    Spell {
        spell_type: SpellType::Recharge,
        mana_charge: 101,
        damage: 0,
        duration: 5,
        armor: 0,
        health_charge: 0,
        cost: 229,
    },
    Spell {
        spell_type: SpellType::MagicMissile,
        mana_charge: 0,
        damage: 4,
        duration: 0,
        armor: 0,
        health_charge: 0,
        cost: 53,
    },
    Spell {
        spell_type: SpellType::Drain,
        mana_charge: 0,
        damage: 2,
        duration: 0,
        armor: 0,
        health_charge: 2,
        cost: 73,
    },
];

fn execute_spells(game: &mut Game) {
    for spell in &mut *game.active_spells {
        game.boss_hitpoints -= spell.damage as i64;
        game.hero_mana += spell.mana_charge as i64;
        game.hero_hitpoints += spell.health_charge as i64;
        spell.duration -= 1;
    }

    game.active_spells = game
        .active_spells
        .iter()
        .filter(|spell| spell.duration != 0)
        .map(|v| v.clone())
        .collect::<Vec<_>>();
}

fn fight_recursive(game_in: &Game) -> (bool, Game) {
    let mut best_mana = game_in.best_mana;
    let mut best_game = game_in.clone();
    let mut win = false;

    // Try all spells in order and recurse
    for spell in &SPELLS {
        let mut game = game_in.clone();

        if game.hard_mode {
            game.hero_hitpoints -= 1;
        }

        if game.hero_hitpoints <= 0 {
            return (false, game);
        }

        // Execute active spells and decrease durations
        execute_spells(&mut game);

        // Poison death
        if game.boss_hitpoints <= 0 {
            return (true, game.clone());
        }

        let spell_already_active = game
            .active_spells
            .iter()
            .find(|v| v.spell_type == spell.spell_type)
            != None;

        if spell_already_active || spell.cost as i64 > game.hero_mana {
            continue;
        }

        game.history.push(spell.clone());

        // Execute immediately if magic missile or drain
        if spell.spell_type == SpellType::MagicMissile {
            game.boss_hitpoints -= 4;
        } else if spell.spell_type == SpellType::Drain {
            game.boss_hitpoints -= 2;
            game.hero_hitpoints += 2;
        } else {
            // Other spell, add to spell list
            game.active_spells.push(spell.clone());
        }

        game.spent_mana += spell.cost;
        game.hero_mana -= spell.cost as i64;

        if game.spent_mana > best_mana {
            continue;
        }

        if game.boss_hitpoints <= 0 {
            if game.spent_mana < best_mana {
                game.best_mana = best_mana;
                best_mana = game.spent_mana;
                best_game = game.clone();
                win = true;
            }
            continue;
        }

        // Boss turn begins
        execute_spells(&mut game);
        if game.boss_hitpoints <= 0 {
            if game.spent_mana < best_mana {
                game.best_mana = best_mana;
                best_mana = game.spent_mana;
                best_game = game.clone();
                win = true;
            }
            continue;
        }

        // Boss attack
        let has_armor = game
            .active_spells
            .iter()
            .find(|v| v.spell_type == SpellType::Shield)
            != None;
        let damage = game.boss_damage - if has_armor { 7 } else { 0 };

        game.hero_hitpoints -= std::cmp::max(1, damage);

        if game.hero_hitpoints <= 0 {
            continue;
        }

        game.best_mana = best_mana;
        let (success, game_next) = fight_recursive(&game.clone());

        if success {
            if game_next.spent_mana < best_mana {
                best_mana = game_next.spent_mana;
                best_game = game_next.clone();
                best_game.best_mana = game_next.spent_mana;
                win = true;
            }
        }
    }

    return (win, best_game);
}

pub fn solve() {
    let base_game = Game {
        hero_hitpoints: HERO_HIT_POINTS,
        hero_mana: HERO_MANA,

        boss_hitpoints: BOSS_HIT_POINTS,
        boss_damage: BOSS_DAMAGE,

        active_spells: Vec::new(),

        spent_mana: 0,
        best_mana: std::usize::MAX,

        hard_mode: false,

        history: Vec::new(),
    };

    {
        let (win, game) = fight_recursive(&base_game);
        assert!(win);
        println!("Part one: {}", game.spent_mana);
    }
    {
        let mut game_hard = base_game.clone();
        game_hard.hard_mode = true;

        let (win, game) = fight_recursive(&game_hard);
        assert!(win);
        println!("Part two: {}", game.spent_mana);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let game_a = Game {
            hero_hitpoints: 10,
            hero_mana: 250,

            boss_hitpoints: 13,
            boss_damage: 8,

            active_spells: Vec::new(),

            spent_mana: 0,
            best_mana: std::usize::MAX,

            hard_mode: false,

            history: Vec::new(),
        };

        let game_b = Game {
            hero_hitpoints: 10,
            hero_mana: 250,

            boss_hitpoints: 14,
            boss_damage: 8,

            active_spells: Vec::new(),

            spent_mana: 0,
            best_mana: std::usize::MAX,

            hard_mode: false,

            history: Vec::new(),
        };

        let fight_a = fight_recursive(&game_a);
        let fight_b = fight_recursive(&game_b);
        assert_eq!(fight_a.0, true);
        assert_eq!(fight_b.0, true);
    }
}
