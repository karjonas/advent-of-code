pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let ints: Vec<usize> = input
        .chars()
        .take_while(|v| v.clone() != '\n')
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect();

    let mut sum = 0;
    let n = ints.len();
    for i in 0..n {
        sum = sum
            + if ints[i] == ints[(i + 1) % n] {
                ints[i]
            } else {
                0
            };
    }

    println!("Part one: {}", sum);

    sum = 0;
    for i in 0..n {
        sum = sum
            + if ints[i] == ints[(i + n / 2) % n] {
                ints[i]
            } else {
                0
            };
    }

    println!("Part two: {}", sum);
}
