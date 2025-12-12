use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let mut is_first = true;
        let mut from = String::new();
        let mut to: Vec<String> = Vec::new();

        for part_str in line.split_ascii_whitespace() {
            let name: String = part_str.chars().take(3).collect();
            if is_first {
                from = name;
                is_first = false;
            } else {
                to.push(name);
            }
        }

        result.insert(from, to);
    }

    result
}

fn count_paths(
    curr: &String,
    target: &String,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if curr == target {
        return 1;
    }

    if let Some(&count) = memo.get(curr) {
        return count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(curr) {
        for next_node in neighbors {
            total_paths += count_paths(next_node, target, graph, memo);
        }
    }

    memo.insert(curr.clone(), total_paths);
    total_paths
}

fn part_one(graph: &HashMap<String, Vec<String>>) -> usize {
    count_paths(
        &"you".to_string(),
        &"out".to_string(),
        graph,
        &mut HashMap::new(),
    )
}

fn part_two(graph: &HashMap<String, Vec<String>>) -> usize {
    let num_svr_fft = count_paths(
        &"svr".to_string(),
        &"fft".to_string(),
        graph,
        &mut HashMap::new(),
    );
    let num_fft_dac = count_paths(
        &"fft".to_string(),
        &"dac".to_string(),
        graph,
        &mut HashMap::new(),
    );
    let num_dac_out = count_paths(
        &"dac".to_string(),
        &"out".to_string(),
        graph,
        &mut HashMap::new(),
    );

    let num_svr_dac = count_paths(
        &"svr".to_string(),
        &"dac".to_string(),
        graph,
        &mut HashMap::new(),
    );
    let num_dac_fft = count_paths(
        &"dac".to_string(),
        &"fft".to_string(),
        graph,
        &mut HashMap::new(),
    );
    let num_fft_out = count_paths(
        &"fft".to_string(),
        &"out".to_string(),
        graph,
        &mut HashMap::new(),
    );

    let num_svr_fft_dac_out = num_svr_fft * num_fft_dac * num_dac_out;
    let num_svr_dac_fft_out = num_svr_dac * num_dac_fft * num_fft_out;

    num_svr_fft_dac_out + num_svr_dac_fft_out
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let machines = parse_input(&input);
    println!("Part one: {}", part_one(&machines));
    println!("Part two: {}", part_two(&machines));
}
