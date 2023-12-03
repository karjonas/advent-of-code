use std::mem;
use std::usize;

const GRID_SIZE: usize = 100;

fn is_wall(x: usize, y: usize, seed: usize) -> bool {
    let v = x * x + 3 * x + 2 * x * y + y + y * y + seed;
    let max_bits = mem::size_of::<usize>() * 8;
    let mut num_set = 0;
    for i in 0..max_bits {
        let b: usize = 1 << i;
        num_set = if v & b > 0 { num_set + 1 } else { num_set };
    }

    let is_even = num_set % 2 == 0;
    return !is_even;
}

fn calc_grid(seed: usize) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut g = [[false; GRID_SIZE]; GRID_SIZE];
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            g[x][y] = is_wall(x, y, seed);
        }
    }

    return g;
}

fn calc_dist_and_num_visited(
    start: (usize, usize),
    stop: (usize, usize),
    max_steps: usize,
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
) -> (usize, usize) {
    let mut stack: Vec<(usize, (usize, usize))> = Vec::new();
    let mut visited = [[false; GRID_SIZE]; GRID_SIZE];
    let mut ret_val = 0;
    stack.push((0, start));
    let mut prev_step = 0;
    while !stack.is_empty() {
        let (steps, (x, y)) = stack.pop().unwrap();
        assert!(steps >= prev_step);
        prev_step = steps;
        if steps > max_steps || grid[x][y] || visited[x][y] {
            continue;
        }
        visited[x][y] = true;

        if ret_val == 0 && stop.0 == x && stop.1 == y {
            ret_val = steps;
        }

        if (x + 1) < GRID_SIZE {
            stack.push((steps + 1, (x + 1, y)));
        }
        if x > 0 {
            stack.push((steps + 1, (x - 1, y)));
        }

        if (y + 1) < GRID_SIZE {
            stack.push((steps + 1, (x, y + 1)));
        }
        if y > 0 {
            stack.push((steps + 1, (x, y - 1)));
        }

        stack.sort_by(|a, b| b.0.cmp(&a.0));
    }

    let num_visited = visited.iter().fold(0, |sum, v| {
        sum + v
            .iter()
            .fold(0, |sum_v, v_i| sum_v + if v_i.clone() { 1 } else { 0 })
    });

    return (ret_val, num_visited);
}

pub fn solve(_filepath: &str) {
    let g = calc_grid(1362);
    let (dist, _) = calc_dist_and_num_visited((1, 1), (31, 39), GRID_SIZE, g);
    let (_, visited) = calc_dist_and_num_visited((1, 1), (31, 39), 50, g);

    println!("Part 1: {}", dist);
    println!("Part 2: {}", visited);
}
