use std::collections::HashMap;

// Values per square:
// 3333333
// 3222223
// 3211123
// 3210123
// 3211123
// 3222223
// 3333333
//
// 0 = 1
// 1 = 4*(1) + 4 = 4*(1 + 1) = 4*(1*2 -1 + 1) = 4*(1*2) = 8*(1)
// 2 = 4*(3) + 4 = 4*(3 + 1) = 4*(2*2 -1 + 1) = 4*(2*2) = 8*(2)
// 3 = 4*(5) + 4 = 4*(5 + 1) = 4*(3*2 -1 + 1) = 4*(3*2) = 8*(3)
// x = 8*x

// Travel pattern:
// |(0,0)|
// |(1, 0) -> (1,1) -> (-1,1) -> (-1,-1) -> (1, -1)|
// |(2,-1) -> (2,2) -> (-2,2) -> (-2,-2) -> (2, -2)|
// |(3,-2) -> (3,3) -> (-3,3) -> (-3,-3) -> (3, -3)|

fn solve_p1(input: i32) {
    assert!(input > 1);

    // Find which square we are in
    let mut square: i32 = 0;
    let mut count: i32 = 1;
    for i in 1..std::i32::MAX {
        let num_vals_in_sq = i * 8;
        square = i as i32;

        if input <= (count + num_vals_in_sq) {
            break;
        }

        count += num_vals_in_sq;
    }

    // Calculate distance to a position that is on the vertical or horizontal middle
    // Steps to first mid: square - 1
    // Steps to next mid: square*2

    let offset_idx: i32 = input - count - 1;
    let dist_to_vert_horz = if offset_idx <= (square - 1) {
        (square - 1) - offset_idx
    } else {
        (offset_idx - (square - 1)) % (square * 2)
    };

    println!("Part one: {}", dist_to_vert_horz + square);
}

fn solve_p2(input: i32) {
    let mut values = HashMap::new();

    let mut square = 1;
    let mut pos = (1, 0);
    let mut goal_id = 0;

    values.insert((0, 0), 1);

    let goal_value;

    loop {
        let mut acc = 0;
        // left
        acc += values.entry((pos.0 - 1, pos.1 - 1)).or_insert(0).clone();
        acc += values.entry((pos.0 - 1, pos.1)).or_insert(0).clone();
        acc += values.entry((pos.0 - 1, pos.1 + 1)).or_insert(0).clone();

        // right
        acc += values.entry((pos.0 + 1, pos.1 - 1)).or_insert(0).clone();
        acc += values.entry((pos.0 + 1, pos.1)).or_insert(0).clone();
        acc += values.entry((pos.0 + 1, pos.1 + 1)).or_insert(0).clone();

        // up/down
        acc += values.entry((pos.0, pos.1 + 1)).or_insert(0).clone();
        acc += values.entry((pos.0, pos.1 - 1)).or_insert(0).clone();

        values.insert(pos, acc);

        if acc > input {
            goal_value = acc;
            break;
        }

        if goal_id > 3 {
            square += 1;
            goal_id = 0;
        }

        let goal_p = match goal_id {
            // Match a single value
            0 => (square, square),
            1 => (-square, square),
            2 => (-square, -square),
            3 => (square + 1, -square),
            _ => (-1337, -1337), // BREAK?
        };

        if pos == goal_p {
            goal_id += 1;
            continue;
        }

        if pos != goal_p {
            if pos.0 != goal_p.0 {
                let dir = if goal_p.0 > pos.0 { 1 } else { -1 };
                pos = (pos.0 + dir, pos.1);
            } else if pos.1 != goal_p.1 {
                let dir = if goal_p.1 > pos.1 { 1 } else { -1 };
                pos = (pos.0, pos.1 + dir);
            }
        }
    }

    println!("Part two: {}", goal_value);
}

pub fn solve() {
    let input = 289326;
    solve_p1(input.clone());
    solve_p2(input.clone());
}
