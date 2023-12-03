extern crate common;

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<usize>>,
    cols: Vec<Vec<usize>>,
}
fn parse_boards(input: &String) -> (Vec<usize>, Vec<Board>) {
    let mut lines = input
        .lines()
        .map(|v| String::from(v.trim().replace("  ", " ")))
        .collect::<Vec<_>>();
    let numbers = lines[0]
        .split(",")
        .map(|v| common::string_to_usize(v))
        .collect::<Vec<usize>>();
    lines.rotate_left(2);
    lines.pop();
    lines.pop();

    let mut boards = Vec::<Board>::new();

    let mut offset = 0;
    while offset < lines.len() {
        let rows = vec![
            lines[offset + 0]
                .split(" ")
                .map(|v| common::string_to_usize(v.trim()))
                .collect::<Vec<_>>(),
            lines[offset + 1]
                .split(" ")
                .map(|v| common::string_to_usize(v.trim()))
                .collect::<Vec<_>>(),
            lines[offset + 2]
                .split(" ")
                .map(|v| common::string_to_usize(v.trim()))
                .collect::<Vec<_>>(),
            lines[offset + 3]
                .split(" ")
                .map(|v| common::string_to_usize(v.trim()))
                .collect::<Vec<_>>(),
            lines[offset + 4]
                .split(" ")
                .map(|v| common::string_to_usize(v.trim()))
                .collect::<Vec<_>>(),
        ];

        let cols = vec![
            vec![rows[0][0], rows[1][0], rows[2][0], rows[3][0], rows[4][0]],
            vec![rows[0][1], rows[1][1], rows[2][1], rows[3][1], rows[4][1]],
            vec![rows[0][2], rows[1][2], rows[2][2], rows[3][2], rows[4][2]],
            vec![rows[0][3], rows[1][3], rows[2][3], rows[3][3], rows[4][3]],
            vec![rows[0][4], rows[1][4], rows[2][4], rows[3][4], rows[4][4]],
        ];

        boards.push(Board {
            rows: rows,
            cols: cols,
        });
        offset += 6;
    }

    return (numbers, boards);
}

fn num_common(a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut ctr = 0;
    for v in a {
        ctr += if b.contains(v) { 1 } else { 0 }
    }
    return ctr;
}

fn calc_score(numbers: &Vec<usize>, board: &Board) -> usize {
    let mut sum = 0;
    for row in &board.rows {
        for v in row {
            sum += if !numbers.contains(v) { *v } else { 0 }
        }
    }

    return sum * numbers.last().unwrap();
}

fn check_board(numbers: &Vec<usize>, board: &Board) -> usize {
    for row in &board.rows {
        if num_common(numbers, row) == row.len() {
            return calc_score(numbers, board);
        }
    }
    for col in &board.cols {
        if num_common(numbers, col) == col.len() {
            return calc_score(numbers, board);
        }
    }

    return 0;
}

fn solve_boards(numbers: &Vec<usize>, boards: &Vec<Board>) -> usize {
    let mut numbers_drawn = Vec::<usize>::new();
    for &number in numbers {
        numbers_drawn.push(number);
        for board in boards {
            let result = check_board(&numbers_drawn, board);
            if result > 0 {
                return result;
            }
        }
    }

    return 0;
}

fn solve_internal_p1(input: &String) -> usize {
    let (numbers, boards) = parse_boards(input);
    return solve_boards(&numbers, &boards);
}

fn solve_internal_p2(input: &String) -> usize {
    let (numbers, mut boards) = parse_boards(input);
    let mut numbers_drawn = Vec::<usize>::new();
    for number in numbers {
        numbers_drawn.push(number);
        let mut ctr = 0;
        while ctr < boards.len() {
            let board = &boards[ctr];
            let result = check_board(&numbers_drawn, &board);
            if result > 0 {
                if boards.len() > 1 {
                    boards.remove(ctr);
                } else {
                    return result;
                }
            } else {
                ctr += 1;
            }
        }
    }

    return 0;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        assert_eq!(solve_internal_p1(&String::from(input)), 4512);
        assert_eq!(solve_internal_p2(&String::from(input)), 1924);
    }
}
