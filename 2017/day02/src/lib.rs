use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day02/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let int_lines: Vec<Vec<usize>> = contents
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
