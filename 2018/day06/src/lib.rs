extern crate common;

fn grid_idx(x: usize, y: usize, comp_len: usize) -> usize {
    return comp_len * y + x;
}

fn diff(a: usize, b: usize) -> usize {
    return std::cmp::max(a, b) - std::cmp::min(a, b);
}

fn part_one(vals: Vec<Vec<usize>>) -> usize {
    let mut max_dist = 0;
    for v in &vals {
        max_dist = std::cmp::max(max_dist, std::cmp::max(v[0], v[1]));
    }

    let comp_len = max_dist + 1;
    let grid_size = comp_len * comp_len;
    let mut grid_dist: Vec<usize> = common::filled_vector(grid_size, 9999999);
    let mut grid_closest_pt: Vec<usize> = common::filled_vector(grid_size, 9999999);
    let mut grid_tie: Vec<bool> = common::filled_vector(grid_size, false);

    // Fill grids with closest distances et c.
    let mut v_idx = 0;
    for v in &vals {
        let v_x = v[0];
        let v_y = v[1];

        for x in 0..comp_len {
            for y in 0..comp_len {
                let idx = grid_idx(x, y, comp_len);
                let dist = diff(v_x, x) + diff(v_y, y);
                if grid_dist[idx] == dist {
                    grid_tie[idx] = true;
                } else if dist < grid_dist[idx] {
                    grid_tie[idx] = false;
                    grid_dist[idx] = dist;
                    grid_closest_pt[idx] = v_idx;
                }
            }
        }
        v_idx += 1;
    }

    // Mark values with infinite area
    let mut value_inf: Vec<bool> = common::filled_vector(vals.len(), false);
    for x in 0..comp_len {
        let idx_up = grid_idx(x, 0, comp_len);
        let idx_down = grid_idx(x, comp_len - 1, comp_len);

        if !grid_tie[idx_up] {
            value_inf[grid_closest_pt[idx_up]] = true;
        }
        if !grid_tie[idx_down] {
            value_inf[grid_closest_pt[idx_down]] = true;
        }
    }

    for y in 0..comp_len {
        let idx_up = grid_idx(0, y, comp_len);
        let idx_down = grid_idx(comp_len - 1, y, comp_len);
        if !grid_tie[idx_up] {
            value_inf[grid_closest_pt[idx_up]] = true;
        }
        if !grid_tie[idx_down] {
            value_inf[grid_closest_pt[idx_down]] = true;
        }
    }

    // Find largest area
    let mut largest = 0;
    for idx in 0..vals.len() {
        if value_inf[idx] {
            continue;
        }

        let mut my_val = 0;

        for best in &grid_closest_pt {
            if best.clone() == idx {
                my_val += 1;
            }
        }
        if my_val > largest {
            largest = my_val;
        }
    }

    return largest;
}

fn part_two(vals: Vec<Vec<usize>>) -> usize {
    let search_area = 10000;
    let comp_len = search_area;

    let mut num_inside = 0;
    for x in 0..comp_len {
        for y in 0..comp_len {
            let mut dist_sum = 0;
            for v in &vals {
                let v_x = v[0];
                let v_y = v[1];
                let dist = diff(v_x, x) + diff(v_y, y);
                dist_sum += dist;
                if dist_sum > search_area {
                    break;
                }
            }

            if dist_sum < search_area {
                num_inside += 1;
            }
        }
    }

    return num_inside;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string()
        .chars()
        .filter(|&c| c != ',')
        .collect::<String>();
    let vals = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    println!("Part one: {:?}", part_one(vals.clone()));
    println!("Part two: {:?}", part_two(vals));
}
