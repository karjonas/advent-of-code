fn execute(mut lines: Vec<i32>, part_one: bool) -> usize {
    let mut p = 0;
    let n = lines.len();
    let mut acc = 0;

    while p < n {
        let steps = lines[p];
        if part_one {
            lines[p] = steps + 1;
        } else {
            lines[p] = if steps >= 3 { steps - 1 } else { steps + 1 };
        }
        p = (p as i32 + steps) as usize;
        acc += 1;
    }

    return acc;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let lines: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect();

    println!("Part one: {}", execute(lines.clone(), true));
    println!("Part two: {}", execute(lines, false));
}
