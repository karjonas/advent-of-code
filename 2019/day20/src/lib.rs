extern crate common;

use std::collections::HashMap;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    portal: String,
    steps: usize,
    level: i32,
    is_inner: bool,
}

#[derive(Debug, Clone)]
struct Portal {
    name: String,
    start_x: usize,
    start_y: usize,
    is_inner: bool,
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
            let c_up = if y > 0 { grid[y - 1][x] } else { '-' };
            let c_down = if y + 1 < height { grid[y + 1][x] } else { '-' };
            let c_down_down = if y + 2 < height { grid[y + 2][x] } else { '-' };
            let c_right = if x + 1 < width { grid[y][x + 1] } else { '-' };
            let c_right_right = if x + 2 < width { grid[y][x + 2] } else { '-' };
            let c_left = if x > 0 { grid[y][x - 1] } else { '-' };

            // Ugly check if we are inside our outside portal
            let is_inner = x > 5 && x < width - 5 && y > 5 && y < height - 5;

            if c.is_ascii_alphabetic() && c_down.is_ascii_alphabetic() {
                let name: String = [c, c_down].iter().collect();
                if c_up == '.' {
                    portals.push(Portal {
                        name: name,
                        start_x: x,
                        start_y: y - 1,
                        is_inner: is_inner,
                    })
                } else if c_down_down == '.' {
                    portals.push(Portal {
                        name: name,
                        start_x: x,
                        start_y: y + 2,
                        is_inner: is_inner,
                    })
                }
            }

            if c.is_ascii_alphabetic() && c_right.is_ascii_alphabetic() {
                let name: String = [c, c_right].iter().collect();
                if c_left == '.' {
                    portals.push(Portal {
                        name: name,
                        start_x: x - 1,
                        start_y: y,
                        is_inner: is_inner,
                    })
                } else if c_right_right == '.' {
                    portals.push(Portal {
                        name: name,
                        start_x: x + 2,
                        start_y: y,
                        is_inner: is_inner,
                    })
                }
            }
        }
    }

    return portals;
}

fn find_reachable(
    grid: Grid,
    portals: &Vec<Portal>,
    origin: &String,
    x: usize,
    y: usize,
    dist: usize,
) -> Vec<(String, usize, bool)> {
    let width = grid[0].len();
    let height = grid.len();
    let mut result = Vec::new();

    for portal in portals {
        if portal.name == *origin {
            continue;
        }

        if (portal.start_x, portal.start_y) == (x, y) {
            return [(portal.name.clone(), dist + 1, portal.is_inner)].to_vec();
        }
    }

    // go left
    if x > 0 && grid[y][x - 1] == '.' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(
            grid_adj,
            &portals,
            &origin,
            x - 1,
            y,
            dist + 1,
        ));
    }
    // go right
    if x < width - 1 && grid[y][x + 1] == '.' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(
            grid_adj,
            &portals,
            &origin,
            x + 1,
            y,
            dist + 1,
        ));
    }
    // go up
    if y > 0 && grid[y - 1][x] == '.' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(
            grid_adj,
            &portals,
            &origin,
            x,
            y - 1,
            dist + 1,
        ));
    }
    // go down
    if y < height - 1 && grid[y + 1][x] == '.' {
        let mut grid_adj = grid.clone();
        grid_adj[y][x] = '#';
        result.append(&mut find_reachable(
            grid_adj,
            &portals,
            &origin,
            x,
            y + 1,
            dist + 1,
        ));
    }

    return result;
}

