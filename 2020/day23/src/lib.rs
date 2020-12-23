extern crate common;

const INPUT: &str = "496138527";

#[derive(Debug, Clone)]
struct Deck {
    next: Vec<usize>,
    start: usize,
}

fn parse_input(input: &str, part_two: bool) -> Deck {
    let mut numbers: Vec<usize> = input
        .chars()
        .map(|v| v.to_digit(10).unwrap() as usize)
        .collect();

    if part_two {
        while numbers.len() < 1_000_000 {
            numbers.push(numbers.len() + 1);
        }
    }

    let mut next = common::filled_vector(numbers.len() + 1, 0);
    for i in 0..numbers.len() {
        next[numbers[i]] = numbers[(i + 1) % numbers.len()];
    }

    return Deck {
        next: next,
        start: numbers[0],
    };
}

fn do_move(data: &mut Deck, current: &mut usize) {
    let max_deck = data.next.len() - 1;
    let mut taken = [0, 0, 0];

    let mut value_next = *current;
    for i in 0..3 {
        value_next = data.next[value_next];
        taken[i] = value_next;
    }

    // disconnect the three taken i.e. connect current to current.next.next.next.next;
    {
        // Beautiful indirection
        let new_next = data.next[data.next[data.next[data.next[*current]]]];
        data.next[*current] = new_next;
    }

    // find lower than current card
    let mut dest = *current;
    loop {
        dest = if dest > 1 { dest - 1 } else { max_deck };
        if !taken.contains(&dest) {
            break;
        }
    }

    // insert taken cards
    let old_next = data.next[dest];
    data.next[dest] = taken[0];
    data.next[taken[2]] = old_next;

    *current = data.next[*current];
}

fn part_one(mut input: Deck) -> String {
    let mut curr_value = input.start;
    for _ in 0..100 {
        do_move(&mut input, &mut curr_value);
    }

    // Create output string
    let mut curr_value = 1;
    let mut output = String::new();
    loop {
        curr_value = input.next[curr_value];
        if curr_value == 1 {
            break;
        }
        output.push_str(curr_value.to_string().as_str());
    }
    return output;
}

fn part_two(mut input: Deck) -> usize {
    let mut curr_value = input.start;

    for _ in 0..10_000_000 {
        do_move(&mut input, &mut curr_value);
    }

    let n0 = input.next[1];
    let n1 = input.next[n0];
    return n0 * n1;
}

pub fn solve() {
    println!("Part one: {}", part_one(parse_input(INPUT, false)));
    println!("Part two: {}", part_two(parse_input(INPUT, true)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(part_one(parse_input(&"389125467", false)), "67384529");
        assert_eq!(part_two(parse_input(&"389125467", true)), 149245887792);
    }
}
