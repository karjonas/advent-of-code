use std::fs::File;
use std::io::prelude::*;

fn hash_sparse(num_reps: usize, input: Vec<usize>) -> Vec<usize> {
    let hash_len = 256;

    let mut hash_vec = vec![0; hash_len];
    for i in 0..hash_len {
        hash_vec[i] = i;
    }

    let mut curr_pos = 0;
    let mut skip_size = 0;

    for _ in 0..num_reps {
        for num_reverse in &input {
            let num_steps = if (num_reverse % 2) == 0 {
                num_reverse / 2
            } else {
                (num_reverse - 1) / 2
            };

            for i in 0..num_steps {
                let fst = (curr_pos + i) % hash_len;
                let snd = (curr_pos + num_reverse - i - 1) % hash_len;

                // Swap elements
                let tmp = hash_vec[fst];
                hash_vec[fst] = hash_vec[snd];
                hash_vec[snd] = tmp;
            }

            curr_pos = (curr_pos + num_reverse + skip_size) % hash_len;
            skip_size += 1;
        }
    }

    return hash_vec;
}

pub fn knot_hash(contents: String, binary: bool) -> String {
    let mut input_p2: Vec<_> = contents.bytes().map(|v| v as usize).collect();
    let tail: Vec<usize> = vec![17, 31, 73, 47, 23];
    input_p2.extend(&tail);

    let hash_vec_p2 = hash_sparse(64, input_p2);

    let mut hash_dense = vec![0; 16];

    let mut strs = String::new();
    for i in 0..16 {
        let mut comb = hash_vec_p2[i * 16];
        for j in 1..16 {
            let v = hash_vec_p2[i * 16 + j];
            comb = comb ^ v;
        }
        hash_dense[i] = comb;

        let f;
        if binary {
            f = format!("{:08b}", comb);
        } else {
            f = format!("{0:x}", comb);
        }
        strs.push_str(&f);
    }

    return strs;
}

pub fn solve() {
    let mut file = File::open("2017/day10/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input_p1: Vec<_> = contents
        .split(',')
        .map(|s| s.to_string().parse::<usize>().unwrap())
        .collect();

    let hash_vec_p1 = hash_sparse(1, input_p1);
    let hash = knot_hash(contents, false);

    println!("Part one: {:?}", hash_vec_p1[0] * hash_vec_p1[1]);
    println!("Part two: {}", hash);
}
