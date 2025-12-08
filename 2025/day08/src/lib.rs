fn parse_input(input: &String) -> Vec<(f64, f64, f64)> {
    let mut vec3s: Vec<(f64, f64, f64)> = Vec::new();
    for line in input.lines() {
        let mut splits = line.split(',');
        let f0 = splits.next().unwrap().parse::<f64>().unwrap();
        let f1 = splits.next().unwrap().parse::<f64>().unwrap();
        let f2 = splits.next().unwrap().parse::<f64>().unwrap();
        vec3s.push((f0, f1, f2));
    }
    return vec3s;
}

fn part_one_two(vec3s: Vec<(f64, f64, f64)>) -> (u32, usize) {
    let n = vec3s.len();
    let mut shortest_distances = Vec::new();
    for i in 0..n {
        let v_i = vec3s[i];
        for j in i + 1..n {
            let v_j = vec3s[j];
            let d_x = v_i.0 - v_j.0;
            let d_y = v_i.1 - v_j.1;
            let d_z = v_i.2 - v_j.2;
            let dist = (d_x * d_x + d_y * d_y + d_z * d_z).sqrt();
            shortest_distances.push((i, j, dist));
        }
    }

    shortest_distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut circuit = vec![u32::MAX; n];
    let mut curr_circuit: u32 = 0;
    let mut part_one = 0;
    let mut part_two = 0;
    for iter in 0..shortest_distances.len() {
        // part one
        if iter == 1000 {
            let mut circuit_sizes = vec![0; curr_circuit as usize];
            for i in 0..curr_circuit {
                let sum = circuit
                    .iter()
                    .fold(0, |acc, &item| acc + (item == i) as u32);
                circuit_sizes[i as usize] = sum;
            }

            circuit_sizes.sort();
            circuit_sizes.reverse();
            part_one = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
        }

        let (i, j, _dist) = shortest_distances[iter];
        if circuit[i] != u32::MAX && circuit[j] != u32::MAX && circuit[i] == circuit[j] {
            continue;
        }

        let cirtcuit_idx = std::cmp::min(std::cmp::min(circuit[i], circuit[j]), curr_circuit);
        let cirtcuit_old = std::cmp::max(circuit[i], circuit[j]);
        if cirtcuit_old != u32::MAX {
            for k in 0..n {
                if circuit[k] == cirtcuit_old {
                    circuit[k] = cirtcuit_idx;
                }
            }
        }

        circuit[i] = cirtcuit_idx;
        circuit[j] = cirtcuit_idx;

        if cirtcuit_idx == curr_circuit {
            curr_circuit += 1;
        }

        // part two
        let mut done = true;
        for i in 0..n {
            if circuit[i] != 0 {
                done = false;
                break;
            }
        }

        if done {
            part_two = (vec3s[i].0 * vec3s[j].0) as usize;
        }
    }

    (part_one, part_two)
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let vec3s = parse_input(&input);
    let (p1, p2) = part_one_two(vec3s);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}
