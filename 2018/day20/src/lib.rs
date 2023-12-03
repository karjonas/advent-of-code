extern crate common;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Action {
    sequence: Vec<Action>,
    step: Vec<char>,
    branch: Vec<Action>,
}

fn is_branch(chars: &Vec<char>) -> bool {
    let mut par_ctr = 0;

    for i in 0..chars.len() {
        if chars[i] == '|' && par_ctr == 0 {
            return true;
        } else if chars[i] == '(' {
            par_ctr += 1;
        } else if chars[i] == ')' {
            par_ctr -= 1;
        }
    }
    return false;
}

fn split_branch(chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut curr_group = Vec::new();
    let mut groups: Vec<Vec<char>> = Vec::new();
    let mut splitted = false;
    let mut par_ctr = 0;
    for i in 0..chars.len() {
        if par_ctr == 0 && !splitted && chars[i] == '|' {
            groups.push(curr_group.clone());
            curr_group.clear();
            splitted = true;
        } else {
            curr_group.push(chars[i]);
        }

        if chars[i] == '(' {
            par_ctr += 1
        } else if chars[i] == ')' {
            par_ctr -= 1
        }
    }

    groups.push(curr_group);
    assert!(groups.len() == 2);

    return groups;
}

fn find_subsections(chars: &Vec<char>) -> Vec<Vec<char>> {
    let mut groups: Vec<Vec<char>> = Vec::new();
    let mut par_ctr = 0;
    let mut curr_group = Vec::new();

    for i in 0..chars.len() {
        if chars[i] == '(' {
            par_ctr += 1;
            // Only store the most shallow groups
            if par_ctr == 1 {
                if !curr_group.is_empty() {
                    groups.push(curr_group.clone());
                    curr_group.clear();
                }
            } else {
                curr_group.push('(');
            }
        } else if chars[i] == ')' {
            par_ctr -= 1;
            if par_ctr == 0 {
                groups.push(curr_group.clone());
                curr_group.clear();
            } else {
                curr_group.push(')');
            }
        } else {
            curr_group.push(chars[i]);
        }
    }

    // Group not in parentheses
    if !curr_group.is_empty() {
        groups.push(curr_group)
    }

    return groups;
}

fn create_grid(map: &HashMap<(i64, i64), char>) -> Vec<Vec<char>> {
    let mut min_x = 666;
    let mut max_x = -666;
    let mut min_y = 666;
    let mut max_y = -666;

    for (k, _v) in map.iter() {
        min_x = std::cmp::min(min_x, k.0);
        max_x = std::cmp::max(max_x, k.0);
        min_y = std::cmp::min(min_y, k.1);
        max_y = std::cmp::max(max_y, k.1);
    }

    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;

    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;

    let mut grid: Vec<Vec<char>> = common::filled_vector(height, common::filled_vector(width, '#'));

    for (k, v) in map.iter() {
        let x = (k.0 - min_x) as usize;
        let y = (k.1 - min_y) as usize;
        grid[y][x] = *v;
    }
    grid[-min_y as usize][-min_x as usize] = 'X';

    return grid;
}

fn build_distances(grid: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let width = grid[0].len();
    let height = grid.len();

    let mut dists: Vec<Vec<usize>> =
        common::filled_vector(height, common::filled_vector(width, std::usize::MAX));

    // Find X
    let mut stack = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'X' {
                stack.push((x, y, 0));
                break;
            }
        }
    }

    while !stack.is_empty() {
        let (x, y, d) = stack.remove(0);

        if d > dists[y][x] {
            continue;
        }

        dists[y][x] = d;

        let up = if y > 0 { grid[y - 1][x] } else { '#' };
        let down = if y + 1 < height { grid[y + 1][x] } else { '#' };
        let left = if x > 0 { grid[y][x - 1] } else { '#' };
        let right = if x + 1 < width { grid[y][x + 1] } else { '#' };

        if up == '-' {
            stack.push((x, y - 2, d + 1));
        }
        if down == '-' {
            stack.push((x, y + 2, d + 1));
        }
        if left == '|' {
            stack.push((x - 2, y, d + 1));
        }
        if right == '|' {
            stack.push((x + 2, y, d + 1));
        }
    }

    return dists;
}

