extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Entry {
    enc_name: String,
    sector_id: i32,
    checksum: String,
}

fn valid_entry(entry: &Entry) -> bool {
    let mut hm = HashMap::new();
    for c in entry.enc_name.chars() {
        let hits = match hm.get(&c) {
            Some(&number) => number,
            _ => 0,
        };
        hm.insert(c, hits + 1);
    }

    let mut chars: Vec<char> = entry.enc_name.chars().collect();
    chars.sort();
    chars.dedup();
    chars.sort_by(|a, b| hm.get(b).unwrap().cmp(hm.get(a).unwrap()));

    let mut sorted_str = String::new();
    for i in 0..5 {
        sorted_str.push(chars[i]);
    }

    return sorted_str == entry.checksum;
}

fn convert_char(c: char, v: i32) -> char {
    let min_char = 'a' as i32;
    let max_char = 'z' as i32;
    let c_i = c as i32;

    return (((c_i - min_char + v) % (max_char - min_char + 1)) + min_char) as u8 as char;
}

fn decrypt_entry(entry: &Entry) -> String {
    let chars: Vec<char> = entry.enc_name.chars().collect();
    let dec = chars
        .iter()
        .map(|v| convert_char(v.clone(), entry.sector_id));

    let mut dec_str = String::new();
    for c in dec {
        dec_str.push(c.clone());
    }
    return dec_str;
}

pub fn solve() {
    let mut file = File::open("2016/day04/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"(.*)-(\w+)\[(\w+)\]").unwrap();

    let mut sum = 0;
    let mut npo_id = 0;

    for line in contents.lines() {
        let cap = re.captures(line).unwrap();

        let sect = cap[2].parse::<i32>().unwrap();
        let encd = cap[1].replace('-', "");
        let chks = cap[3].to_string();

        let entry = Entry {
            enc_name: encd,
            sector_id: sect,
            checksum: chks,
        };

        if valid_entry(&entry) {
            sum += entry.sector_id;
        }
        if decrypt_entry(&entry) == "northpoleobjectstorage" {
            npo_id = entry.sector_id;
        }
    }

    println!("Part 1: {}", sum);
    println!("part 2: {}", npo_id);
}
