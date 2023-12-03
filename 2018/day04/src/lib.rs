extern crate common;

use std::collections::HashMap;

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let mut lines: Vec<String> = input.lines().map(|v| v.to_string()).collect();
    lines.sort();

    let mut guards: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut curr_guard = -1;
    let mut sleep_time = 0;
    let mut guard_asleep: HashMap<i32, usize> = HashMap::new();

    for line in lines {
        let parts: Vec<_> = line.split(" ").collect();

        let time = parts[1].split(":").collect::<Vec<_>>()[1][..2]
            .to_string()
            .parse::<usize>()
            .unwrap();

        if parts[2] == "Guard" {
            let guard_id = parts[3][1..].to_string().parse::<i32>().unwrap();
            curr_guard = guard_id;
            guards.entry(guard_id).or_insert(common::zeroed_vector(60));
        }

        if parts[2] == "falls" {
            sleep_time = time;
        }

        if parts[2] == "wakes" {
            for i in sleep_time..time {
                guards.get_mut(&curr_guard).unwrap()[i] += 1;
                *guard_asleep.entry(curr_guard).or_insert(0) += 1;
            }
        }
    }

    {
        let mut sleepiest_guard = 0;
        let mut max_sleep = 0;
        for (guard_id, time_asleep) in &guard_asleep {
            if time_asleep.clone() > max_sleep {
                max_sleep = time_asleep.clone();
                sleepiest_guard = guard_id.clone();
            }
        }

        let mut best_minute = 0;
        let mut best_minute_value = 0;
        for i in 0..60 {
            let v = guards.get(&sleepiest_guard).unwrap()[i];
            if v > best_minute_value {
                best_minute_value = v;
                best_minute = i;
            }
        }
        println!("Part one: {}", (sleepiest_guard as usize) * best_minute);
    }

    {
        let mut sleepiest_guard = 0;
        let mut best_minute = 0;
        let mut best_minute_value = 0;
        for (guard_id, _) in guard_asleep {
            for i in 0..60 {
                let v = guards.get(&guard_id).unwrap()[i];
                if v > best_minute_value {
                    best_minute_value = v;
                    best_minute = i;
                    sleepiest_guard = guard_id;
                }
            }
        }

        println!("Part two: {}", (sleepiest_guard as usize) * best_minute);
    }
}
