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

fn parse_input(input: String) -> Grid {
    return input
        .split("\n")
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
}

fn find_unit(grid: &Grid) -> ObjectPos {
    let height = grid.len();
    let width = grid[0].len();

    for x in 0..width {
        for y in 0..height {
            if grid[y][x] == '@' {
                return ObjectPos {
                    x: x,
                    y: y,
                    c: '@',
                    dist: 0,
                };
            }
        }
    }

    panic!("Could not find unit");
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
    let unit = find_unit(&grid_start);

    let mut states = VecDeque::new();
    states.push_back(State {
        grid: grid_start.clone(),
        keys: Vec::new(),
        steps: 0,
        x: unit.x,
        y: unit.y,
    });
    let mut prev = 0;
    let mut best = 9999999;
    loop {
        //states.sort_by(|a, b| b.steps.cmp(&a.steps));
        if states.is_empty() {
            break;
        }
        let mut state = states.pop_front().unwrap();

        if state.steps >= best {
            continue;
        }

        let mut reachable = find_reachable(state.grid.clone(), state.x, state.y, 0);
        if reachable.is_empty() {
            best = std::cmp::min(best, state.steps);
        }

        prev = state.steps;

        reachable.sort_by(|a, b| b.dist.cmp(&a.dist));

        for object in reachable {
            if object.c.is_uppercase() && state.keys.contains(&object.c.to_ascii_lowercase()) {
                // Door with key
                // remove door from grid
                let mut grid_new = state.grid.clone();
                grid_new[object.y][object.x] = '.';
                // remove key
                let mut keys_new = state.keys.clone();
                keys_new.retain(|v| *v != object.c.to_ascii_lowercase());

                let mut state_next = State {
                    grid: grid_new,
                    keys: keys_new,
                    steps: state.steps + object.dist,
                    x: object.x,
                    y: object.y,
                };
                states.push_back(state_next);
            } else if object.c.is_lowercase() {
                // key
                let mut grid_new = state.grid.clone();
                grid_new[object.y][object.x] = '.';
                // remove key
                let mut keys_new = state.keys.clone();
                keys_new.push(object.c);

                let mut state_next = State {
                    grid: grid_new,
                    keys: keys_new,
                    steps: state.steps + object.dist,
                    x: object.x,
                    y: object.y,
                };
                states.push_back(state_next);
            }
        }
    }

    return best;
}

pub fn solve() {
    //  let input = ["#########", "#b.A.@.a#", "#########"].join("\n");

    //let input1 = [
    //    "#################",
    //    "#i.G..c...e..H.p#",
    //    "########.########",
    //    "#j.A..b...f..D.o#",
    //    "########@########",
    //    "#k.E..a...g..B.n#",
    //    "########.########",
    //    "#l.F..d...h..C.m#",
    //    "#################",
    //]
    //.join("\n");

    //let input = [
    //    "########################",
    //    "#f.D.E.e.C.b.A.@.a.B.c.#",
    //    "######################.#",
    //    "#d.....................#",
    //    "########################",
    //]
    //.join("\n");

    let input = common::read_file("2019/day18/input");
    let grid = parse_input(input);
    let unit = find_unit(&grid);
    //grid[unit.y][unit.x] = '.';
    let reachable = find_reachable(grid.clone(), unit.x, unit.y, 0);

    let x = solve_grid(grid.clone());

    println!("{:?}", x);
    println!("{:?}", reachable);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = ["#########", "#b.A.@.a#", "#########"].join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 8);
    }

    #[test]
    fn example_b() {
        let input = [
            "########################",
            "#f.D.E.e.C.b.A.@.a.B.c.#",
            "######################.#",
            "#d.....................#",
            "########################",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 86);
    }

    #[test]
    fn example_c() {
        let input = [
            "########################",
            "#...............b.C.D.f#",
            "#.######################",
            "#.....@.a.B.c.d.A.e.F.g#",
            "########################",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 132);
    }

    #[test]
    fn example_d() {
        let input = [
            "#################",
            "#i.G..c...e..H.p#",
            "########.########",
            "#j.A..b...f..D.o#",
            "########@########",
            "#k.E..a...g..B.n#",
            "########.########",
            "#l.F..d...h..C.m#",
            "#################",
        ]
        .join("\n");
        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 136);
    }

    #[test]
    fn example_e() {
        let input = [
            "########################",
            "#@..............ac.GI.b#",
            "###d#e#f################",
            "###A#B#C################",
            "###g#h#i################",
            "########################",
        ]
        .join("\n");

        let grid = parse_input(input);
        assert_eq!(solve_grid(grid), 81);
    }
}
