extern crate common;

fn is_cart(c: char) -> bool {
    return c == '^' || c == 'v' || c == '<' || c == '>';
}

fn is_horizontal(c: char) -> bool {
    return c == '+' || c == '-' || c == '/' || c == '\\';
}

fn is_vertical(c: char) -> bool {
    return c == '+' || c == '|' || c == '/' || c == '\\';
}

fn find_replace_char(up: char, down: char, left: char, right: char) -> char {
    let ret = if is_horizontal(left) && is_horizontal(right) && is_vertical(up) && is_vertical(down)
    {
        '+'
    } else if is_vertical(up) && is_horizontal(left) {
        '/'
    } else if is_vertical(up) && is_horizontal(right) {
        '\\'
    } else if is_vertical(down) && is_horizontal(right) {
        '/'
    } else if is_vertical(down) && is_horizontal(left) {
        '\\'
    } else if is_horizontal(left) && is_horizontal(right) {
        '-'
    } else if is_vertical(up) && is_vertical(down) {
        '|'
    } else {
        ' '
    };

    assert!(ret != ' ');
    return ret;
}

fn find_new_cart_pos(grid: &Vec<Vec<char>>, pos: Cart) -> Cart {
    let x = pos.x;
    let y = pos.y;
    let mut c = pos.dir;
    let mut is = pos.cross_ctr;
    let tick = pos.tick;

    let mut res: (usize, usize, char) = (0, 0, '.');

    let height = grid.len();
    let width = grid[0].len();

    let up = if y == 0 { ' ' } else { grid[y - 1][x] };
    let down = if y == height - 1 { ' ' } else { grid[y + 1][x] };
    let left = if x == 0 { ' ' } else { grid[y][x - 1] };
    let right = if x == width - 1 { ' ' } else { grid[y][x + 1] };
    let curr = grid[y][x];

    if curr == '+' {
        if c == '>' {
            if is == 0 {
                c = '^';
            } else if is == 1 {
                c = '>';
            } else if is == 2 {
                c = 'v';
            }
        } else if c == '<' {
            if is == 0 {
                c = 'v';
            } else if is == 1 {
                c = '<';
            } else if is == 2 {
                c = '^';
            }
        } else if c == '^' {
            if is == 0 {
                c = '<';
            } else if is == 1 {
                c = '^';
            } else if is == 2 {
                c = '>';
            }
        } else if c == 'v' {
            if is == 0 {
                c = '>';
            } else if is == 1 {
                c = 'v';
            } else if is == 2 {
                c = '<';
            }
        } else {
            assert!(false);
        }

        is = (is + 1) % 3;
    }

    if c == '>' {
        if right == '+' || right == '-' {
            res = (x + 1, y, '>');
        } else if right == '\\' {
            res = (x + 1, y, 'v');
        } else if right == '/' {
            res = (x + 1, y, '^');
        }
    } else if c == '<' {
        if left == '+' || left == '-' {
            res = (x - 1, y, '<');
        } else if left == '\\' {
            res = (x - 1, y, '^');
        } else if left == '/' {
            res = (x - 1, y, 'v');
        }
    } else if c == '^' {
        if up == '|' || up == '+' {
            res = (x, y - 1, '^');
        } else if up == '\\' {
            res = (x, y - 1, '<');
        } else if up == '/' {
            res = (x, y - 1, '>');
        }
    } else if c == 'v' {
        if down == '|' || down == '+' {
            res = (x, y + 1, 'v');
        } else if down == '\\' {
            res = (x, y + 1, '>');
        } else if down == '/' {
            res = (x, y + 1, '<');
        }
    }

    return Cart {
        x: res.0,
        y: res.1,
        dir: res.2,
        cross_ctr: is,
        tick: tick,
        crash: false,
    };
}

#[derive(Clone, Debug)]
struct Cart {
    x: usize,
    y: usize,
    dir: char,
    cross_ctr: i32,
    tick: usize,
    crash: bool,
}

#[derive(Clone, Debug)]
struct GridState {
    grid: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

fn print_grid(state: &GridState) {
    let mut grid = state.grid.clone();

    for cart in state.carts.clone() {
        grid[cart.y][cart.x] = cart.dir;
    }

    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
}

fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_by(|a, b| {
        if a.y == b.y {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });
}

fn step_grid(pos_new: &mut Vec<Cart>, grid: &Vec<Vec<char>>) {
    sort_carts(pos_new);
    let num_carts = pos_new.len();

    for i in 0..num_carts {
        if pos_new[i].crash {
            continue;
        }

        pos_new[i] = find_new_cart_pos(grid, pos_new[i].clone());
        for j in 0..num_carts {
            if i == j {
                continue;
            }
            if !pos_new[j].crash && pos_new[i].x == pos_new[j].x && pos_new[i].y == pos_new[j].y {
                pos_new[i].crash = true;
                pos_new[j].crash = true;
            }
        }
    }
}

fn read_grid(input: &String) -> GridState {
    let mut start_positions: Vec<Cart> = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        grid.push(chars);
    }

    let width = grid[0].len();
    let height = grid.len();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            if is_cart(c) {
                let up = if y == 0 { ' ' } else { grid[y - 1][x] };
                let down = if y == height - 1 { ' ' } else { grid[y + 1][x] };
                let left = if x == 0 { ' ' } else { grid[y][x - 1] };
                let right = if x == width - 1 { ' ' } else { grid[y][x + 1] };
                let rep = find_replace_char(up, down, left, right);
                grid[y][x] = rep;
                start_positions.push(Cart {
                    x: x,
                    y: y,
                    dir: c,
                    cross_ctr: 0,
                    tick: 0,
                    crash: false,
                });
            }
        }
    }

    return GridState {
        grid: grid,
        carts: start_positions,
    };
}

fn part_two(grid_state: &mut GridState) -> String {
    loop {
        step_grid(&mut grid_state.carts, &grid_state.grid);
        let num_crashed = grid_state
            .carts
            .iter()
            .fold(0, |sum, val| sum + if val.crash { 1 } else { 0 });

        if grid_state.carts.len() - num_crashed == 1 {
            let last_cart = grid_state.carts.iter().find(|&v| !v.crash).unwrap();
            return last_cart.x.to_string() + "," + &last_cart.y.to_string();
        }

        if false {
            print_grid(&grid_state)
        };
    }
}

fn part_one(grid_state: &mut GridState) -> String {
    loop {
        step_grid(&mut grid_state.carts, &grid_state.grid);
        let num_crashed = grid_state
            .carts
            .iter()
            .fold(0, |sum, val| sum + if val.crash { 1 } else { 0 });

        if num_crashed > 0 {
            let crash_cart = grid_state.carts.iter().find(|&v| v.crash).unwrap();
            return crash_cart.x.to_string() + "," + &crash_cart.y.to_string();
        }

        if false {
            print_grid(&grid_state)
        };
    }
}
pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath).unwrap().to_string();
    let grid_state = read_grid(&input);
    println!("Part one: {}", part_one(&mut grid_state.clone()));
    println!("Part two: {}", part_two(&mut grid_state.clone()));
}
