use std::collections::HashMap;
use std::mem;

enum Instruction {
    Rotate(usize),
    SwapIndex(usize, usize),
    SwapChars(char, char),
}

fn vec_to_usize(input: &Vec<char>) -> usize {
    assert!(input.len() == 16);
    let mut output: usize = 0;

    for i in 0..16 {
        let v = (input[i] as u8 - 'a' as u8) as usize;
        let v0 = v << (i * 4);
        output = output | v0;
    }

    return output;
}

fn usize_to_vec(input: usize) -> Vec<char> {
    let mut output = vec!['0'; 16];

    for i in 0..16 {
        let lower_bits = (input >> 4 * i) & 0b1111;
        let v = (lower_bits as u8 + 'a' as u8) as char;
        output[i] = v;
    }
    return output;
}

fn collect_instructions(cmds: Vec<String>) -> Vec<Instruction> {
    let mut instrs = Vec::<Instruction>::new();

    for mut cmd in cmds.clone() {
        let action = cmd.remove(0);
        match action {
            's' => {
                let num = cmd.parse::<usize>().unwrap();
                let inst = Instruction::Rotate(num);
                instrs.push(inst);
            }
            'x' => {
                let args: Vec<_> = cmd.split('/').collect();
                let arg0 = args[0].parse::<usize>().unwrap();
                let arg1 = args[1].parse::<usize>().unwrap();

                let inst = Instruction::SwapIndex(arg0, arg1);
                instrs.push(inst);
            }
            'p' => {
                let chrs: Vec<char> = cmd.chars().collect();
                let arg0 = chrs[0];
                let arg1 = chrs[2];

                let inst = Instruction::SwapChars(arg0, arg1);
                instrs.push(inst);
            }
            _ => {
                panic!("No match");
            }
        }
    }

    return instrs;
}

fn calc_hash(instrs: &Vec<Instruction>, reps: usize) -> String {
    let queue_base = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    let mut queue = queue_base.clone();

    let queue_len = queue.len();

    let mut queue_old = queue.clone();

    let mut visited: HashMap<usize, usize> = HashMap::new();

    let mut qhash = vec_to_usize(&queue);

    let mut ctr = 0;
    loop {
        if visited.contains_key(&qhash) {
            break;
        }

        queue = usize_to_vec(qhash);

        let qhash_prev = qhash;

        for inst in instrs {
            match inst {
                &Instruction::Rotate(num) => {
                    mem::swap(&mut queue_old, &mut queue);
                    for i in 0..queue_len {
                        queue[(i + num) % queue_len] = queue_old[i];
                    }
                }
                &Instruction::SwapIndex(arg0, arg1) => {
                    queue.swap(arg0, arg1);
                }
                &Instruction::SwapChars(arg0, arg1) => {
                    let i0 = queue.iter().position(|&r| r == arg0).unwrap();
                    let i1 = queue.iter().position(|&r| r == arg1).unwrap();

                    queue.swap(i0, i1);
                }
            }
        }

        qhash = vec_to_usize(&queue);
        visited.insert(qhash_prev, qhash);
        ctr += 1;
    }

    qhash = vec_to_usize(&queue_base);

    for _ in 0..(reps % ctr) {
        match visited.get(&qhash) {
            Some(next_value) => {
                qhash = next_value.clone();
            }
            None => {}
        };
    }

    queue = usize_to_vec(qhash);

    let s: String = queue.iter().collect();

    return s;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let cmds: Vec<_> = input.split(',').map(|v| v.to_string()).collect();

    let instrs = collect_instructions(cmds);

    println!("Part one: {}", calc_hash(&instrs, 1));
    println!("Part two: {}", calc_hash(&instrs, 1000000000));
}
