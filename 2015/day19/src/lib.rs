extern crate common;

use std::collections::HashSet;

fn parse_input(input: &str) -> (String, Vec<(String, String)>) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut map = Vec::new();
    for i in 0..lines.len() - 2 {
        let words = lines[i].split_whitespace().collect::<Vec<_>>();
        let from = words[0].to_string();
        let to = words[2].to_string();
        map.push((from, to));
    }
    let source_text = lines[lines.len() - 1].to_string();

    return (source_text, map);
}

fn do_step(inputs: &HashSet<String>, map: &Vec<(String, String)>) -> HashSet<String> {
    let mut results = HashSet::new();
    for source_text in inputs {
        for (src, dst) in map {
            let mut start_idx = 0;
            loop {
                if start_idx >= source_text.len() {
                    break;
                }
                let slice = source_text.get(start_idx..).unwrap();
                match slice.find(src.as_str()) {
                    Some(idx) => {
                        let idx_adj = idx + start_idx;
                        let mut source_clone = source_text.clone();
                        for _i in 0..src.len() {
                            source_clone.remove(idx_adj);
                        }
                        source_clone.insert_str(idx_adj, dst.as_str());
                        results.insert(source_clone);
                        start_idx = idx_adj + 1;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
    }
    return results;
}

fn part_two(input: &str) -> usize {
    let (source_text, map_inv) = parse_input(input);
    let mut inputs = HashSet::new();
    inputs.insert(source_text.to_string());
    let mut map = Vec::new();
    for (src, dst) in map_inv {
        map.push((dst, src));
    }

    for i in 0..std::usize::MAX {
        let results = do_step(&inputs, &map);
        if results.contains("e") {
            return i + 1;
        }
        let mut inputs_new = HashSet::new();
        let mut shortest = std::usize::MAX;
        for result in results {
            let len = result.len();
            if len > shortest {
                break;
            } else if len < shortest {
                inputs_new.clear();
                shortest = len;
            }
            inputs_new.insert(result);
        }
        inputs = inputs_new;
    }
    return 0;
}

fn part_one(input: &str) -> usize {
    let (source_text, map) = parse_input(input);
    let mut inputs = HashSet::new();
    inputs.insert(source_text);
    let results = do_step(&inputs, &map);
    return results.len();
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", part_one(input.as_str()));
    println!("Part two: {}", part_two(input.as_str()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input_a = ["H => HO", "H => OH", "O => HH", "", "HOH"].join("\n");
        let input_b = ["H => HO", "H => OH", "O => HH", "", "HOHOHO"].join("\n");
        let input_c = [
            "e => H", "e => O", "H => HO", "H => OH", "O => HH", "", "HOHOHO",
        ]
        .join("\n");
        let input_d = [
            "e => H", "e => O", "H => HO", "H => OH", "O => HH", "", "HOH",
        ]
        .join("\n");

        assert_eq!(part_one(input_a.as_str()), 4);
        assert_eq!(part_one(input_b.as_str()), 7);
        assert_eq!(part_two(input_c.as_str()), 6);
        assert_eq!(part_two(input_d.as_str()), 3);
    }
}
