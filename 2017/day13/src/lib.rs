use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let mut file = File::open("2017/day13/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents = contents.chars().filter(|v| v.clone() != ':').collect();

    let lines: Vec<Vec<_>> = contents
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut firewall = Vec::<usize>::new();

    for l in lines {
        let fw_idx = l[0];
        let fw_depth = l[1];
        firewall.resize(fw_idx + 1, 0);
        firewall[fw_idx] = fw_depth;
    }

    let scanner_pos = |time: usize, index: usize| -> usize {
        let depth = firewall[index];
        let wrap = depth * 2 - 2;
        let i = time % wrap;
        if i >= depth {
            wrap - i
        } else {
            i
        }
    };

    let do_sweep = |wait: usize, bail: bool| -> (bool, usize) {
        let mut accum = 0;
        let mut hit = false;
        for i in 0..firewall.len() {
            let i_adj = i + wait;
            let depth = firewall[i];
            if depth > 0 {
                let pos = scanner_pos(i_adj, i);
                if pos == 0 {
                    accum += depth * i;
                    hit = true;
                    if bail {
                        break;
                    }
                }
            }
        }

        (hit, accum)
    };

    let (_, accum) = do_sweep(0, false);
    println!("Part one: {}", accum);

    for wait in 0..std::usize::MAX {
        let (hit, _) = do_sweep(wait, true);
        if !hit {
            println!("Part two: {}", wait);
            break;
        }
    }
}
