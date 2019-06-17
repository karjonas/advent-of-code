extern crate common;

const INPUT: [i8; 10] = [1, 3, 2, 1, 1, 3, 1, 1, 1, 2];

fn next_sequence(in_seq: &[i8]) -> Vec<i8> {
    assert!(!in_seq.is_empty());

    let mut result = Vec::new();
    let mut current_number = in_seq[0];
    let mut current_runlength = 1;

    for i in &in_seq[1..] {
        if current_number == *i {
            current_runlength += 1;
        } else {
            result.push(current_runlength);
            result.push(current_number);
            current_runlength = 1;
            current_number = *i;
        }
    }
    result.push(current_runlength);
    result.push(current_number);
    result
}

pub fn solve() {
    let mut seq = INPUT.to_vec();
    for _i in 0..40 {
        seq = next_sequence(&seq);
    }

    println!("Part one: {}", seq.len());

    for _i in 0..10 {
        seq = next_sequence(&seq);
    }

    println!("Part two: {}", seq.len());
}