fn print_map(grid: &Vec<Vec<char>>) {
    for line in grid.iter() {
        let s = line.iter().collect::<String>();
        println!("{}", s);
    }
}

fn furthest_doors(dists: &Vec<Vec<usize>>) -> usize {
    let width = dists[0].len();
    let height = dists.len();
    let mut best_dist = 0;

    for y in 0..height {
        for x in 0..width {
            if dists[y][x] != std::usize::MAX && dists[y][x] > best_dist {
                best_dist = dists[y][x];
            }
        }
    }

    return best_dist;
}

fn num_doors_further_than(dists: &Vec<Vec<usize>>, above: usize) -> usize {
    let width = dists[0].len();
    let height = dists.len();
    let mut num = 0;

    for y in 0..height {
        for x in 0..width {
            if dists[y][x] != std::usize::MAX && dists[y][x] >= above {
                num += 1;
            }
        }
    }

    return num;
}

fn parse_recursive(chars_in: &Vec<char>) -> Action {
    let mut chars = chars_in.clone();
    if chars.len() > 0 && chars_in[0] == '(' && chars[chars.len() - 1] == ')' {
        chars.remove(0);
        chars.pop();
    }

    let subs = find_subsections(&chars);

    let mut action = Action {
        sequence: Vec::new(),
        step: Vec::new(),
        branch: Vec::new(),
    };

    if subs.is_empty() {
        return action;
    }

    if is_branch(&chars) {
        let branches = split_branch(&chars);
        action.branch.push(parse_recursive(&branches[0]));
        action.branch.push(parse_recursive(&branches[1]));
    } else if subs.len() > 1 {
        for sub in subs {
            action.sequence.push(parse_recursive(&sub.clone()));
        }
    } else {
        assert!(subs.len() == 1);
        action.step = subs[0].clone();
    }

    return action;
}

fn create_paths(curr_path: &Vec<char>, action: &Action) -> Vec<Vec<char>> {
    let mut ret = Vec::new();

    let num_seqs = action.sequence.len();
    if num_seqs > 0 {
        let mut paths_next = Vec::new();
        paths_next.push(curr_path.clone());

        let mut paths_curr = Vec::new();

        for i in 0..num_seqs {
            for path in paths_next {
                let mut paths = create_paths(&path, &action.sequence[i]);
                paths_curr.append(&mut paths);
            }
            paths_next = paths_curr.clone();
            paths_curr.clear();
        }

        ret.append(&mut paths_next);
    }

    for branch in &action.branch {
        let mut paths = create_paths(curr_path, &branch);
        ret.append(&mut paths);
    }

    if !action.step.is_empty() {
        let mut step = curr_path.clone();
        step.append(&mut action.step.clone());
        ret.push(step)
    }

    return ret;
}

fn solve_paths(paths: Vec<Vec<char>>, map: &mut HashMap<(i64, i64), char>) {
    for path in paths {
        let mut pos = (0 as i64, 0 as i64);
        for c in path {
            if c == 'N' {
                map.insert((pos.0, pos.1 - 1), '-');
                map.insert((pos.0, pos.1 - 2), '.');
                pos = (pos.0, pos.1 - 2);
            } else if c == 'S' {
                map.insert((pos.0, pos.1 + 1), '-');
                map.insert((pos.0, pos.1 + 2), '.');
                pos = (pos.0, pos.1 + 2);
            } else if c == 'W' {
                map.insert((pos.0 - 1, pos.1), '|');
                map.insert((pos.0 - 2, pos.1), '.');
                pos = (pos.0 - 2, pos.1);
            } else if c == 'E' {
                map.insert((pos.0 + 1, pos.1), '|');
                map.insert((pos.0 + 2, pos.1), '.');
                pos = (pos.0 + 2, pos.1);
            }
        }
    }
}

pub fn solve(filepath: &str) {
    let mut chars = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    chars.remove(0);
    chars.pop();

    let action = parse_recursive(&chars);
    let paths = create_paths(&Vec::new(), &action.clone());
    let mut map = HashMap::new();
    solve_paths(paths.clone(), &mut map);
    let grid = create_grid(&map);
    let dists = build_distances(&grid);
    let furthest = furthest_doors(&dists);
    let num_above = num_doors_further_than(&dists, 1000);
    if false {
        print_map(&grid);
    }
    println!("Part one: {}", furthest);
    println!("Part two: {}", num_above);
}