fn solve_grid(grid_start: Grid, count_levels: bool) -> usize {
    let portals = find_portals(&grid_start);
    let mut reachable = HashMap::new();

    for portal in portals.clone() {
        let mut new_reach = find_reachable(
            grid_start.clone(),
            &portals,
            &portal.name,
            portal.start_x,
            portal.start_y,
            0,
        );
        reachable
            .entry((portal.name.clone(), portal.is_inner))
            .or_insert(Vec::<(String, usize, bool)>::new())
            .append(&mut new_reach);
    }

    let mut stack = Vec::new();
    stack.push(State {
        portal: String::from("AA"),
        steps: 0,
        level: 0,
        is_inner: false,
    });

    let mut visited = HashSet::new();

    while !stack.is_empty() {
        stack.sort_by(|a, b| b.steps.cmp(&a.steps));
        let state = stack.pop().unwrap();

        if visited.contains(&(state.portal.clone(), state.level, state.is_inner)) {
            continue;
        }

        visited.insert((state.portal.clone(), state.level, state.is_inner));
        if (state.level <= 0 || !count_levels) && state.portal == "ZZ" {
            return state.steps - 1;
        }

        if !reachable.contains_key(&(state.portal.clone(), state.is_inner)) {
            continue;
        }

        let neighs = reachable
            .get(&(state.portal.clone(), state.is_inner))
            .unwrap();

        if count_levels {
            for (neigh, dist, is_inner) in neighs {
                // Disallow endpoints in upper levels
                if state.level > 0 && (neigh == "ZZ" || neigh == "AA") {
                    continue;
                }

                if state.level <= 0 && neigh != "ZZ" && !is_inner {
                    continue;
                }

                let state_next = State {
                    portal: neigh.clone(),
                    steps: state.steps + dist,
                    level: state.level + if *is_inner { 1 } else { -1 },
                    is_inner: !*is_inner,
                };

                stack.push(state_next);
            }
        } else {
            for (neigh, dist, is_inner) in neighs {
                let state_next = State {
                    portal: neigh.clone(),
                    steps: state.steps + dist,
                    level: 0,
                    is_inner: !*is_inner,
                };

                stack.push(state_next);
            }
        }
    }

    panic!("Could not find solution");
}

pub fn solve() {
    let input = common::read_file("2019/day20/input");
    println!(
        "Part one: {}",
        solve_grid(parse_input(input.clone()), false)
    );
    println!("Part two: {}", solve_grid(parse_input(input), true));
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
        assert_eq!(solve_grid(grid, false), 23);
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
        assert_eq!(solve_grid(grid, false), 58);
    }

    #[test]
    fn example_c() {
        let input = [
            "             Z L X W       C                 ",
            "             Z P Q B       K                 ",
            "  ###########.#.#.#.#######.###############  ",
            "  #...#.......#.#.......#.#.......#.#.#...#  ",
            "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
            "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
            "  #.###.#######.###.###.#.###.###.#.#######  ",
            "  #...#.......#.#...#...#.............#...#  ",
            "  #.#########.#######.#.#######.#######.###  ",
            "  #...#.#    F       R I       Z    #.#.#.#  ",
            "  #.###.#    D       E C       H    #.#.#.#  ",
            "  #.#...#                           #...#.#  ",
            "  #.###.#                           #.###.#  ",
            "  #.#....OA                       WB..#.#..ZH",
            "  #.###.#                           #.#.#.#  ",
            "CJ......#                           #.....#  ",
            "  #######                           #######  ",
            "  #.#....CK                         #......IC",
            "  #.###.#                           #.###.#  ",
            "  #.....#                           #...#.#  ",
            "  ###.###                           #.#.#.#  ",
            "XF....#.#                         RF..#.#.#  ",
            "  #####.#                           #######  ",
            "  #......CJ                       NM..#...#  ",
            "  ###.#.#                           #.###.#  ",
            "RE....#.#                           #......RF",
            "  ###.###        X   X       L      #.#.#.#  ",
            "  #.....#        F   Q       P      #.#.#.#  ",
            "  ###.###########.###.#######.#########.###  ",
            "  #.....#...#.....#.......#...#.....#.#...#  ",
            "  #####.#.###.#######.#######.###.###.#.#.#  ",
            "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
            "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
            "  #.......#.....#.#...#...............#...#  ",
            "  #############.#.#.###.###################  ",
            "               A O F   N                     ",
            "               A A D   M                     ",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid, true), 396);
    }
}
