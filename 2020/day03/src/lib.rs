extern crate common;

type GridT = Vec<Vec<char>>;

fn parse_input(input: &String) -> GridT {
    return input.lines().map(|line| line.chars().collect()).collect();
}

fn solve_grid(grid: &GridT, step_right: usize, step_down: usize) -> usize {
    let mut pos = (0, 0);
    let mut hit_trees = 0;
    let height = grid.len();
    let width = grid[0].len();

    while pos.1 < height {
        if grid[pos.1][pos.0] == '#' {
            hit_trees += 1;
        }

        pos = ((pos.0 + step_right) % width, pos.1 + step_down);
    }

    return hit_trees;
}

fn part_one(grid: &GridT) -> usize {
    return solve_grid(&grid, 3, 1);
}

fn part_two(grid: &GridT) -> usize {
    return solve_grid(&grid, 1, 1)
        * solve_grid(&grid, 3, 1)
        * solve_grid(&grid, 5, 1)
        * solve_grid(&grid, 7, 1)
        * solve_grid(&grid, 1, 2);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let grid = parse_input(&input);
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 7);
    }

    #[test]
    fn test_samples_part_two() {
        let input = [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .join("\n");
        assert_eq!(part_two(&parse_input(&input)), 336);
    }
}
