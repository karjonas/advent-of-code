extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;

type Edges = [usize; 4];
type Sides = Vec<Edges>;

#[derive(Debug, Clone)]
struct Square {
    id: usize,
    sides: Sides,
    originals: Vec<String>,
}

type Squares = Vec<Square>;

enum FlipMode {
    X,
    Y,
    CW,
}

const TILE_WIDTH_HEIGHT: usize = 10;

const SEA_MONSTER_UNITS: usize = 15;
const SEA_MONSTER_W: usize = 20;
const SEA_MONSTER_H: usize = 3;
const SEA_MONSTER: [[char; 20]; 3] = [
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        '#', ' ',
    ],
    [
        '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#',
        '#', '#',
    ],
    [
        ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ',
        ' ', ' ',
    ],
];

#[derive(Debug, Clone)]
struct SolverState {
    placed_ids: HashMap<(i32, i32), usize>,
    used_id: HashSet<usize>,
    id_to_sides: HashMap<usize, Edges>,
    id_to_original: HashMap<usize, String>,
}

fn get_sides(input: &String) -> (Sides, Vec<String>) {
    let mut sides = Sides::new();
    let mut originals = Vec::new();

    for permutation in permute_map(input) {
        let mut edges: [String; 4] = [String::new(), String::new(), String::new(), String::new()];
        let grid: Vec<Vec<char>> = permutation.lines().map(|v| v.chars().collect()).collect();
        let mut values: [usize; 4] = [0, 0, 0, 0];

        let width = grid.len();

        for i in 0..width {
            edges[0].push(grid[0][i]);
            edges[1].push(grid[i][width - 1]);
            edges[2].push(grid[width - 1][i]);
            edges[3].push(grid[i][0]);
        }

        // Store edges as number for quicker comparison
        for i in 0..4 {
            values[i] =
                usize::from_str_radix(edges[i].replace("#", "1").replace(".", "0").as_str(), 2)
                    .unwrap();
        }

        sides.push(values);
        originals.push(permutation);
    }

    return (sides, originals);
}

fn parse_input(input: &String) -> Squares {
    let mut squares = Squares::new();

    let mut grid_curr = String::new();
    let mut id = 0;
    for line in input.lines() {
        if line.is_empty() {
            let (sides, originals) = get_sides(&String::from(grid_curr.trim()));
            if !grid_curr.is_empty() {
                squares.push(Square {
                    id: id,
                    sides: sides,
                    originals: originals,
                });
            }
            grid_curr.clear();
            continue;
        }

        let tile_option = scan_fmt!(line, "Tile {d}:", usize);
        if tile_option.is_ok() {
            id = tile_option.unwrap();
            continue;
        }
        grid_curr.push_str(line);
        grid_curr.push('\n');
    }

    let (sides, originals) = get_sides(&grid_curr);
    squares.push(Square {
        id: id,
        sides: sides,
        originals: originals,
    });

    return squares;
}

