extern crate common;

fn round_p1(player: char, enemy: char) -> usize {
    let round = match (player, enemy) {
        ('X', 'A') => 3,
        ('Y', 'B') => 3,
        ('Z', 'C') => 3,
        ('X', 'C') => 6,
        ('Y', 'A') => 6,
        ('Z', 'B') => 6,
        _ => 0,
    };
    return 1 + (player as usize - 'X' as usize) + round;
}

fn round_p2(player: char, enemy: char) -> usize {
    return 1 + match (player, enemy) {
        ('X', 'A') => 2,
        ('X', 'B') => 0,
        ('X', 'C') => 1,
        ('Y', 'A') => 0 + 3,
        ('Y', 'B') => 1 + 3,
        ('Y', 'C') => 2 + 3,
        ('Z', 'A') => 1 + 6,
        ('Z', 'B') => 2 + 6,
        ('Z', 'C') => 0 + 6,
        _ => 0,
    };
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let enemy: Vec<char> = input.lines().map(|v| v.as_bytes()[0] as char).collect();
    let player: Vec<char> = input.lines().map(|v| v.as_bytes()[2] as char).collect();
    let len = enemy.len();
    let part_one: usize = (0..len).map(|i| round_p1(player[i], enemy[i])).sum();
    let part_two: usize = (0..len).map(|i| round_p2(player[i], enemy[i])).sum();
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
