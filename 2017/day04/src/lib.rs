pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let lines: Vec<Vec<String>> = input
        .trim()
        .split('\n')
        .map(|line| line.split(' ').map(|v| v.to_string()).collect())
        .collect();

    let mut sum = 0;
    for line in &lines {
        let n = line.len();
        let mut valid = true;
        for i in 0..n {
            for j in i + 1..n {
                if line[i] == line[j] {
                    valid = false;
                }
            }
        }

        sum += if valid { 1 } else { 0 };
    }
    println!("Part one: {}", sum);

    sum = 0;
    for line in lines {
        let n = line.len();
        let mut valid = true;
        for i in 0..n {
            let mut v0: Vec<char> = line[i].chars().collect();
            v0.sort();
            for j in i + 1..n {
                let mut v1: Vec<char> = line[j].chars().collect();
                v1.sort();
                if v0 == v1 {
                    valid = false;
                }
            }
        }

        sum += if valid { 1 } else { 0 };
    }

    println!("Part two: {}", sum);
}
