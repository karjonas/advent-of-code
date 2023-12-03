extern crate common;
extern crate doubly;

use doubly::DoublyLinkedList;

fn calc_insert_index(curr_marble_index: usize, ring_size: usize) -> usize {
    return 1 + ((curr_marble_index + 1) % ring_size);
}

fn solve_internal(input: &String, factor: usize) -> usize {
    let input: Vec<Vec<String>> = input
        .lines()
        .map(|v| v.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
    let num_players = input[0][0].parse::<usize>().unwrap();
    let last_worth = input[0][6].parse::<usize>().unwrap() * factor;

    let mut player_scores: Vec<usize> = common::filled_vector(num_players, 0);
    let mut board: DoublyLinkedList<usize> = DoublyLinkedList::new();
    let mut curr_marble_index = 0;
    let mut curr_worth = 0;

    board.push_back(0);
    while curr_worth < last_worth {
        for player in 0..num_players {
            curr_worth += 1;
            if (curr_worth % 23) == 0 {
                player_scores[player] += curr_worth;
                let remove_index = (curr_marble_index + board.len() - 7) % board.len();
                player_scores[player] += board.remove(remove_index);
                curr_marble_index = remove_index % board.len();
            } else {
                let insert_index = calc_insert_index(curr_marble_index, board.len());
                board.insert(insert_index, curr_worth);
                curr_marble_index = insert_index;
            }

            if curr_worth == last_worth {
                break;
            }
        }
    }

    let max_score = player_scores
        .iter()
        .fold(0, |sum, &v| std::cmp::max(sum, v));

    return max_score;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {:?}", solve_internal(&input, 1));
    println!("Part two: {:?}", solve_internal(&input, 100));
}
