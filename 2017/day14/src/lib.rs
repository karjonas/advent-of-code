extern crate year2017day10;

const GRID_SZ: usize = 128;

fn fill_recursive(island: usize, row: usize, col: usize, grid: &mut Vec<Vec<usize>>) {
    if grid[row][col] == 1 {
        grid[row][col] = island;
        if row + 1 < GRID_SZ {
            fill_recursive(island, row + 1, col, grid);
        }
        if row > 0 {
            fill_recursive(island, row - 1, col, grid);
        }
        if col + 1 < GRID_SZ {
            fill_recursive(island, row, col + 1, grid);
        }
        if col > 0 {
            fill_recursive(island, row, col - 1, grid);
        }
    }
}

pub fn solve(_filepath: &str) {
    let mut accum = 0;
    let mut grid = vec![vec![0; GRID_SZ]; GRID_SZ];

    for i in 0..GRID_SZ {
        let fmt = format!("jxqlasbh-{}", i);
        let hash = year2017day10::knot_hash(fmt.to_string(), true);
        let line: Vec<_> = hash.chars().map(|v| if v == '1' { 1 } else { 0 }).collect();

        assert!(line.len() == GRID_SZ);

        accum += line.iter().fold(0, |sum, v| sum + v);
        grid[i] = line;
    }

    let mut island_idx = 1; // Note: we start our first island at 2
    for r in 0..GRID_SZ {
        for c in 0..GRID_SZ {
            if grid[r][c] == 1 {
                island_idx += 1;
                fill_recursive(island_idx, r, c, &mut grid);
            }
        }
    }

    println!("Part one: {}", accum);
    println!("Part two: {}", island_idx - 1);
}
