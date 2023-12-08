use std::cmp::Ordering;

type Cards = [u8; 5];

#[derive(Debug, Clone)]
struct Hand {
    cards: Cards,
    value: usize,
    bid: usize,
}

fn value(c: char, part_two: bool) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if part_two {
                1
            } else {
                11
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!(""),
    }
}

fn strength(cards: &Cards) -> usize {
    // Five of a kind, where all five cards have the same label: AAAAA
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // High card, where all cards' labels are distinct: 23456
    let mut map = vec![0; 15];
    let mut strongest = 0;
    for c in cards {
        let count = map[*c as usize] + 1;
        map[*c as usize] = count;
        strongest = strongest.max(count);
    }
    let num_pairs = map.iter().filter(|v| **v > 0).count();

    match num_pairs {
        1 => 7, // AAAAA
        2 => {
            if strongest == 4 {
                6
            } else {
                5
            }
        } // AA8AA, 23332
        3 => {
            if strongest == 3 {
                4
            } else {
                3
            }
        } // TTT98, 23432
        4 => 2, // A23A4
        5 => 1, // 23456
        _ => 0,
    }
}

fn strength_joker(cards: &Cards) -> usize {
    let mut best = 0;
    for i in 0..5 {
        // Joker is 1
        if cards[i] != 1 {
            continue;
        }
        for c in cards {
            if *c == 1 {
                continue;
            }
            let mut cards_next = cards.clone();
            cards_next[i] = *c;
            best = best.max(strength_joker(&cards_next));
        }
    }

    best = best.max(strength(&cards));

    return best;
}

fn hand_cmp(l: &Hand, r: &Hand) -> Ordering {
    if l.value < r.value {
        return Ordering::Less;
    } else if l.value > r.value {
        return Ordering::Greater;
    }

    for i in 0..5 {
        if l.cards[i] < r.cards[i] {
            return Ordering::Less;
        } else if l.cards[i] > r.cards[i] {
            return Ordering::Greater;
        }
    }

    panic!("unsortable");
}

fn parse_input(input: &String, part_two: bool) -> Vec<Hand> {
    let mut result = Vec::new();
    for line in input.lines() {
        let hand_vec: Vec<u8> = line
            .split_ascii_whitespace()
            .nth(0)
            .unwrap()
            .chars()
            .map(|c| value(c, part_two))
            .collect();
        let hand = [
            hand_vec[0],
            hand_vec[1],
            hand_vec[2],
            hand_vec[3],
            hand_vec[4],
        ];
        let bid = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let value = if part_two {
            strength_joker(&hand)
        } else {
            strength(&hand)
        };
        result.push(Hand {
            cards: hand,
            value: value,
            bid: bid,
        });
    }
    result.sort_by(hand_cmp);
    return result;
}

fn sum_hands(hands: &Vec<Hand>) -> usize {
    let mut ctr = 0;
    let mut sum = 0;
    for hand in hands {
        ctr += 1;
        sum += hand.bid * ctr;
    }
    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", sum_hands(&parse_input(&input, false)));
    println!("Part two: {}", sum_hands(&parse_input(&input, true)));
}
