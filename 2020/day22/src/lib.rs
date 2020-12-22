extern crate common;

use std::collections::HashSet;
use std::collections::VecDeque;

type Deck = VecDeque<usize>;
type Game = (Deck, Deck);

fn parse_input(input: &String) -> Game {
    let player_strings = input.split("\n\n").collect::<Vec<_>>();
    let mut players_parsed = Vec::new();
    for player in player_strings {
        let mut deck = VecDeque::new();
        for line in player.lines() {
            if line.contains("Player") {
                continue;
            }
            deck.push_back(line.parse::<usize>().unwrap());
        }
        players_parsed.push(deck);
    }

    return (players_parsed[0].clone(), players_parsed[1].clone());
}

fn calc_winner_sum(winner: &VecDeque<usize>) -> usize {
    return winner.iter().rev().zip(1..).map(|(i, card)| i * card).sum();
}

fn play_game(
    mut p0: VecDeque<usize>,
    mut p1: VecDeque<usize>,
    is_part2: bool,
) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut visited = HashSet::<Game>::new();

    while !(p0.is_empty() || p1.is_empty()) {
        let (p0_next, p1_next) = play_round(&mut p0, &mut p1, &mut visited, is_part2);
        p0 = p0_next;
        p1 = p1_next;
    }
    return (p0, p1);
}

fn play_round(
    p0: &mut VecDeque<usize>,
    p1: &mut VecDeque<usize>,
    visited: &mut HashSet<Game>,
    is_part2: bool,
) -> Game {
    if is_part2 {
        if visited.contains(&(p0.clone(), p1.clone())) {
            return (p0.clone(), VecDeque::new());
        }

        visited.insert((p0.clone(), p1.clone()));
    }

    let p0_card = p0.pop_front().unwrap();
    let p1_card = p1.pop_front().unwrap();

    if is_part2 && p0.len() >= p0_card && p1.len() >= p1_card {
        let (_p0_next, p1_next) = play_game(
            p0.iter().copied().take(p0_card).collect(),
            p1.iter().copied().take(p1_card).collect(),
            is_part2,
        );
        if p1_next.is_empty() {
            p0.push_back(p0_card);
            p0.push_back(p1_card);
        } else {
            p1.push_back(p1_card);
            p1.push_back(p0_card);
        }
    } else {
        if p0_card > p1_card {
            p0.push_back(p0_card);
            p0.push_back(p1_card);
        } else {
            p1.push_back(p1_card);
            p1.push_back(p0_card);
        }
    }
    return (p0.clone(), p1.clone());
}

fn part_one(input: &Game) -> usize {
    let (p0, p1) = play_game(input.0.clone(), input.1.clone(), false);
    return calc_winner_sum(if p0.is_empty() { &p1 } else { &p0 });
}

fn part_two(input: &Game) -> usize {
    let (p0, p1) = play_game(input.0.clone(), input.1.clone(), true);
    return calc_winner_sum(if p0.is_empty() { &p1 } else { &p0 });
}

pub fn solve() {
    let input = parse_input(&common::read_file("2020/day22/input"));

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "Player 1:",
            "9",
            "2",
            "6",
            "3",
            "1",
            "",
            "Player 2:",
            "5",
            "8",
            "4",
            "7",
            "10",
        ]
        .join("\n");

        let input_loop = ["Player 1:", "43", "19", "", "Player 2:", "2", "29", "14"].join("\n");

        assert_eq!(part_one(&parse_input(&input)), 306);
        assert_eq!(part_two(&parse_input(&input)), 291);
        assert_eq!(part_two(&parse_input(&input_loop)), 105);
    }
}
