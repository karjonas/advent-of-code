pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let int_lines: Vec<Vec<usize>> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split('\t')
                .map(|v| v.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    {
        let mut hash = 0;
        for line in &int_lines {
            let mut min = std::usize::MAX;
            let mut max = 0;

            for v in line {
                max = std::cmp::max(max, v.clone());
                min = std::cmp::min(min, v.clone());
            }

            hash = hash + (max - min);
        }
        println!("Part one: {}", hash);
    }

    {
        let mut hash = 0;
        for line in int_lines {
            let n = line.len();
            let mut v = 0;
            for i in 0..n {
                let v_i = line[i];
                for j in (i + 1)..n {
                    let v_j = line[j];
                    let max = std::cmp::max(v_i, v_j);
                    let min = std::cmp::min(v_i, v_j);

                    if max % min == 0 {
                        v = max / min;
                    }
                }
            }

            hash = hash + v;
        }
        println!("Part two: {}", hash);
    }
}
