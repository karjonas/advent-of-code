extern crate common;

#[derive(Clone, Debug)]
struct Unit {
    x: usize,
    y: usize,
    is_goblin: bool,
    health: usize,
    alive: bool,
    attack: usize,
}

fn sort_units(units: &mut Vec<Unit>) {
    units.sort_by(|a, b| {
        if a.y == b.y {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });
}

fn are_units_sorted(a: &Unit, b: &Unit) -> bool {
    return if a.y == b.y { a.x < b.x } else { a.y < b.y };
}

fn read_input(input: &String) -> (Vec<Vec<char>>, Vec<Unit>) {
    let mut board = Vec::new();
    let mut units = Vec::new();

    let mut y = 0;
    for line in input.lines() {
        let mut line_chars = line.chars().collect::<Vec<_>>();
        for x in 0..line_chars.len() {
            let c = line_chars[x].clone();
            if c == 'G' || c == 'E' {
                let is_goblin = c == 'G';
                units.push(Unit {
                    x: x,
                    y: y,
                    is_goblin: is_goblin,
                    health: 200,
                    alive: true,
                    attack: 3,
                });
                line_chars[x] = '.';
            }
        }
        board.push(line_chars);
        y += 1;
    }

    return (board, units);
}

fn print_board(board: &Vec<Vec<char>>, units_in: &Vec<Unit>) {
    let mut board_copy = board.clone();
    let mut units = units_in.clone();
    sort_units(&mut units);

    for unit in &units {
        if unit.alive {
            board_copy[unit.y][unit.x] = if unit.is_goblin { 'G' } else { 'E' }
        }
    }

    let mut y = 0;
    for line in board_copy {
        let line_str = line.iter().collect::<String>();
        let mut unit_strs: Vec<String> = Vec::new();

        for unit in &units {
            if unit.alive && unit.y == y {
                let c = if unit.is_goblin { "G" } else { "E" };
                let h = unit.health.to_string();
                let unit_str = c.to_string() + "(" + &h + ")";
                unit_strs.push(unit_str);
            }
        }

        let unit_str = unit_strs.join(", ");
        println!("{}", line_str + "  " + &unit_str);

        y += 1;
    }
}

fn find_targets(unit_idx: usize, units: &Vec<Unit>) -> Vec<usize> {
    let mut output = Vec::new();
    let is_goblin = units[unit_idx].is_goblin;
    for i in 0..units.len() {
        if units[i].alive && is_goblin != units[i].is_goblin {
            output.push(i)
        }
    }
    return output;
}

fn units_adjacent(a: &Unit, b: &Unit) -> bool {
    return (a.x + 1 == b.x && a.y == b.y)
        || (a.x == b.x + 1 && a.y == b.y)
        || (a.y + 1 == b.y && a.x == b.x)
        || (a.y == b.y + 1 && a.x == b.x);
}

fn get_adjacent_unit_indices(
    unit_idx: usize,
    target_indices: &Vec<usize>,
    units: &Vec<Unit>,
) -> Vec<usize> {
    let mut res = Vec::new();
    let unit_src = units[unit_idx].clone();
    for i in target_indices {
        let unit_dst = units[*i].clone();
        if units_adjacent(&unit_src, &unit_dst) {
            res.push(*i);
        }
    }
    return res;
}

fn has_adjacent_units(unit_idx: usize, target_indices: &Vec<usize>, units: &Vec<Unit>) -> bool {
    return !get_adjacent_unit_indices(unit_idx, target_indices, units).is_empty();
}

fn attack(unit_idx: usize, target_indices: &Vec<usize>, units: &mut Vec<Unit>) {
    let adj_units = get_adjacent_unit_indices(unit_idx, target_indices, units);
    let mut best_idx = adj_units[0];
    for i in adj_units {
        if units[i].health < units[best_idx].health {
            best_idx = i;
        } else if units[i].health == units[best_idx].health
            && are_units_sorted(&units[i], &units[best_idx])
        {
            best_idx = i;
        }
    }

    assert!(units[best_idx].alive);
    units[best_idx].health -= std::cmp::min(units[best_idx].health, units[unit_idx].attack);
    units[best_idx].alive = units[best_idx].health > 0;
}

fn get_adjacent(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();

    if y > 0 {
        res.push((x, y - 1));
    }
    if x > 0 {
        res.push((x - 1, y));
    }
    if x + 1 < width {
        res.push((x + 1, y));
    }
    if y + 1 < height {
        res.push((x, y + 1));
    }
    return res;
}

fn build_distances(
    pos: (usize, usize),
    board: &Vec<Vec<char>>,
    units: &Vec<Unit>,
) -> Vec<Vec<i32>> {
    const EMPTY: i32 = -1;
    const FULL: i32 = -2;

    let width = board[0].len();
    let height = board.len();

    let mut dist = common::filled_vector(height, common::filled_vector(width, EMPTY as i32));

    for y in 0..height {
        for x in 0..width {
            dist[y][x] = if board[y][x] == '#' { FULL } else { EMPTY };
        }
    }

    for unit in units {
        if unit.alive {
            dist[unit.y][unit.x] = FULL;
        }
    }

    // Set this unit position as open
    dist[pos.1][pos.0] = EMPTY;

    let mut stack: Vec<(usize, usize, usize)> = Vec::new();
    stack.push((pos.0, pos.1, 0));
    let last_val = 0;
    while !stack.is_empty() {
        let (x, y, val) = stack.remove(0);
        assert!(last_val <= val);
        if dist[y][x] >= 0 || dist[y][x] == FULL {
            continue;
        }
        dist[y][x] = val as i32;
        let adjacents = get_adjacent(x, y, width, height);

        for (x_adj, y_adj) in adjacents {
            stack.push((x_adj, y_adj, val + 1));
        }
    }

    return dist;
}

fn get_adjacent_positions(
    board: &Vec<Vec<char>>,
    target_indices: &Vec<usize>,
    units: &Vec<Unit>,
) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let width = board[0].len();
    let height = board.len();

    for idx in target_indices {
        let unit = &units[*idx];
        let mut adjs = get_adjacent(unit.x, unit.y, width, height);
        ret.append(&mut adjs);
    }

    return ret;
}

