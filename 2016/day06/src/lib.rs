use std::collections::HashMap;

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let lines: Vec<_> = input.lines().collect();
    let num_cols = lines[0].len();

    let mut decrypted0 = String::new();
    let mut decrypted1 = String::new();

    for i in 0..num_cols {
        let mut hm = HashMap::new();
        for line in &lines {
            assert!(line.len() == num_cols);
            let c = line.chars().nth(i).unwrap();
            let hits = match hm.get(&c) {
                Some(&number) => number,
                _ => 0,
            };
            hm.insert(c, hits + 1);
        }

        let (max_char, _) = hm.iter().fold(('0', 0), |sum, pair| {
            if sum.0 == '0' || pair.1 > &sum.1 {
                (pair.0.clone(), pair.1.clone())
            } else {
                sum
            }
        });
        let (min_char, _) = hm.iter().fold(('0', 0), |sum, pair| {
            if sum.0 == '0' || pair.1 < &sum.1 {
                (pair.0.clone(), pair.1.clone())
            } else {
                sum
            }
        });

        decrypted0.push(max_char);
        decrypted1.push(min_char);
    }

    println!("Part1: {}", decrypted0);
    println!("Part2: {}", decrypted1);
}
