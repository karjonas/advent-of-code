extern crate common;

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect());
    }
    return result;
}

fn part_one(input: &String) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut keep_running = true;
    let mut num_steps = 0;
    while keep_running {
        keep_running = false;
        let mut row = 0;
        let mut grid_next = grid.clone();
        while row < rows {
            let mut col = 0;
            while col < cols {
                let col_next = (col + 1) % cols;

                if grid[row][col] == '>' && grid[row][col_next] == '.' {
                    // move right
                    grid_next[row][col_next] = '>';
                    grid_next[row][col] = '.';
                    col += 1;
                    keep_running = true;
                }
                col += 1;
            }
            row += 1;
        }

        grid = grid_next.clone();

        let mut col = 0;
        while col < cols {
            let mut row = 0;
            while row < rows {
                let row_next = (row + 1) % rows;
                if grid[row][col] == 'v' && grid[row_next][col] == '.' {
                    // move down
                    grid_next[row_next][col] = 'v';
                    grid_next[row][col] = '.';
                    row += 1;
                    keep_running = true;
                }
                row += 1;
            }
            col += 1;
        }
        grid = grid_next;
        num_steps += 1;
    }

    return num_steps;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    println!("Part one: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            .to_string();
        assert_eq!(part_one(&input), 58);
    }
}
