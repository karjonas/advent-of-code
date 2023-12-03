extern crate common;

use std::collections::HashSet;

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    return input
        .lines()
        .map(|v| v.chars().map(|c| (c as u8 - '0' as u8) as usize).collect())
        .collect();
}

fn get_adjacents(pos: (usize, usize), cols: usize, rows: usize) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let y_adj = pos.1 as i32 + dy;
        let x_adj = pos.0 as i32 + dx;

        if y_adj >= rows as i32 || y_adj < 0 || x_adj >= cols as i32 || x_adj < 0 {
            continue;
        }

        results.push((x_adj as usize, y_adj as usize));
    }
    return results;
}

fn solve_internal(grid: &Vec<Vec<usize>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();

    let mut stack = [((0, 0), 0)].to_vec();
    while !stack.is_empty() {
        let (pos, cost) = stack.pop().unwrap();
        if pos == (cols - 1, rows - 1) {
            return cost;
        }

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        for (x1, y1) in get_adjacents(pos, cols, rows) {
            stack.push(((x1, y1), cost + grid[y1][x1]));
        }

        stack.sort_by(|(_, cost_a), (_, cost_b)| return cost_b.cmp(&cost_a));
    }
    return 0;
}

fn solve_internal_p1(input: &String) -> usize {
    return solve_internal(&parse_input(input));
}

fn solve_internal_p2(input: &String) -> usize {
    let mut grid = parse_input(input);
    let grid_orig = grid.clone();
    let cols_orig = grid[0].len();
    let rows_orig = grid.len();
    grid.resize(rows_orig * 5, Vec::new());
    grid.iter_mut().for_each(|col| col.resize(cols_orig * 5, 0));
    let cols = grid[0].len();
    let rows = grid.len();

    for row in 0..rows {
        for col in 0..cols {
            let q_row = row / rows_orig;
            let q_col = col / cols_orig;
            let value = grid_orig[row % rows_orig][col % cols_orig] + q_row + q_col;
            grid[row][col] = value % 10 + value / 10;
        }
    }

    return solve_internal(&grid);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(solve_internal_p1(&String::from(input)), 40);
        assert_eq!(solve_internal_p2(&String::from(input)), 315);
    }
}
