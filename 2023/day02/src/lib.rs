#[derive(Debug, Clone, Copy)]
struct Turn {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(games_str: String) -> Vec<Vec<Turn>> {
    let mut games = Vec::new();
    for line in games_str.lines() {
        let mut game = Vec::new();
        for turn_str in line.split(':').nth(1).unwrap().split(';') {
            let mut turn = Turn {
                red: 0,
                green: 0,
                blue: 0,
            };
            for num_and_color in turn_str.split(',') {
                let num = num_and_color
                    .split_ascii_whitespace()
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                match num_and_color.split_ascii_whitespace().nth(1).unwrap() {
                    "red" => turn.red = num,
                    "green" => turn.green = num,
                    "blue" => turn.blue = num,
                    _ => panic!(),
                };
            }
            game.push(turn);
        }
        games.push(game);
    }

    return games;
}

fn part_one(games: &Vec<Vec<Turn>>) -> usize {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let mut game_id = 0;
    let mut sum = 0;
    for game in games {
        game_id = game_id + 1;
        let mut valid = true;
        for turn in game {
            if turn.red > MAX_RED || turn.green > MAX_GREEN || turn.blue > MAX_BLUE {
                valid = false;
                break;
            }
        }
        if valid {
            sum = sum + game_id;
        }
    }

    return sum;
}

fn part_two(games: &Vec<Vec<Turn>>) -> usize {
    let mut sum = 0;
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for turn in game {
            max_red = std::cmp::max(max_red, turn.red);
            max_green = std::cmp::max(max_green, turn.green);
            max_blue = std::cmp::max(max_blue, turn.blue);
        }
        sum = sum + (max_red * max_green * max_blue);
    }

    return sum;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let games = parse(input);

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}
