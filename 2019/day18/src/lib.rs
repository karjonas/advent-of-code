extern crate common;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct ObjectPos {
    x: usize,
    y: usize,
    c: char,
    dist: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UnitsState {
    unit_pos: [char; 4],
    keys: BTreeSet<char>,
}

#[derive(Debug, Clone)]
struct MetaState {
    steps: usize,
    state: UnitsState,
    neighs: HashMap<char, HashMap<char, usize>>,
}

fn parse_input(input: String, is_part_two: bool) -> Grid {
    let mut grid = input
        .trim()
        .split("\n")
        .map(|v| v.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let u = find_unit(&grid);

    if is_part_two {
        grid[u.y][u.x] = '#';
        grid[u.y - 1][u.x] = '#';
        grid[u.y + 1][u.x] = '#';
        grid[u.y][u.x - 1] = '#';
        grid[u.y][u.x + 1] = '#';

        grid[u.y - 1][u.x - 1] = '1';
        grid[u.y - 1][u.x + 1] = '2';
        grid[u.y + 1][u.x - 1] = '3';
        grid[u.y + 1][u.x + 1] = '4';
    } else {
        grid[u.y][u.x] = '1';
    }
    return grid;
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

fn find_items(grid: &Grid) -> Vec<ObjectPos> {
    let height = grid.len();
    let width = grid[0].len();
    let mut result = Vec::new();

    for x in 0..width {
        for y in 0..height {
            assert_eq!(grid[y].len(), width);
            if grid[y][x] != '.' && grid[y][x] != '#' {
                result.push(ObjectPos {
                    x: x,
                    y: y,
                    c: grid[y][x],
                    dist: 0,
                });
            }
        }
    }

    return result;
}

fn find_reachable(grid: Grid, x: usize, y: usize, dist: usize) -> Vec<ObjectPos> {
    let width = grid[0].len();
    let height = grid.len();
    let curr_c = grid[y][x];
    let mut result = Vec::new();

    if !['.', '#', '@', '1', '2', '3', '4'].contains(&curr_c) {
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

fn delete_neigh(
    input: &HashMap<char, HashMap<char, usize>>,
    c: char,
) -> HashMap<char, HashMap<char, usize>> {
    let mut result = input.clone();

    // Erase c from map
    result.get_mut(&c).unwrap().clear();
    for (_k, v) in result.iter_mut() {
        v.remove(&c);
    }

    let c_neighs = input.get(&c).unwrap().clone();

    // Connect all c neighbours
    for (neigh_a, dist_a) in c_neighs.clone() {
        let neighs_a = result.entry(neigh_a).or_insert(HashMap::new());
        for (neigh_b, dist_b) in c_neighs.clone() {
            if neigh_a == neigh_b {
                continue;
            }
            let dist_tot = dist_a + dist_b;
            let dist_prev = neighs_a.entry(neigh_b).or_insert(dist_tot).clone();
            neighs_a.insert(neigh_b, std::cmp::min(dist_tot, dist_prev));
        }
    }

    return result;
}

fn solve_grid(grid_start: Grid, is_part_two: bool) -> usize {
    let mut neighs_start: HashMap<char, HashMap<char, usize>> = HashMap::new();
    let mut visited_start = HashMap::new();
    let mut all_chars = Vec::new();
    let mut all_keys = Vec::new();

    {
        let items = find_items(&grid_start);
        for item in items {
            let mut grid = grid_start.clone();
            grid[item.y][item.x] = '.';
            let reachable = find_reachable(grid, item.x, item.y, 0);

            for neigh in reachable {
                neighs_start
                    .entry(item.c)
                    .or_insert(HashMap::new())
                    .insert(neigh.c, neigh.dist);
            }

            visited_start.insert(item.c, false);
            all_chars.push(item.c);

            if item.c.is_ascii_lowercase() {
                all_keys.push(item.c);
            }
        }
    }

    let mut stack = Vec::new();
    stack.push(MetaState {
        steps: 0,
        state: UnitsState {
            unit_pos: ['1', '2', '3', '4'],
            keys: BTreeSet::new(),
        },
        neighs: neighs_start.clone(),
    });

    let mut visited_states = HashSet::new();

    while !stack.is_empty() {
        stack.sort_by(|a, b| b.steps.cmp(&a.steps));

        let metastate = stack.pop().unwrap();
        let steps = metastate.steps;

        if visited_states.contains(&metastate.state) {
            continue;
        }

        visited_states.insert(metastate.state.clone());

        if metastate.state.keys.len() == all_keys.len() {
            return steps;
        }

        for i in 0..4 {
            let unit = metastate.state.unit_pos[i];
            let this_neighs = metastate.neighs.get(&unit).unwrap().clone();

            for (n_c, n_d) in &this_neighs {
                let is_door_with_key =
                    n_c.is_uppercase() && metastate.state.keys.contains(&n_c.to_ascii_lowercase());
                let is_key = n_c.is_lowercase();

                if is_door_with_key || is_key {
                    let mut state_next = metastate.state.clone();

                    // Update unit
                    state_next.unit_pos[i] = n_c.clone();
                    // Add key
                    if is_key {
                        state_next.keys.insert(n_c.clone());
                    }

                    if visited_states.contains(&state_next) {
                        continue;
                    }

                    stack.push(MetaState {
                        steps: metastate.steps + n_d,
                        state: state_next,
                        neighs: delete_neigh(&metastate.neighs, unit),
                    });
                }
            }

            if !is_part_two {
                break;
            }
        }
    }

    panic!("No solution found");
}

fn solve_part_one(input: String) -> usize {
    return solve_grid(parse_input(input, false), false);
}

fn solve_part_two(input: String) -> usize {
    return solve_grid(parse_input(input, true), true);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim()
        .to_string()
        .trim()
        .to_string();
    println!("Part one: {}", solve_part_one(input.clone()));
    println!("Part two: {}", solve_part_two(input.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let input = ["#########", "#b.A.@.a#", "#########"].join("\n");
        assert_eq!(solve_part_one(input), 8);
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
        assert_eq!(solve_part_one(input), 86);
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
        assert_eq!(solve_part_one(input), 132);
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
        assert_eq!(solve_part_one(input), 136);
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
        assert_eq!(solve_part_one(input), 81);
    }

    #[test]
    fn example_f() {
        let input = [
            "#######", //
            "#a.#Cd#", //
            "##...##", //
            "##.@.##", //
            "##...##", //
            "#cB#Ab#", //
            "#######", //
        ]
        .join("\n");
        assert_eq!(solve_part_two(input), 8);
    }
    //
    #[test]
    fn example_g() {
        let input = [
            "###############", //
            "#d.ABC.#.....a#", //
            "######...######", //
            "######.@.######", //
            "######...######", //
            "#b.....#.....c#", //
            "###############", //
        ]
        .join("\n");
        assert_eq!(solve_part_two(input), 24);
    }

    #[test]
    fn example_h() {
        let input = [
            "#############", //
            "#DcBa.#.GhKl#", //
            "#.###...#I###", //
            "#e#d#.@.#j#k#", //
            "###C#...###J#", //
            "#fEbA.#.FgHi#", //
            "#############", //
        ]
        .join("\n");
        assert_eq!(solve_part_two(input), 32);
    }

    #[test]
    fn example_i() {
        let input = [
            "#############", //
            "#g#f.D#..h#l#", //
            "#F###e#E###.#", //
            "#dCba...BcIJ#", //
            "#####.@.#####", //
            "#nK.L...G...#", //
            "#M###N#H###.#", //
            "#o#m..#i#jk.#", //
            "#############", //
        ]
        .join("\n");
        assert_eq!(solve_part_two(input), 72);
    }
}
