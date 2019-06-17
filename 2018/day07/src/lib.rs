extern crate common;

const TIME_QUOTE: usize = 60;
const INPUT_FILE: &str = "2018/day07/input";
const NUM_LETTERS: usize = 1 + ('Z' as u8 - 'A' as u8) as usize;

fn solve_time(c: char) -> usize {
    return TIME_QUOTE + 1 + (c as u8 - 'A' as u8) as usize;
}

fn workers_done(workers: &Vec<(char, usize)>) -> bool {
    for (c, _i) in workers {
        if *c != '.' {
            return false;
        }
    }
    return true;
}

fn to_index(c: char) -> usize {
    return (c as u8 - 'A' as u8) as usize;
}

fn solve_input(input: &Vec<Vec<String>>, num_workers: usize) -> (String, usize) {
    let mut depends_on: Vec<Vec<char>> = common::filled_vector(NUM_LETTERS, Vec::new());
    let mut goes_to: Vec<Vec<char>> = common::filled_vector(NUM_LETTERS, Vec::new());
    let mut stack: Vec<char> = Vec::new();

    for line in input {
        let from = line[1].chars().next().unwrap();
        let to = line[7].chars().next().unwrap();

        goes_to[to_index(from)].push(to);
        depends_on[to_index(to)].push(from);
        stack.push(from);
    }

    stack.dedup();
    stack.sort();

    let mut visited: Vec<bool> = common::filled_vector(NUM_LETTERS, false);
    let mut char_done: Vec<bool> = common::filled_vector(NUM_LETTERS, false);
    let mut workers: Vec<(char, usize)> = common::filled_vector(num_workers, ('.', 0));
    let mut result = String::new();
    let mut time = 0;

    while stack.len() > 0 || !workers_done(&workers) {
        time += 1;
        let mut result_curr = Vec::new();

        for i in 0..num_workers {
            let (worked_char, seconds) = workers[i];
            if worked_char != '.' {
                if seconds < solve_time(worked_char) {
                    workers[i].1 += 1;
                    continue;
                }
                char_done[to_index(worked_char)] = true;
                workers[i].0 = '.';
                result_curr.push(worked_char);
            }
        }

        result_curr.sort();
        result += &result_curr.iter().collect::<String>();

        for i in 0..num_workers {
            if workers[i].0 != '.' {
                continue;
            }

            let mut idx = 0;
            while idx < stack.len() {
                let curr = stack[idx];
                let letter_idx = to_index(stack[idx]);
                {
                    let mut open = true;
                    for from in &depends_on[letter_idx] {
                        let from_idx = to_index(*from);
                        open = open && char_done[from_idx] && visited[from_idx];
                    }
                    if !open {
                        idx += 1;
                        continue;
                    }
                }

                stack.remove(idx);
                if visited[letter_idx] {
                    continue;
                }
                visited[letter_idx] = true;

                for to in &goes_to[letter_idx] {
                    stack.push(*to);
                }
                stack.dedup();
                stack.sort();

                workers[i].0 = curr;
                workers[i].1 = 1;
                break;
            }
        }
    }

    return (result, time);
}

pub fn solve() {
    let input = common::read_file(INPUT_FILE)
        .lines()
        .map(|l| l.split_whitespace().map(|v| v.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    println!("Part one: {:?}", solve_input(&input, 1).0);
    println!("Part two: {:?}", solve_input(&input, 5).1);
}