fn solve_tiles(input: &Squares) -> SolverState {
    let width = (input.len() as f32).sqrt() as usize;
    assert_eq!(input.len(), width * width);

    // For quick lookup of matching tiles
    let mut square_matching_ids: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..input.len() {
        let side_values_i =
            common::flatten(input[i].sides.iter().map(|side| side.to_vec()).collect());

        for j in i + 1..input.len() {
            let side_values_j =
                common::flatten(input[j].sides.iter().map(|side| side.to_vec()).collect());

            for num in &side_values_i {
                if side_values_j.contains(num) {
                    square_matching_ids
                        .entry(input[i].id)
                        .or_default()
                        .insert(input[j].id);
                    square_matching_ids
                        .entry(input[j].id)
                        .or_default()
                        .insert(input[i].id);
                    break;
                }
            }
        }
    }

    // Permute all square orientations in one vector
    let mut squares_permutated = Vec::new();
    for square in input {
        for i in 0..square.sides.len() {
            squares_permutated.push((
                square.id,
                square.sides[i].clone(),
                square.originals[i].clone(),
            ));
        }
    }

    let mut states = Vec::new();
    for (id, sides, original) in &squares_permutated {
        let mut initial = SolverState {
            placed_ids: HashMap::new(),
            used_id: HashSet::new(),
            id_to_sides: HashMap::new(),
            id_to_original: HashMap::new(),
        };
        initial.placed_ids.insert((0, 0), *id);
        initial.id_to_sides.insert(*id, sides.clone());
        initial.used_id.insert(*id);
        initial.id_to_original.insert(*id, original.clone());
        states.push(initial);
    }
    for y in 0..width as i32 {
        for x in 0..width as i32 {
            if (x, y) == (0, 0) {
                continue;
            }
            let mut states_new = Vec::new();
            for state in states {
                let matching_ids = if state.placed_ids.contains_key(&(x - 1, y)) {
                    square_matching_ids
                        .get(state.placed_ids.get(&(x - 1, y)).unwrap())
                        .unwrap()
                } else {
                    square_matching_ids
                        .get(state.placed_ids.get(&(x, y - 1)).unwrap())
                        .unwrap()
                };

                // Go through all squares and see if it can match
                for (id, sides, original) in &squares_permutated {
                    if state.used_id.contains(id) || !matching_ids.contains(id) {
                        continue;
                    }

                    // Check if square is ok in this position
                    let mut ok = true;
                    for (x_n, y_n, my_side, other_side) in [
                        (x, y - 1, 0, 2),
                        (x + 1, y, 1, 3),
                        (x, y + 1, 2, 0),
                        (x - 1, y, 3, 1),
                    ]
                    .iter()
                    {
                        let key = &(*x_n, *y_n);
                        if state.placed_ids.contains_key(key) {
                            let neigh_id = *state.placed_ids.get(key).unwrap();
                            if state.id_to_sides.get(&neigh_id).unwrap()[*other_side]
                                != sides[*my_side]
                            {
                                ok = false;
                                break;
                            }
                        }
                    }

                    // Square fits, insert and add as possible state
                    if ok {
                        let mut state_new = state.clone();
                        state_new.placed_ids.insert((x, y), *id);
                        state_new.id_to_sides.insert(*id, sides.clone());
                        state_new.used_id.insert(*id);
                        state_new.id_to_original.insert(*id, original.clone());
                        states_new.push(state_new);
                    }
                }
            }
            states = states_new;
        }
    }

    // return first matching state
    return states[0].clone();
}

fn get_map(
    input: &Squares,
    tile_positions: HashMap<(i32, i32), usize>,
    id_to_original: HashMap<usize, String>,
) -> String {
    let width = (input.len() as f32).sqrt() as usize;

    let mut map = String::new();
    for tile_row in 0..width {
        for y in 1..TILE_WIDTH_HEIGHT - 1 {
            for tile_col in 0..width {
                let tile_id = tile_positions
                    .get(&(tile_col as i32, tile_row as i32))
                    .unwrap();

                let tile_original: Vec<Vec<char>> = id_to_original
                    .get(tile_id)
                    .unwrap()
                    .lines()
                    .map(|v| v.chars().collect())
                    .collect();

                for x in 1..TILE_WIDTH_HEIGHT - 1 {
                    map.push(tile_original[y][x]);
                }
            }
            map.push('\n');
        }
    }

    return map;
}

