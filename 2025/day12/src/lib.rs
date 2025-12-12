use std::convert::TryInto;

fn parse_input(input: &str) -> (Vec<usize>, Vec<((usize, usize), [usize;6])>) {
    let lines : Vec<&str> = input.lines().collect();
    let mut boxes : Vec<usize> = Vec::new();
    let mut dims_numbers : Vec<((usize, usize), [usize;6])> = Vec::new();

    for box_i in 0..6 {
        let offset = box_i * 5;
        let mut curr_box  = 0;
        for i in 0..3 {
            let line : Vec<char> = lines[offset + i + 1].chars().collect();
            curr_box += (line[0] == '#') as usize;
            curr_box += (line[1] == '#') as usize;
            curr_box += (line[2] == '#') as usize;
        }
        boxes.push(curr_box);
    }

    for i in 30..lines.len() {
        // Split at ':' -> ["45x44", "36 25 41 45 29 33"]
        let line: &str = lines[i].trim();
        let (dims, nums) = line.split_once(": ").unwrap();

        let dims: Vec<usize> = dims
            .split('x')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        let numbers: Vec<usize> = nums
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let numbers_array : [usize; 6] = numbers.try_into().unwrap();

        dims_numbers.push(((dims[0], dims[1]), numbers_array));
    }

    (boxes, dims_numbers)
}

fn part_one(input: (Vec<usize>, Vec<((usize, usize), [usize;6])>)) -> usize {
    let boxes = input.0.clone();
    let dims_numbers = input.1.clone();

    let mut count = 0;
    for ((x,y), num_needed) in &dims_numbers {
        let mut total_needed = 0;
        let area = x * y;
        for i in 0..num_needed.len() {
            let box_i = boxes[i];
            let need_i = num_needed[i];
            total_needed += box_i * need_i;
        }
        if total_needed < area {
            count += 1;
        }
    }

    count
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let machines = parse_input(&input);
    println!("Part one: {}", part_one(machines));
}
