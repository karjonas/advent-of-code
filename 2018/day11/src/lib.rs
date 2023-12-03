extern crate common;

fn get_value(pos: (usize, usize), size: usize, grid_summed: &Vec<Vec<Vec<i64>>>) -> i64 {
    let x_start = pos.0;
    let y_start = pos.1;
    let y_end = y_start + size;

    let mut sum: i64 = 0;
    for y in y_start..y_end {
        sum += grid_summed[x_start][y][size];
    }

    return sum;
}

fn find_best_solution(grid_summed: &Vec<Vec<Vec<i64>>>, part_one: bool) -> (usize, usize, usize) {
    let mut best_index = (0, 0, 0);
    let mut best_value = -1;
    for x in 1..301 {
        for y in 1..301 {
            let min_size = if part_one { 3 } else { 1 };
            let max_size = if part_one {
                std::cmp::min(3, std::cmp::min(301 - x, 301 - y))
            } else {
                std::cmp::min(301 - x, 301 - y)
            };

            for z in min_size..(max_size + 1) {
                let sum = get_value((x, y), z, &grid_summed);
                if best_value < sum {
                    best_value = sum;
                    best_index = (x, y, z);
                }
            }
        }
    }

    return (best_index.0, best_index.1, best_index.2);
}

pub fn solve(_filepath: &str) {
    let serial_number = 9306;
    let mut grid = common::filled_vector(301, common::filled_vector(301, 0 as i64));
    let mut grid_summed = common::filled_vector(
        301,
        common::filled_vector(301, common::filled_vector(301, 0 as i64)),
    );

    for x in 1..301 {
        for y in 1..301 {
            let rack_id = x + 10;
            let power_level = (rack_id * y + serial_number) * rack_id;
            let digit = (power_level % 1000 - (power_level % 100)) / 100;
            grid[x][y] = digit as i64 - 5;
        }
    }

    // Calculate some partial sums to speed-up solution
    for x_inv in 1..301 {
        let x = 301 - x_inv;
        for y_inv in 1..301 {
            let y = 301 - y_inv;
            for size in 1..(301 - x + 1) {
                grid_summed[x][y][size] = grid[x][y]
                    + if size == 1 {
                        0
                    } else {
                        grid_summed[x + 1][y][size - 1]
                    };
            }
        }
    }

    let res0 = find_best_solution(&grid_summed, true);
    println!("Part one: {},{}", res0.0, res0.1);
    let res1 = find_best_solution(&grid_summed, false);
    println!("Part two: {},{},{}", res1.0, res1.1, res1.2);
}
