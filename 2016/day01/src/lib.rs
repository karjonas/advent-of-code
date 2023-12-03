fn do_step(dir_in: i32, x: i32, y: i32, turn: i32, steps: i32) -> (i32, i32, i32) {
    let dir_add = dir_in + turn;
    let d = if dir_add == -1 { 3 } else { dir_add % 4 };

    return match d {
        0 => (d, x, y + steps),
        1 => (d, x + steps, y),
        2 => (d, x, y - steps),
        3 => (d, x - steps, y),
        _ => (d, x, y),
    };
}

fn find_intersection(
    v: &Vec<(i32, i32, i32, i32)>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) -> (bool, i32, i32) {
    let a_horiz = y0 == y1;

    let ax0 = std::cmp::min(x0, x1);
    let ax1 = std::cmp::max(x0, x1);

    let ay0 = std::cmp::min(y0, y1);
    let ay1 = std::cmp::max(y0, y1);

    let end_idx = if v.len() > 0 { v.len() - 1 } else { 0 };

    for i in 0..end_idx {
        let tup = v[i];
        let bx0 = std::cmp::min(tup.0, tup.2);
        let bx1 = std::cmp::max(tup.0, tup.2);

        let by0 = std::cmp::min(tup.1, tup.3);
        let by1 = std::cmp::max(tup.1, tup.3);

        let b_horiz = by0 == by1;

        if a_horiz && !b_horiz && (ax0 <= bx0 && bx0 <= ax1) && (by0 <= ay0 && ay0 <= by1) {
            return (true, bx0, ay0);
        }
        if !a_horiz && b_horiz && (ay0 <= by0 && by0 <= ay1) && (bx0 <= ax0 && ax0 <= bx1) {
            return (true, ax0, by0);
        }
    }

    return (false, 0, 0);
}

pub fn solve(_filepath: &str) {
    let input = vec![
        ('R', 5),
        ('R', 4),
        ('R', 2),
        ('L', 3),
        ('R', 1),
        ('R', 1),
        ('L', 4),
        ('L', 5),
        ('R', 3),
        ('L', 1),
        ('L', 1),
        ('R', 4),
        ('L', 2),
        ('R', 1),
        ('R', 4),
        ('R', 4),
        ('L', 2),
        ('L', 2),
        ('R', 4),
        ('L', 4),
        ('R', 1),
        ('R', 3),
        ('L', 3),
        ('L', 1),
        ('L', 2),
        ('R', 1),
        ('R', 5),
        ('L', 5),
        ('L', 1),
        ('L', 1),
        ('R', 3),
        ('R', 5),
        ('L', 1),
        ('R', 4),
        ('L', 5),
        ('R', 5),
        ('R', 1),
        ('L', 185),
        ('R', 4),
        ('L', 1),
        ('R', 51),
        ('R', 3),
        ('L', 2),
        ('R', 78),
        ('R', 1),
        ('L', 4),
        ('R', 188),
        ('R', 1),
        ('L', 5),
        ('R', 5),
        ('R', 2),
        ('R', 3),
        ('L', 5),
        ('R', 3),
        ('R', 4),
        ('L', 1),
        ('R', 2),
        ('R', 2),
        ('L', 4),
        ('L', 4),
        ('L', 5),
        ('R', 5),
        ('R', 4),
        ('L', 4),
        ('R', 2),
        ('L', 5),
        ('R', 2),
        ('L', 1),
        ('L', 4),
        ('R', 4),
        ('L', 4),
        ('R', 2),
        ('L', 3),
        ('L', 4),
        ('R', 2),
        ('L', 3),
        ('R', 3),
        ('R', 2),
        ('L', 2),
        ('L', 3),
        ('R', 4),
        ('R', 3),
        ('R', 1),
        ('L', 4),
        ('L', 2),
        ('L', 5),
        ('R', 4),
        ('R', 4),
        ('L', 1),
        ('R', 1),
        ('L', 5),
        ('L', 1),
        ('R', 3),
        ('R', 1),
        ('L', 2),
        ('R', 1),
        ('R', 1),
        ('R', 3),
        ('L', 4),
        ('L', 1),
        ('L', 3),
        ('R', 2),
        ('R', 4),
        ('R', 2),
        ('L', 2),
        ('R', 1),
        ('L', 5),
        ('R', 3),
        ('L', 3),
        ('R', 3),
        ('L', 1),
        ('R', 4),
        ('L', 3),
        ('L', 3),
        ('R', 4),
        ('L', 2),
        ('L', 1),
        ('L', 3),
        ('R', 2),
        ('R', 3),
        ('L', 2),
        ('L', 1),
        ('R', 4),
        ('L', 3),
        ('L', 5),
        ('L', 2),
        ('L', 4),
        ('R', 1),
        ('L', 4),
        ('L', 4),
        ('R', 3),
        ('R', 5),
        ('L', 4),
        ('L', 1),
        ('L', 1),
        ('R', 4),
        ('L', 2),
        ('R', 5),
        ('R', 1),
        ('R', 1),
        ('R', 2),
        ('R', 1),
        ('R', 5),
        ('L', 1),
        ('L', 3),
        ('L', 5),
        ('R', 2),
    ];

    // let input = vec![('R',8), ('R', 4), ('R', 4), ('R', 8)];
    let mut dir = 3;
    let mut x = 0;
    let mut y = 0;
    let mut v: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut dist_find_twice = 0;

    for tup in input {
        let turn: i32 = if tup.0 == 'L' { -1 } else { 1 };
        let steps: i32 = tup.1;
        let ret = do_step(dir, x, y, turn, steps);
        dir = ret.0;

        if dist_find_twice == 0 {
            let isect = find_intersection(&v, x, y, ret.1, ret.2);

            if isect.0 {
                x = isect.1;
                y = isect.2;
                dist_find_twice = x.abs() + y.abs();
            }

            v.push((x, y, ret.1, ret.2));
        }
        x = ret.1;
        y = ret.2;
    }

    println!("Part 1: {}", x.abs() + y.abs());
    println!("Part 2: {}", dist_find_twice);
}
