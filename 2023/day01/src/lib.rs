fn part_one(input: &String) -> u32 {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    return lines.iter().fold(0, |acc, line| {
        acc + 10 * line.first().unwrap() + line.last().unwrap()
    });
}

fn find_number(line: &str, find_first: bool) -> u32 {
    let dict = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut best_idx = if find_first { std::usize::MAX } else { 0 };
    const MAGIC_FAIL: usize = 123456789;
    let mut result = 0;

    for (from, to) in &dict {
        let idx;
        let found;
        if find_first {
            idx = line.find(from).unwrap_or(MAGIC_FAIL);
            found = idx != MAGIC_FAIL && idx <= best_idx;
        } else {
            idx = line.rfind(from).unwrap_or(MAGIC_FAIL);
            found = idx != MAGIC_FAIL && idx >= best_idx;
        }
        if found {
            result = *to;
            best_idx = idx;
        }
    }
    return result;
}

fn part_two(input: &String) -> u32 {
    return input
        .lines()
        .map(|line| {
            let l = find_number(line, true) * 10;
            let r = find_number(line, false);
            l + r
        })
        .sum();
}

pub fn solve() {
    let input = std::fs::read_to_string("2023/day01/input")
        .unwrap()
        .trim()
        .to_string();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
