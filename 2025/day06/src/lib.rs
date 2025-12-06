fn parse_input_p1(input: &String) -> Vec<(Vec<usize>, char)> {
    let mut result = Vec::new();
    let mut grid: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        grid.push(line.split_ascii_whitespace().collect::<Vec<&str>>());
    }

    for c in 0..grid[0].len() {
        let mut row = Vec::new();
        for r in 0..grid.len() - 1 {
            row.push(grid[r][c].to_string().parse::<usize>().unwrap());
        }
        let operator = grid[grid.len() - 1][c].chars().next().unwrap();
        result.push((row, operator));
    }

    return result;
}

fn parse_input_p2(input: &String) -> Vec<(Vec<usize>, char)> {
    let mut result = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    // put operator first
    grid.insert(0, grid[grid.len() - 1].clone());
    grid.pop();

    let mut curr_numbers: Vec<usize> = Vec::new();
    let mut operator = '*';
    for col in 0..grid[0].len() {
        let c = grid[0][col];
        if c == '*' || c == '+' {
            // operator, done
            if !curr_numbers.is_empty() {
                result.push((curr_numbers.clone(), operator));
            }
            curr_numbers.clear();
            operator = c;
        }
        // number
        let mut number = 0;
        for row in 1..grid.len() {
            let digit = grid[row][col];
            if digit.is_ascii_whitespace() {
                continue;
            }
            number = number * 10 + digit as usize - 48;
        }
        if number != 0 {
            curr_numbers.push(number);
        }
    }

    if !curr_numbers.is_empty() {
        result.push((curr_numbers.clone(), operator));
    }

    return result;
}

fn part_one_two(ranges: &Vec<(Vec<usize>, char)>) -> usize {
    let mut result = 0;
    for (numbers, operator) in ranges {
        let mut result_range = numbers[0];
        for i in 1..numbers.len() {
            if *operator == '*' {
                result_range *= numbers[i];
            } else if *operator == '+' {
                result_range += numbers[i];
            }
        }
        result += result_range;
    }
    result
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let ranges_p1 = parse_input_p1(&input);
    let ranges_p2 = parse_input_p2(&input);
    println!("Part one: {}", part_one_two(&ranges_p1));
    println!("Part two: {}", part_one_two(&ranges_p2));
}