fn prune_invalid_positions(
    dist: &Vec<Vec<i32>>,
    adjs: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for (x, y) in adjs {
        if dist[*y][*x] >= 0 {
            result.push((*x, *y));
        }
    }
    return result;
}

fn find_closest_pos(dist: &Vec<Vec<i32>>, adjs: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut best = (666, 666);
    let mut best_dist = 666;

    for (x, y) in adjs {
        if dist[*y][*x] == best_dist {
            if *y < best.1 || (*y == best.1 && *x < best.0) {
                best = (*x, *y);
                best_dist = dist[*y][*x];
            }
        } else if dist[*y][*x] < best_dist && dist[*y][*x] >= 0 {
            best = (*x, *y);
            best_dist = dist[*y][*x];
        }
    }
    assert!(best != (666, 666));
    return best;
}

fn do_step(board: &Vec<Vec<char>>, units: &mut Vec<Unit>) -> bool {
    sort_units(units);

    let width = board[0].len();
    let height = board.len();

    for i in 0..units.len() {
        if !units[i].alive {
            continue;
        }

        let targets = find_targets(i, &units);
        // Round ends prematurely
        if targets.is_empty() {
            return false;
        }

        if has_adjacent_units(i, &targets, &units) {
            attack(i, &targets, units);
        } else {
            let dist = build_distances((units[i].x, units[i].y), board, &units);
            let mut adjs = get_adjacent_positions(board, &targets, &units);
            adjs = prune_invalid_positions(&dist, &adjs);
            if adjs.is_empty() {
                continue;
            }

            assert!(!adjs.is_empty());

            // Choose closest pos in adjs
            let selected = find_closest_pos(&dist, &adjs);
            // Use inverse djikstra to find closest path
            let dist_inv = build_distances(selected, board, &units);

            let immediate_adjs = get_adjacent(units[i].x, units[i].y, width, height);
            let next_move = find_closest_pos(&dist_inv, &immediate_adjs);
            units[i].x = next_move.0;
            units[i].y = next_move.1;

            if has_adjacent_units(i, &targets, &units) {
                attack(i, &targets, units);
            }

            //  print_board(board, units);
        }
    }
    return true;
}

fn part_one(board: &Vec<Vec<char>>, units: &mut Vec<Unit>) -> String {
    let mut i = 0;
    loop {
        if false {
            println!("Step {}", i);
            print_board(&board, &units);
        }

        if !do_step(&board, units) {
            // Count health
            let health_rem = units.iter().fold(0, |sum, u| sum + u.health);
            return (health_rem * i).to_string();
        }

        i += 1;
    }
}

fn part_two(board: &Vec<Vec<char>>, units_in: &mut Vec<Unit>) -> String {
    let mut attack = 4;
    loop {
        let mut units = units_in.clone();
        for i in 0..units.len() {
            if !units[i].is_goblin {
                units[i].attack = attack;
            }
        }

        let outcome = part_one(board, &mut units);

        let elves_won = units.iter().fold(true, |sum, u| {
            sum && if !u.is_goblin { u.health > 0 } else { true }
        });

        if elves_won {
            return outcome;
        }

        attack += 1;
    }
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let (board, units) = read_input(&input);
    println!("Part one: {}", part_one(&board, &mut units.clone()));
    println!("Part two: {}", part_two(&board, &mut units.clone()));
}
