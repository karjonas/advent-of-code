extern crate common;
extern crate intcode;

fn sum_alignment(input: String) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|value| value.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut sum = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut intersection = true;
            for c in [
                lines[y][x],
                lines[y][x - 1],
                lines[y][x + 1],
                lines[y - 1][x],
                lines[y + 1][x],
            ]
            .iter()
            {
                if !['#', '^', 'v', '<', '>'].contains(c) {
                    intersection = false;
                    break;
                }
            }

            if intersection {
                sum += y * x;
            }
        }
    }

    return sum;
}

fn get_map(memory: Vec<i64>) -> String {
    let (_memory_new, output_numbers, _index, _relative_base, _halted) =
        intcode::run(memory, Vec::new(), 0, 0);
    let mut s = String::new();
    for number in output_numbers {
        s.push(number as u8 as char);
    }
    return s;
}

fn solve_part_one(memory: Vec<i64>) -> usize {
    assert_eq!(memory[0], 1);
    let map = get_map(memory).trim().to_string();
    // println!("{}", map);

    return sum_alignment(map);
}

fn find_robot(map: &Vec<Vec<char>>) -> (usize, usize, char) {
    let width = map[0].len();
    let height = map.len();

    for y in 0..height {
        for x in 0..width {
            if ['<', '>', '^', 'v'].contains(&map[y][x]) {
                return (x, y, map[y][x]);
            }
        }
    }

    panic!("Could not find robot");
}

fn walk_robot(
    map: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir_x: i64,
    dir_y: i64,
) -> (usize, usize, usize) {
    // Walk as far as possible
    let width = map[0].len() as i64;
    let height = map.len() as i64;

    let mut steps = 0;
    let mut x_end = 0;
    let mut y_end = 0;

    let mut x_new = x as i64;
    let mut y_new = y as i64;

    loop {
        x_new = x_new + dir_x;
        y_new = y_new + dir_y;
        if x_new < 0 || x_new >= width || y_new < 0 || y_new >= height {
            // Out of bounds
            break;
        }

        if map[y_new as usize][x_new as usize] == '#' {
            // walkable
            x_end = x_new as usize;
            y_end = y_new as usize;
            steps += 1;
        } else {
            // outside trail
            break;
        }
    }

    return (x_end, y_end, steps);
}

fn find_path(input: String) -> Vec<i64> {
    let map: Vec<Vec<char>> = input.lines().map(|value| value.chars().collect()).collect();

    let (mut rob_x, mut rob_y, mut rob_dir) = find_robot(&map);

    let mut steps = Vec::new();

    loop {
        let dirs;
        if rob_dir == '^' {
            dirs = [(-1, 0, '<', 'L'), (1, 0, '>', 'R')].to_vec();
        } else if rob_dir == '<' {
            dirs = [(0, 1, 'v', 'L'), (0, -1, '^', 'R')].to_vec();
        } else if rob_dir == 'v' {
            dirs = [(-1, 0, '<', 'R'), (1, 0, '>', 'L')].to_vec();
        } else if rob_dir == '>' {
            dirs = [(0, 1, 'v', 'R'), (0, -1, '^', 'L')].to_vec();
        } else {
            panic!("invalid dir");
        }

        let mut stuck = true;
        for (dir_x, dir_y, dir_done, step_char) in dirs.clone() {
            let (move_x, move_y, num_steps) = walk_robot(&map, rob_x, rob_y, dir_x, dir_y);
            if num_steps > 0 {
                rob_dir = dir_done;
                rob_x = move_x;
                rob_y = move_y;
                stuck = false;
                steps.push(step_char as i64);
                steps.push(num_steps as i64);
                break;
            }
        }

        if stuck {
            break;
        }
    }

    return steps;
}

fn to_offset(i: i64) -> char {
    let c = i as u8;
    return if c < 'A' as u8 {
        ('a' as u8 + c) as char
    } else {
        c as char
    };
}

fn from_offset(c: char) -> i64 {
    return if c > 'a' {
        (c as u8 - 'a' as u8) as i64
    } else {
        c as i64
    };
}