fn flip_map(input: &String, mode: FlipMode) -> String {
    let original: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let width = original.len();

    let mut flipped = original.clone();
    for y in 0..width {
        for x in 0..width {
            match mode {
                FlipMode::CW => flipped[x][y] = original[width - 1 - y][x],
                FlipMode::X => flipped[x][y] = original[width - 1 - x][y],
                FlipMode::Y => flipped[x][y] = original[x][width - 1 - y],
            };
        }
    }

    return flipped
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn permute_map(input: &String) -> Vec<String> {
    let mut maps: Vec<String> = Vec::new();
    maps.push(input.clone());
    for i in 1..4 {
        maps.push(flip_map(&maps[i - 1], FlipMode::CW));
    }

    for i in 0..4 {
        maps.push(flip_map(&maps[i], FlipMode::Y));
        maps.push(flip_map(&maps[i], FlipMode::X));
    }

    return maps
        .into_iter()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
}

fn part_one(tile_positions: &HashMap<(i32, i32), usize>, width: usize) -> usize {
    let mut prod = 1;
    for (x, y) in [
        (0, 0),
        (width - 1, 0),
        (0, width - 1),
        (width - 1, width - 1),
    ]
    .iter()
    {
        prod = prod * tile_positions.get(&(*x as i32, *y as i32)).unwrap();
    }

    return prod;
}

fn part_two(squares: &Vec<Square>, solved_state: &SolverState) -> usize {
    let map = get_map(
        squares,
        solved_state.placed_ids.clone(),
        solved_state.id_to_original.clone(),
    );
    let maps = permute_map(&map);

    let find_monster = |x: usize, y: usize, grid: &Vec<Vec<char>>| -> bool {
        for y_m in 0..SEA_MONSTER_H {
            for x_m in 0..SEA_MONSTER_W {
                if SEA_MONSTER[y_m][x_m] == '#' && grid[y + y_m][x + x_m] != '#' {
                    return false;
                }
            }
        }
        return true;
    };

    for map in &maps {
        let grid: Vec<Vec<char>> = map.lines().map(|v| v.chars().collect()).collect();
        let width = grid[0].len();
        let height = grid.len();
        let mut sum = 0;

        for x in 0..width - SEA_MONSTER_W {
            for y in 0..height - SEA_MONSTER_H {
                if find_monster(x, y, &grid) {
                    sum += 1;
                }
            }
        }

        if sum > 0 {
            let num_hashes: usize = grid
                .iter()
                .map(|line| line.iter().filter(|v| **v == '#').count())
                .sum();
            return num_hashes - sum * SEA_MONSTER_UNITS;
        }
    }

    return 0;
}

pub fn solve(filepath: &str) {
    let squares = parse_input(
        &std::fs::read_to_string(filepath)
            .unwrap()
            .trim()
            .to_string(),
    );
    let solved_state = solve_tiles(&squares);
    let width = (squares.len() as f32).sqrt() as usize;

    println!("Part one: {}", part_one(&solved_state.placed_ids, width));
    println!("Part two: {}", part_two(&squares, &solved_state));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let tile_2311 = [
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
        ]
        .join("\n");

        let tile_1951 = [
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
        ]
        .join("\n");
        let tile_1171 = [
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
        ]
        .join("\n");
        let tile_1427 = [
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
        ]
        .join("\n");
        let tile_1489 = [
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
        ]
        .join("\n");
        let tile_2473 = [
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
        ]
        .join("\n");
        let tile_2971 = [
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
        ]
        .join("\n");
        let tile_2729 = [
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
        ]
        .join("\n");
        let tile_3079 = [
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
        ]
        .join("\n");

        let input_mini_a = [
            tile_1951.clone(),
            tile_2311.clone(),
            tile_2729.clone(),
            tile_1427.clone(),
        ]
        .join("\n\n");
        let input_mini_b = [
            tile_3079.clone(),
            tile_2311.clone(),
            tile_2473.clone(),
            tile_1427.clone(),
        ]
        .join("\n\n");
        let input_mini_c = [
            tile_2971.clone(),
            tile_1489.clone(),
            tile_2729.clone(),
            tile_1427.clone(),
        ]
        .join("\n\n");
        let input_mini_d = [
            tile_2473.clone(),
            tile_1489.clone(),
            tile_1171.clone(),
            tile_1427.clone(),
        ]
        .join("\n\n");

        let input = [
            tile_1171.clone(),
            tile_1427.clone(),
            tile_1489.clone(),
            tile_1951.clone(),
            tile_2311.clone(),
            tile_2473.clone(),
            tile_2729.clone(),
            tile_2971.clone(),
            tile_3079.clone(),
        ]
        .join("\n\n");

        {
            let squares = parse_input(&input_mini_a);
            let solved_state = solve_tiles(&squares);
            let width = (squares.len() as f32).sqrt() as usize;

            assert_eq!(
                part_one(&solved_state.placed_ids, width),
                1951 * 2311 * 2729 * 1427
            );
        }

        {
            let squares = parse_input(&input_mini_b);
            let solved_state = solve_tiles(&squares);
            let width = (squares.len() as f32).sqrt() as usize;
            assert_eq!(
                part_one(&solved_state.placed_ids, width),
                3079 * 2311 * 2473 * 1427
            );
        }

        {
            let squares = parse_input(&input_mini_c);
            let solved_state = solve_tiles(&squares);
            let width = (squares.len() as f32).sqrt() as usize;
            assert_eq!(
                part_one(&solved_state.placed_ids, width),
                2971 * 1489 * 2729 * 1427
            );
        }

        {
            let squares = parse_input(&input_mini_d);
            let solved_state = solve_tiles(&squares);
            let width = (squares.len() as f32).sqrt() as usize;
            assert_eq!(
                part_one(&solved_state.placed_ids, width),
                2473 * 1489 * 1171 * 1427
            );
        }

        {
            let squares = parse_input(&input);
            let solved_state = solve_tiles(&squares);
            let width = (squares.len() as f32).sqrt() as usize;
            assert_eq!(part_one(&solved_state.placed_ids, width), 20899048083289);
            assert_eq!(part_two(&squares, &solved_state), 273);
        }
    }
}
