extern crate common;

use std::collections::VecDeque;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct ObjectPos {
    x: usize,
    y: usize,
    c: char,
    dist: usize,
}

#[derive(Debug, Clone)]
struct State {
    keys: Vec<char>,
    grid: Grid,
    steps: usize,
    x: usize,
    y: usize,
}
#[derive(Debug, Clone)]
struct Portal {
    name: String,
    x: usize,
    y: usize,
}

fn parse_input(input: String) -> Grid {
    return input
        .split("\n")
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
}

fn find_portals(grid: &Grid) -> Vec<Portal> {
    // Find all adjacent letters
    let mut portals = Vec::new();
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            let c = grid[y][x];
            let c_up = if y > 0 { grid[y - 1][x] } else { '.' };
            let c_down = if y + 1 < height { grid[y + 1][x] } else { '.' };
            let c_down_down = if y + 2 < height { grid[y + 2][x] } else { '.' };
            let c_right = if x + 1 < width { grid[y][x + 1] } else { '.' };
            let c_right_right = if x + 2 < width { grid[y][x + 2] } else { '.' };
            let c_left = if x > 0 { grid[y][x - 1] } else { '.' };

            if c.is_ascii_alphabetic() && c_down.is_ascii_alphabetic() {
                let name: String = [c, c_down].iter().collect();
                if c_up == '.' {
                    portals.push(Portal {
                        name: name,
                        x: x,
                        y: y - 1,
                    })
                } else if c_down == '.' {
                    portals.push(Portal {
                        name: name,
                        x: x,
                        y: y + 1,
                    })
                }
            }

            if c.is_ascii_alphabetic() && c_right.is_ascii_alphabetic() {
                let name: String = [c, c_right].iter().collect();
                if c_up == '.' {
                    portals.push(Portal {
                        name: name,
                        x: x,
                        y: y - 1,
                    })
                } else if c_down == '.' {
                    portals.push(Portal {
                        name: name,
                        x: x,
                        y: y + 1,
                    })
                }
            }
        }
    }
    return portals;
}

fn find_reachable(grid: Grid, x: usize, y: usize, dist: usize) -> Vec<ObjectPos> {
    let width = grid[0].len();
    let height = grid.len();
    let curr_c = grid[y][x];
    let mut result = Vec::new();

    if curr_c != '.' && curr_c != '#' && curr_c != '@' {
        result.push(ObjectPos {
            x: x,
            y: y,
            c: curr_c,
            dist: dist,
        });
        return result;
    }

    // go left
    if x > 0 && grid[y][x - 1] != '#' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(grid_adj, x - 1, y, dist + 1));
    }
    // go right
    if x < width - 1 && grid[y][x + 1] != '#' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(grid_adj, x + 1, y, dist + 1));
    }
    // go up
    if y > 0 && grid[y - 1][x] != '#' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(grid_adj, x, y - 1, dist + 1));
    }
    // go down
    if y < height - 1 && grid[y + 1][x] != '#' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(grid_adj, x, y + 1, dist + 1));
    }

    return result;
}

fn solve_grid(grid_start: Grid) -> usize {
    let portals = find_portals(&grid_start);
    return 0;
}

pub fn solve() {
    let input = common::read_file("2019/day20/input");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = [
            "         A           ",
            "         A           ",
            "  #######.#########  ",
            "  #######.........#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #######.#######.#  ",
            "  #####  B    ###.#  ",
            "BC...##  C    ###.#  ",
            "  ##.##       ###.#  ",
            "  ##...DE  F  ###.#  ",
            "  #####    G  ###.#  ",
            "  #########.#####.#  ",
            "DE..#######...###.#  ",
            "  #.#########.###.#  ",
            "FG..#########.....#  ",
            "  ###########.#####  ",
            "             Z       ",
            "             Z       ",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 23);
    }

    #[test]
    fn example_b() {
        let input = [
            "                   A               ",
            "                   A               ",
            "  #################.#############  ",
            "  #.#...#...................#.#.#  ",
            "  #.#.#.###.###.###.#########.#.#  ",
            "  #.#.#.......#...#.....#.#.#...#  ",
            "  #.#########.###.#####.#.#.###.#  ",
            "  #.............#.#.....#.......#  ",
            "  ###.###########.###.#####.#.#.#  ",
            "  #.....#        A   C    #.#.#.#  ",
            "  #######        S   P    #####.#  ",
            "  #.#...#                 #......VT",
            "  #.#.#.#                 #.#####  ",
            "  #...#.#               YN....#.#  ",
            "  #.###.#                 #####.#  ",
            "DI....#.#                 #.....#  ",
            "  #####.#                 #.###.#  ",
            "ZZ......#               QG....#..AS",
            "  ###.###                 #######  ",
            "JO..#.#.#                 #.....#  ",
            "  #.#.#.#                 ###.#.#  ",
            "  #...#..DI             BU....#..LF",
            "  #####.#                 #.#####  ",
            "YN......#               VT..#....QG",
            "  #.###.#                 #.###.#  ",
            "  #.#...#                 #.....#  ",
            "  ###.###    J L     J    #.#.###  ",
            "  #.....#    O F     P    #.#...#  ",
            "  #.###.#####.#.#####.#####.###.#  ",
            "  #...#.#.#...#.....#.....#.#...#  ",
            "  #.#####.###.###.#.#.#########.#  ",
            "  #...#.#.....#...#.#.#.#.....#.#  ",
            "  #.###.#####.###.###.#.#.#######  ",
            "  #.#.........#...#.............#  ",
            "  #########.###.###.#############  ",
            "           B   J   C               ",
            "           U   P   P               ",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 58);
    }
}
