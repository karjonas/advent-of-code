extern crate common;

use std::collections::HashMap;
use std::collections::VecDeque;

type Board = [Vec<char>; 11];
const ROOM_POSITIONS: [usize; 4] = [2, 4, 6, 8];
const EMPTY: char = '.';

fn get_move_cost(c: char) -> usize {
    match c {
        'A' => return 1,
        'B' => return 10,
        'C' => return 100,
        'D' => return 1000,
        _ => panic!("fail"),
    }
}

fn get_goal_room(c: char) -> usize {
    match c {
        'A' => return 2,
        'B' => return 4,
        'C' => return 6,
        'D' => return 8,
        _ => panic!("fail"),
    }
}

fn is_reachable(board: &Board, start_pos: usize, end_pos: usize) -> bool {
    let a = std::cmp::min(start_pos, end_pos);
    let b = std::cmp::max(start_pos, end_pos);
    for pos in a..=b {
        if pos != start_pos && !ROOM_POSITIONS.contains(&pos) && board[pos][0] != EMPTY {
            return false;
        }
    }
    return true;
}

fn room_is_movable(board: &Board, piece: char, dest_pos: usize) -> bool {
    for &c in &board[dest_pos] {
        if c != piece && c != EMPTY {
            return false;
        }
    }
    return true;
}

fn get_top_unit_from_room(room: &Vec<char>) -> char {
    for &c in room {
        if c != EMPTY {
            return c;
        }
    }
    return EMPTY;
}

fn get_possible_moves(board: &Board, pos: usize) -> Vec<usize> {
    let room = &board[pos];

    if !ROOM_POSITIONS.contains(&pos) {
        let unit = room[0]; // Hallway, just take unit
        let goal_room = get_goal_room(unit);
        if is_reachable(board, pos, goal_room) && room_is_movable(board, unit, goal_room) {
            return [goal_room].to_vec();
        }
        return Vec::new();
    }

    let unit_to_move = get_top_unit_from_room(room);
    if pos == get_goal_room(unit_to_move) && room_is_movable(board, unit_to_move, pos) {
        // Already in correct room
        return Vec::new();
    }

    let mut possible = Vec::new();
    for dest in 0..board.len() {
        if dest == pos {
            continue;
        }
        if ROOM_POSITIONS.contains(&dest) && get_goal_room(unit_to_move) != dest {
            continue;
        }
        if get_goal_room(unit_to_move) == dest && !room_is_movable(board, unit_to_move, dest) {
            continue;
        }
        if is_reachable(board, pos, dest) {
            possible.push(dest);
        }
    }
    return possible;
}

fn add_unit_to_room(unit: char, room: &Vec<char>) -> (Vec<char>, usize) {
    let index = room.len() - room.iter().rev().position(|&r| r == EMPTY).unwrap() - 1;
    let mut result = room.clone();
    result[index] = unit;
    let dist = index + 1;
    return (result, dist);
}

fn move_unit(board: &Board, pos: usize, dest: usize) -> (Board, usize) {
    let mut new_board = board.clone();
    let mut dist = 0;
    let unit_to_move = get_top_unit_from_room(&board[pos]);

    if board[pos].len() == 1 {
        new_board[pos] = [EMPTY].to_vec();
    } else {
        let mut new_room = Vec::new();
        let mut found = false;
        for &c in &board[pos] {
            if c == EMPTY {
                dist += 1;
                new_room.push(c);
            } else if !found {
                new_room.push(EMPTY);
                dist += 1;
                found = true;
            } else {
                new_room.push(c);
            }
        }
        new_board[pos] = new_room;
    }

    dist += std::cmp::max(pos, dest) - std::cmp::min(pos, dest);

    if board[dest].len() == 1 {
        new_board[dest] = [unit_to_move].to_vec();
    } else {
        let (new_board_dest, addl_dist) = add_unit_to_room(unit_to_move, &board[dest]);
        new_board[dest] = new_board_dest;
        dist += addl_dist
    }

    return (new_board, dist * get_move_cost(unit_to_move));
}

fn solve_internal(board: &Board) -> HashMap<Board, usize> {
    let mut states = HashMap::new();
    states.insert(board.clone(), 0);

    let mut queue = VecDeque::from([board.clone()]);
    while !queue.is_empty() {
        let board = queue.pop_front().unwrap();
        for (pos, piece) in board.iter().enumerate() {
            if get_top_unit_from_room(piece) == EMPTY {
                continue;
            }
            for dest in get_possible_moves(&board, pos) {
                let (new_board, cost) = move_unit(&board, pos, dest);
                let old_cost = states.get(&board).unwrap();
                let new_cost = old_cost + cost;
                let cost = states.entry(new_board.clone()).or_insert(std::usize::MAX);

                if new_cost < *cost {
                    *cost = new_cost;
                    queue.push_back(new_board);
                }
            }
        }
    }
    return states;
}

fn parse_input(input: &String) -> Board {
    let chars: Vec<Vec<char>> = input.lines().map(|v| v.chars().collect()).collect();
    let mut board: Board = Default::default();

    board[0].push('.');
    board[1].push('.');
    board[2].push(chars[2][3]);
    board[2].push(chars[3][3]);
    board[3].push('.');
    board[4].push(chars[2][5]);
    board[4].push(chars[3][5]);
    board[5].push('.');
    board[6].push(chars[2][7]);
    board[6].push(chars[3][7]);
    board[7].push('.');
    board[8].push(chars[2][9]);
    board[8].push(chars[3][9]);
    board[9].push('.');
    board[10].push('.');

    return board;
}

fn get_goal_board(part_one: bool) -> Board {
    let depth = if part_one { 2 } else { 4 };
    let mut goal: Board = Default::default();
    goal[0] = ['.'].to_vec();
    goal[1] = ['.'].to_vec();
    goal[2] = std::iter::repeat('A').take(depth).collect::<Vec<_>>();
    goal[3] = ['.'].to_vec();
    goal[4] = std::iter::repeat('B').take(depth).collect::<Vec<_>>();
    goal[5] = ['.'].to_vec();
    goal[6] = std::iter::repeat('C').take(depth).collect::<Vec<_>>();
    goal[7] = ['.'].to_vec();
    goal[8] = std::iter::repeat('D').take(depth).collect::<Vec<_>>();
    goal[9] = ['.'].to_vec();
    goal[10] = ['.'].to_vec();
    return goal;
}

fn solve_internal_p1(input: &String) -> usize {
    let board = parse_input(&input);
    let boards = solve_internal(&board);
    let goal = get_goal_board(true);

    return *boards.get(&goal).unwrap();
}

fn solve_internal_p2(input: &String) -> usize {
    let mut board = parse_input(&input);
    board[2].insert(1, 'D');
    board[2].insert(2, 'D');
    board[4].insert(1, 'C');
    board[4].insert(2, 'B');
    board[6].insert(1, 'B');
    board[6].insert(2, 'A');
    board[8].insert(1, 'A');
    board[8].insert(2, 'C');

    let boards = solve_internal(&board);
    let goal = get_goal_board(false);
    return *boards.get(&goal).unwrap();
}

pub fn solve() {
    let input = common::read_file("2021/day23/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}
