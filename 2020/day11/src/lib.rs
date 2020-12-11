extern crate common;

type Grid = Vec<Vec<char>>;

fn parse_input(input: &String) -> Grid {
    return input.lines().map(|v| v.chars().collect()).collect();
}

fn num_occupied_adjacent(grid: &Grid, row: i32, col: i32, max_steps: usize) -> usize {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut num_adjacent = 0;
    for (row_step, col_step) in [
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, 0),
        (1, -1),
        (1, 1),
        (0, 1),
        (0, -1),
    ]
    .iter()
    {
        let mut row_curr = row + row_step;
        let mut col_curr = col + col_step;
        let mut steps = 0;
        while row_curr >= 0
            && row_curr < height
            && col_curr >= 0
            && col_curr < width
            && steps < max_steps
        {
            let seat = grid[row_curr as usize][col_curr as usize];
            if seat == '#' {
                num_adjacent += 1;
                break;
            } else if seat == 'L' {
                break;
            }

            row_curr += row_step;
            col_curr += col_step;
            steps += 1;
        }
    }

    return num_adjacent;
}

fn solve_grid(grid: &Grid, min_adjacent: usize, max_steps: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut curr_grid = grid.clone();
    let mut any_change = true;

    while any_change {
        any_change = false;
        let mut grid_new = curr_grid.clone();
        for row in 0..height {
            for col in 0..width {
                let seat = curr_grid[row][col];
                if seat == '.' {
                    continue;
                }

                let num_adj = num_occupied_adjacent(&curr_grid, row as i32, col as i32, max_steps);
                if seat == 'L' && num_adj == 0 {
                    grid_new[row][col] = '#';
                    any_change = true;
                } else if seat == '#' && num_adj >= min_adjacent {
                    grid_new[row][col] = 'L';
                    any_change = true;
                }
            }
        }
        curr_grid = grid_new;
    }

    return curr_grid
        .iter()
        .map(|list| list.iter().filter(|&c| *c == '#').count())
        .sum();
}

fn part_one(grid: &Grid) -> usize {
    return solve_grid(grid, 4, 1);
}

fn part_two(grid: &Grid) -> usize {
    return solve_grid(grid, 5, std::usize::MAX);
}

pub fn solve() {
    let input = common::read_file("2020/day11/input");
    let grid = parse_input(&input);
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 37);
    }
}
