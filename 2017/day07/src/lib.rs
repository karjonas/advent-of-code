use std::collections::HashMap;

fn calc_weights_recursive(
    root: String,
    children: &HashMap<String, Vec<String>>,
    mut weights_combined: &mut HashMap<String, usize>,
    weights: &HashMap<String, usize>,
    mut black_sheep: &mut String,
) -> usize {
    let my_children = children.get(&root).unwrap();

    if my_children.is_empty() {
        return weights.get(&root).unwrap().clone();
    } else {
        let mut acc = weights.get(&root).unwrap().clone();
        let mut done_vals = HashMap::new();

        for child in my_children {
            let val = calc_weights_recursive(
                child.clone(),
                &children,
                &mut weights_combined,
                &weights,
                &mut black_sheep,
            );
            acc += val;
            let count = done_vals.entry(val).or_insert(0);
            *count += 1;
        }

        if done_vals.len() > 1 && black_sheep.is_empty() {
            let mut goal_v = 0;
            for (k, v) in done_vals {
                if v == 1 {
                    for child in my_children {
                        let weight_child = weights_combined.get(child).unwrap();
                        if weight_child.clone() == k {
                            *black_sheep = child.clone();
                        }
                    }
                } else {
                    goal_v = k.clone();
                }
            }

            assert!(goal_v != 0);
            let weight_black_sheep = weights.get(black_sheep).unwrap().clone();
            let weight_black_sheep_only_children =
                weights_combined.get(black_sheep).unwrap().clone() - weight_black_sheep;
            let solve_weight = goal_v - weight_black_sheep_only_children;
            println!("Part two: {}", solve_weight);
        }

        weights_combined.insert(root, acc);
        return acc;
    }
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let contents: String = input
        .chars()
        .filter(|v| v.is_alphanumeric() || v.clone() == '\n' || v.clone() == ' ')
        .collect();
    let lines: Vec<Vec<_>> = contents
        .split('\n')
        .map(|line| line.split(' ').collect())
        .collect();

    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    let mut weights = HashMap::new();
    let mut weights_combined = HashMap::new();

    for line in &lines {
        let n = line.len();
        let name = line[0].to_string();
        let weight = line[1].parse::<usize>().unwrap();
        let ref mut my_children = children.entry(name.clone()).or_insert(Vec::<String>::new());
        weights.insert(name.clone(), weight);

        if n == 2 {
            weights_combined.insert(name.clone(), weight);
        } else {
            weights_combined.insert(name.clone(), 0);
        }

        for i in 3..n {
            let child = line[i];
            parents.insert(child, name.clone());
            my_children.push(child.to_string());
        }
    }

    let mut bottom_name = "";
    for line in &lines {
        let name = line[0];
        if !parents.contains_key(name) {
            bottom_name = name;
            break;
        }
    }

    println!("Part one: {}", bottom_name);

    let mut black_sheep = <String>::new();

    calc_weights_recursive(
        bottom_name.to_string(),
        &children,
        &mut weights_combined,
        &weights,
        &mut black_sheep,
    );
}
