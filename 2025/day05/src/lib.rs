fn parse_input(input: &String, ranges: &mut Vec<(i64, i64)>, numbers: &mut Vec<i64>) {
    for line in input.lines() {
        if line.contains('-') {
            let mut splits = line.split('-').take(2);
            let first = splits.next().unwrap().parse::<i64>().unwrap();
            let last = splits.next().unwrap().parse::<i64>().unwrap();
            ranges.push((first, last));
        } else if !line.is_empty() {
            let number = line.parse::<i64>().unwrap();
            numbers.push(number);
        }
    }

    let mut keep_merging = true;
    while keep_merging {
        keep_merging = false;
        for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                let (min_i, max_i) = ranges[i];
                let (min_j, max_j) = ranges[j];

                let i_in_j = min_i >= min_j && max_i <= max_j;
                let j_in_i = min_j >= min_i && max_j <= max_i;
                let overlaps_i = max_i >= min_j && max_i <= max_j;
                let overlaps_j = max_j >= min_i && max_j <= max_i;
                if i_in_j || j_in_i || overlaps_i || overlaps_j {
                    ranges[i] = (std::cmp::min(min_i, min_j), std::cmp::max(max_i, max_j));
                    ranges.remove(j);
                    keep_merging = true;
                    break;
                }
            }
            if keep_merging {
                break;
            }
        }
    }
}

fn part_one(ranges: &Vec<(i64, i64)>, numbers: &Vec<i64>) -> usize {
    let mut result = 0;
    for number in numbers {
        for (first, last) in ranges {
            if number >= first && number <= last {
                result += 1;
                break;
            }
        }
    }
    result
}

fn part_two(ranges: &Vec<(i64, i64)>) -> usize {
    let mut sum = 0;
    for (min, max) in ranges {
        sum += 1 + max - min;
    }
    sum as usize
}

pub fn solve(filepath: &str) {
    let input = &std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut numbers: Vec<i64> = Vec::new();
    parse_input(input, &mut ranges, &mut numbers);

    println!("Part one: {}", part_one(&ranges, &numbers));
    println!("Part two: {}", part_two(&ranges));
}
