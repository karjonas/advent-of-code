extern crate common;

use std::collections::HashSet;
type Pos = [i32; 4];
type Grid = HashSet<Pos>;

fn parse_input(input: &String) -> Grid {
    let vec_grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let width = vec_grid[0].len();
    let height = vec_grid.len();

    let mut grid = Grid::new();
    for y in 0..height {
        for x in 0..width {
            if vec_grid[y][x] == '#' {
                grid.insert([x as i32, y as i32, 0, 0]);
            }
        }
    }
    return grid;
}

fn get_bounds(grid: &Grid) -> (Pos, Pos) {
    let mut min_value = [std::i32::MAX; 4];
    let mut max_value = [std::i32::MIN; 4];

    for dims in grid {
        for i in 0..4 {
            min_value[i] = std::cmp::min(min_value[i], dims[i]);
            max_value[i] = std::cmp::max(max_value[i], dims[i]);
        }
    }

    return (min_value, max_value);
}

fn get_num_neighbours(grid: &Grid, pos: Pos) -> usize {
    let mut ctr = 0;
    for x in pos[0] - 1..pos[0] + 2 {
        for y in pos[1] - 1..pos[1] + 2 {
            for z in pos[2] - 1..pos[2] + 2 {
                for w in pos[3] - 1..pos[3] + 2 {
                    let neigh = [x, y, z, w];
                    if neigh != pos && grid.contains(&[x, y, z, w]) {
                        ctr += 1;
                    }
                }
            }
        }
    }

    return ctr;
}

fn solve_both(grid: &Grid, part_two: bool) -> usize {
    let mut grid_curr = grid.clone();

    for _ in 0..6 {
        let (min, max) = get_bounds(&grid_curr);
        let mut grid_next = grid_curr.clone();

        for x in min[0] - 1..max[0] + 2 {
            for y in min[1] - 1..max[1] + 2 {
                for z in min[2] - 1..max[2] + 2 {
                    for w in min[3] - 1..max[3] + 2 {
                        let pos: Pos = [x, y, z, if part_two { w } else { 0 }];
                        let num_neighs = get_num_neighbours(&grid_curr, pos.clone());
                        let active = grid_curr.contains(&pos);
                        if active && (num_neighs == 2 || num_neighs == 3) {
                            grid_next.insert(pos);
                        } else if !active && num_neighs == 3 {
                            grid_next.insert(pos);
                        } else {
                            grid_next.remove(&pos);
                        }
                    }
                }
            }
        }
        grid_curr = grid_next;
    }

    return grid_curr.len();
}

fn part_one(grid: &Grid) -> usize {
    return solve_both(grid, false);
}

fn part_two(grid: &Grid) -> usize {
    return solve_both(grid, true);
}

pub fn solve() {
    let input = common::read_file("2020/day17/input");
    let grid = parse_input(&input);
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [".#.", "..#", "###"].join("\n");
        assert_eq!(part_one(&parse_input(&input)), 112);
    }
}
