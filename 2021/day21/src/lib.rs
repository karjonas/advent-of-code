extern crate common;
#[macro_use]
extern crate scan_fmt;
use std::collections::HashMap;

fn modulo(mut value: usize, min: usize, max: usize) -> usize {
    if value >= max {
        let range = max - min;
        let q = 1 + (value - max) / range;
        value -= q * range;
    }

    return value;
}

fn parse_input(input: &String) -> (usize, usize) {
    let (p1, p2) = scan_fmt!(
        input,
        "Player 1 starting position: {d}\nPlayer 2 starting position: {d}",
        usize,
        usize
    )
    .unwrap();
    return (p1, p2);
}

fn play_game(p1: usize, p2: usize) -> usize {
    let mut pos = [p1, p2];
    let mut scores = [0, 0];
    let mut dice = 1;
    let mut player = 0;
    loop {
        let player_other = (player + 1) % 2;
        let roll = dice * 3 + 3;
        dice += 3;
        pos[player] = modulo(pos[player] + roll, 1, 11);
        scores[player] += pos[player];
        if scores[player] >= 1000 {
            return scores[player_other] * (dice - 1);
        }
        player = player_other;
    }
}

static FREQS: &'static [(usize, usize)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn solve_recursive(
    pos_1: usize,
    pos_2: usize,
    score_1: usize,
    score_2: usize,
    cache: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
) -> (usize, usize) {
    let key = (pos_1, pos_2, score_1, score_2);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    if score_1 >= 21 {
        return (1, 0);
    }
    if score_2 >= 21 {
        return (0, 1);
    }

    let mut total_p1_wins = 0;
    let mut total_p2_wins = 0;

    for (roll, freq) in FREQS {
        let new_position = modulo(pos_1 + roll, 1, 11);
        let new_score = score_1 + new_position;

        let (p2_wins, p1_wins) = solve_recursive(pos_2, new_position, score_2, new_score, cache);

        total_p1_wins += freq * p1_wins;
        total_p2_wins += freq * p2_wins;
    }

    cache.insert(
        (pos_1, pos_2, score_1, score_2),
        (total_p1_wins, total_p2_wins),
    );

    return (total_p1_wins, total_p2_wins);
}

fn play_game_p2(p1: usize, p2: usize) -> usize {
    let (wins_p1, wins_p2) = solve_recursive(p1, p2, 0, 0, &mut HashMap::new());
    return std::cmp::max(wins_p1, wins_p2);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let (p1, p2) = parse_input(&input);

    println!("Part one: {}", play_game(p1, p2));
    println!("Part two: {}", play_game_p2(p1, p2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(modulo(10, 1, 11), 10);
        assert_eq!(modulo(23, 1, 11), 3);
        assert_eq!(modulo(24, 1, 11), 4);
        assert_eq!(modulo(30, 1, 11), 10);
        assert_eq!(modulo(31, 1, 11), 1);
        assert_eq!(modulo(41, 1, 11), 1);

        let input = "Player 1 starting position: 4
Player 2 starting position: 8"
            .to_string();
        let (p1, p2) = parse_input(&input);
        assert_eq!(play_game(p1, p2), 739785);
        assert_eq!(play_game_p2(p1, p2), 444356092776315);
    }
}