fn from_offset_str(s: &String) -> Vec<i64> {
    return s.chars().map(|v| from_offset(v)).collect();
}

fn inject_commas(v: &Vec<i64>) -> Vec<i64> {
    let mut idx = 0;
    let mut result = v.clone();
    while idx + 1 < result.len() {
        result.insert(idx + 1, ',' as u8 as i64);
        idx += 2;
    }

    // Ugly hack to convert numbers to two digit chars
    let mut result_str = String::new();
    for c in result {
        if c < 20 {
            result_str = result_str + &c.to_string();
        } else {
            result_str.push(c as u8 as char);
        }
    }
    result_str.push('\n');
    return result_str.chars().map(|c| c as u8 as i64).collect();
}

fn compress_paths(path: &Vec<i64>) -> (Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>) {
    let path_str: String = path.iter().map(|i| to_offset(*i)).collect();

    const MAX_LEN: usize = 10;
    let mut best_len = 0;
    let mut str_best = [String::new(), String::new(), String::new()];
    let mut strs = [String::new(), String::new(), String::new()];
    let mut strs_done = [String::new(), String::new(), String::new()];
    let mut max_len = [MAX_LEN, MAX_LEN, MAX_LEN];

    for a_len in 1..max_len[0] {
        strs[0] = path_str.chars().take(a_len).collect();
        strs_done[0] = path_str.replace(strs[0].as_str(), "");

        max_len[1] = std::cmp::min(strs_done[0].len(), MAX_LEN);
        for b_len in 1..max_len[1] {
            strs[1] = strs_done[0].chars().take(b_len).collect();
            strs_done[1] = strs_done[0].replace(strs[1].as_str(), "");

            max_len[2] = std::cmp::min(strs_done[1].len(), MAX_LEN);
            for c_len in 1..max_len[2] {
                strs[2] = strs_done[1].chars().take(c_len).collect();
                strs_done[2] = strs_done[1].replace(strs[2].as_str(), "");

                if strs_done[2].is_empty()
                    && !strs[0].is_empty()
                    && !strs[1].is_empty()
                    && !strs[2].is_empty()
                {
                    let compressed = path_str
                        .replace(str_best[0].as_str(), "A")
                        .replace(str_best[1].as_str(), "B")
                        .replace(str_best[2].as_str(), "C");
                    if best_len <= compressed.len() {
                        best_len = compressed.len();
                        str_best[0] = strs[0].clone();
                        str_best[1] = strs[1].clone();
                        str_best[2] = strs[2].clone();
                    }
                }
            }
        }
    }

    let compressed = path_str
        .replace(str_best[0].as_str(), "A")
        .replace(str_best[1].as_str(), "B")
        .replace(str_best[2].as_str(), "C");

    return (
        inject_commas(&from_offset_str(&compressed)),
        inject_commas(&from_offset_str(&str_best[0])),
        inject_commas(&from_offset_str(&str_best[1])),
        inject_commas(&from_offset_str(&str_best[2])),
    );
}

fn solve_part_two(memory_in: Vec<i64>) -> usize {
    let map = get_map(memory_in.clone()).trim().to_string();
    let path = find_path(map);
    let (all, a, b, c) = compress_paths(&path);

    let mut memory = memory_in.clone();
    memory[0] = 2;

    let inputs = [all, a, b, c, ['n' as u8 as i64, '\n' as u8 as i64].to_vec()].to_vec();
    let mut index = 0;
    let mut relative_base = 0;
    let mut last_output = 0;

    for input in inputs {
        let (memory_new, output_numbers, index_new, relative_base_new, _halted) =
            intcode::run(memory.clone(), input.clone(), index, relative_base);

        index = index_new;
        relative_base = relative_base_new;
        memory = memory_new;

        last_output = *output_numbers.last().unwrap() as usize;
    }

    return last_output;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone()));
    println!("Part two: {}", solve_part_two(memory));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_a() {
        let input = "..#..........\n..#..........\n#######...###\n#.#...#...#.#\n#############\n..#...#...#..\n..#####...^..";
        assert_eq!(sum_alignment(String::from(input)), 76);
    }
}
