#[derive(Debug)]
struct AABB {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn as_digit(c: char) -> i32 {
    return c as i32 - '0' as i32;
}

fn overlaps(a: &AABB, b: &AABB) -> bool {
    return (a.x0 < b.x1 && a.x1 > b.x0) && (a.y0 < b.y1 && a.y1 > b.y0);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let grid: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let _sum = 0;
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for y in 0..height {
        assert!(grid[y as usize].len() == width as usize);
        let mut x = -1;
        while x < (width - 1) {
            x = x + 1;
            if !is_digit(grid[y as usize][x as usize]) {
                continue;
            }

            let x0 = x;
            let mut curr_number = 0;
            for x_n in x..width {
                x = x_n;
                let c_n = grid[y as usize][x_n as usize];
                if !is_digit(c_n) {
                    break;
                }

                curr_number = curr_number * 10 + as_digit(c_n);
            }

            let aabb = AABB {
                x0: x0,
                y0: y,
                x1: x,
                y1: y + 1,
            };
            numbers.push((curr_number, aabb));
        }
    }

    for y in 0..height {
        for x in 0..width {
            let c = grid[y as usize][x as usize];
            if is_digit(c) || c == '.' {
                continue;
            }
            // symbol found
            let aabb = AABB {
                x0: x - 1,
                x1: x + 2,
                y0: y - 1,
                y1: y + 2,
            };
            symbols.push((c, aabb));
        }
    }

    let mut sum = 0;
    for (number, aabb_number) in &numbers {
        for (_symbol, aabb_symbol) in &symbols {
            if overlaps(aabb_number, aabb_symbol) {
                sum += number;
                break;
            }
        }
    }

    let part_one = sum;

    sum = 0;
    for (symbol, aabb_symbol) in &symbols {
        if *symbol != '*' {
            continue;
        }
        let mut gear_a = 0;
        let mut gear_b = 0;
        for (number, aabb_number) in &numbers {
            if overlaps(aabb_number, aabb_symbol) {
                if gear_a == 0 {
                    gear_a = *number
                } else if gear_b == 0 {
                    gear_b = *number
                } else {
                    gear_a = 0;
                    gear_b = 0;
                    break;
                }
            }
        }
        sum += gear_a * gear_b;
    }

    let part_two = sum;

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_one(input), 4361);
    }
}
