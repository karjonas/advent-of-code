use std::collections::HashMap;

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let bank_values: Vec<i32> = input
        .trim()
        .split('\t')
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect();

    assert!(bank_values.len() == 16);

    let mut vals: [u8; 16] = [0; 16];

    for i in 0..16 {
        vals[i] = bank_values[i] as u8;
    }

    let mut visited = HashMap::new();
    let mut cycles = 0;
    let cycle_size;

    loop {
        match visited.get(&vals) {
            Some(cycle) => {
                cycle_size = cycles - cycle;
                break;
            }
            None => (),
        }
        visited.insert(vals, cycles);

        let max = vals.iter().fold(0, |sum, v| std::cmp::max(sum, v.clone()));
        let index = vals.iter().position(|&v| v == max).unwrap();
        let steps = vals[index] as usize;

        // Clear value and redistribute
        vals[index] = 0;
        for i in 0..steps {
            let idx = (i + index + 1) % vals.len();
            vals[idx] += 1;
        }

        cycles += 1;
    }

    println!("Part one: {}", cycles);
    println!("Part two: {}", cycle_size);
}
