extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const MAXROBOTS: usize = 1000;

pub fn solve() {
    let mut file = File::open("2016/day10/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let regex_goes = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let regex_bot_gives =
        Regex::new(r"bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)").unwrap();

    let mut bot_source_low = vec![-1; MAXROBOTS];
    let mut bot_source_high = vec![-1; MAXROBOTS];

    let mut bot_dest_low = vec![-1; MAXROBOTS];
    let mut bot_dest_high = vec![-1; MAXROBOTS];

    let mut bot_dest_output_low = vec![-1; MAXROBOTS];
    let mut bot_dest_output_high = vec![-1; MAXROBOTS];

    let mut bot_value_low = vec![std::i32::MAX; MAXROBOTS];
    let mut bot_value_high = vec![std::i32::MIN; MAXROBOTS];

    let mut out_value = vec![-1; MAXROBOTS];

    let mut bot_done = vec![false; MAXROBOTS];
    let mut bot_num_inputs = vec![0; MAXROBOTS];

    let mut max_idx = 0;

    for line in contents.lines() {
        let mut ok = false;
        match regex_goes.captures(line) {
            Some(cap) => {
                let value = cap[1].parse::<i32>().unwrap();
                let bot = cap[2].parse::<usize>().unwrap();
                bot_value_low[bot] = std::cmp::min(bot_value_low[bot], value);
                bot_value_high[bot] = std::cmp::max(bot_value_high[bot], value);
                max_idx = std::cmp::max(max_idx, bot);
                bot_num_inputs[bot] = bot_num_inputs[bot] + 1;
                ok = true;
            }
            None => (),
        }

        match regex_bot_gives.captures(line) {
            Some(cap) => {
                let from = cap[1].parse::<i32>().unwrap();
                let type_low = cap[2].to_string();
                let to_low = cap[3].parse::<i32>().unwrap();
                let type_high = cap[4].to_string();
                let to_high = cap[5].parse::<i32>().unwrap();

                assert!(type_low == "bot" || type_low == "output");
                assert!(type_high == "bot" || type_high == "output");

                if type_low == "bot" {
                    bot_source_low[to_low as usize] = from;
                    bot_dest_low[from as usize] = to_low;
                } else {
                    bot_dest_output_low[from as usize] = to_low;
                }

                if type_high == "bot" {
                    bot_source_high[to_high as usize] = from;
                    bot_dest_high[from as usize] = to_high;
                } else {
                    bot_dest_output_high[from as usize] = to_low;
                }
                max_idx = std::cmp::max(max_idx, to_high as usize);
                max_idx = std::cmp::max(max_idx, to_low as usize);
                max_idx = std::cmp::max(max_idx, from as usize);
                ok = true;
            }
            None => (),
        }
        assert!(ok);
    }

    let mut any_change = true;
    while any_change {
        any_change = false;
        for bot in 0..(max_idx + 1) {
            if bot_done[bot] {
                continue;
            }

            let low = bot_value_low[bot];
            let hgh = bot_value_high[bot];
            let dst_low = bot_dest_low[bot];
            let dst_hgh = bot_dest_high[bot];

            let dst_low_out = bot_dest_output_low[bot];
            let dst_hgh_out = bot_dest_output_high[bot];

            assert!(bot_num_inputs[bot] < 3);

            if bot_num_inputs[bot] == 2 {
                if dst_low != -1 {
                    bot_value_low[dst_low as usize] =
                        std::cmp::min(bot_value_low[dst_low as usize], low);
                    bot_value_high[dst_low as usize] =
                        std::cmp::max(bot_value_high[dst_low as usize], low);

                    bot_num_inputs[dst_low as usize] = bot_num_inputs[dst_low as usize] + 1;
                }
                if dst_hgh != -1 {
                    bot_value_low[dst_hgh as usize] =
                        std::cmp::min(bot_value_low[dst_hgh as usize], hgh);
                    bot_value_high[dst_hgh as usize] =
                        std::cmp::max(bot_value_high[dst_hgh as usize], hgh);

                    bot_num_inputs[dst_hgh as usize] = bot_num_inputs[dst_hgh as usize] + 1;
                }

                if dst_low_out != -1 {
                    out_value[dst_low_out as usize] = low;
                }

                if dst_hgh_out != -1 {
                    out_value[dst_hgh_out as usize] = hgh;
                }

                bot_done[bot] = true;
                any_change = true;
            }
        }
    }

    let fnd_min = 17;
    let fnd_max = 61;
    let mut bt_fnd = 0;
    for bot in 0..(max_idx + 1) {
        if bot_value_low[bot] == fnd_min && bot_value_high[bot] == fnd_max {
            bt_fnd = bot;
        }
    }

    println!("Part 1: {}", bt_fnd);
    println!("Part 2: {}", out_value[0] * out_value[1] * out_value[2]);
}
