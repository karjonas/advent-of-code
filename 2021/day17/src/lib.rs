extern crate common;
#[macro_use]
extern crate scan_fmt;

fn parse_input(input: &String) -> ((i32, i32), (i32, i32)) {
    let (x0, x1, y0, y1) =
        scan_fmt!(input, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();
    return ((x0, x1), (y0, y1));
}

fn solve_internal(input: &String) -> (i32, usize) {
    let ((x0, x1), (y0, y1)) = parse_input(input);

    let mut max_y = 0;
    let mut num_hits = 0;
    for x in -500..500 {
        for y in -500..500 {
            let mut hit = false;
            let mut curr_x = 0;
            let mut curr_y = 0;
            let mut speed_y = y;
            let mut speed_x = x;
            let mut highest_y = 0;
            loop {
                curr_x += speed_x;
                curr_y += speed_y;
                speed_x += if speed_x > 0 {
                    -1
                } else if speed_x < 0 {
                    1
                } else {
                    0
                };
                speed_y -= 1;
                highest_y = std::cmp::max(highest_y, curr_y);

                if curr_y < y0 || (speed_x == 0 && (curr_x < x0 || curr_x > x1)) {
                    break;
                }

                if curr_x >= x0 && curr_x <= x1 && curr_y >= y0 && curr_y <= y1 {
                    hit = true;
                    num_hits += 1;
                    break;
                }
            }

            if hit && highest_y >= max_y {
                max_y = highest_y;
            }
        }
    }
    return (max_y, num_hits);
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let (p0, p1) = solve_internal(&input);
    println!("Part one: {}", p0);
    println!("Part two: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(
            solve_internal(&String::from("target area: x=20..30, y=-10..-5")).0,
            45
        );
        assert_eq!(
            solve_internal(&String::from("target area: x=20..30, y=-10..-5")).1,
            112
        );
    }
}
