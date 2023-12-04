#[derive(Debug, Clone)]
struct Card {
    numbers: Vec<usize>,
    winners: Vec<usize>,
}

fn parse_input(input: &String) -> Vec<Card> {
    let mut input_cleaned = input.clone().replace("  ", " ");
    input_cleaned = input_cleaned.replace("  ", " ");
    input_cleaned = input_cleaned.replace("Card ", "");
    input_cleaned = input_cleaned.replace(":", "");

    let mut result = Vec::new();

    for line in input_cleaned.lines() {
        let mut winners: Vec<usize> = line
            .split(" | ")
            .nth(0)
            .unwrap()
            .split(" ")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let numbers: Vec<usize> = line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split(" ")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        winners.remove(0);

        result.push(Card {
            numbers: numbers,
            winners: winners,
        });
    }

    return result;
}

fn part_one(cards: &Vec<Card>) -> usize {
    let mut sum = 0;
    for card in cards {
        let mut num_hits = 0;
        for number in &card.numbers {
            num_hits += card.winners.contains(number) as u32;
        }
        if num_hits == 0 {
            continue;
        }
        sum += (2 as usize).pow(num_hits - 1);
    }

    return sum;
}

fn part_two(cards: &Vec<Card>) -> usize {
    let mut sum = 0;
    let mut num_instances = vec![1; cards.len()];
    let mut id = 0;
    for card in cards {
        let mut num_hits = 0;
        for number in &card.numbers {
            num_hits += card.winners.contains(number) as usize;
        }
        for offset in 0..num_hits {
            num_instances[id + offset + 1] += num_instances[id];
        }
        sum += num_instances[id];
        id += 1;
    }

    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let cards = parse_input(&input);
    println!("Part one: {}", part_one(&cards));
    println!("Part two: {}", part_two(&cards));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse_input(&input.to_string());
        assert_eq!(part_one(&cards), 13);
        assert_eq!(part_two(&cards), 30);
    }
}
