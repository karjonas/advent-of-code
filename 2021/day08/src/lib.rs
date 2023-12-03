extern crate common;

#[derive(Debug, Clone)]
struct Entries {
    signals: Vec<Vec<String>>,
    outputs: Vec<Vec<String>>,
}

fn parse_input(input: &String) -> Entries {
    let mut prevs = Vec::new();
    let mut posts = Vec::new();

    for line in input.lines() {
        let tokens: Vec<_> = line.split(" ").collect();

        let mut is_prevs = true;
        let mut prevs_curr = Vec::new();
        let mut posts_curr = Vec::new();
        for token in tokens {
            if token == "|" {
                is_prevs = false;
                continue;
            }

            let mut chars: Vec<char> = String::from(token).chars().collect();
            chars.sort_by(|a, b| a.cmp(b));
            let as_string: String = chars.into_iter().collect();
            if is_prevs {
                prevs_curr.push(as_string.clone());
            } else {
                posts_curr.push(as_string.clone());
            }
        }

        prevs.push(prevs_curr);
        posts.push(posts_curr);
    }

    return Entries {
        signals: prevs,
        outputs: posts,
    };
}

fn contains_chars(original: &String, chars: &String) -> bool {
    for c in chars.chars() {
        if !original.contains(c) {
            return false;
        }
    }
    return true;
}

fn solve_internal_p1(input: &String) -> usize {
    let entries = parse_input(input);

    let mut ctr = 0;
    for output in entries.outputs {
        for digit in output {
            if [2, 3, 4, 7].contains(&digit.len()) {
                ctr += 1;
            }
        }
    }

    return ctr;
}

fn solve_internal_p2(input: &String) -> usize {
    let entries = parse_input(input);
    let mut sum = 0;

    for i in 0..entries.outputs.len() {
        let digits = &entries.signals[i];

        // easy
        let d1 = digits.iter().find(|v| v.len() == 2).unwrap();
        let d4 = digits.iter().find(|v| v.len() == 4).unwrap();
        let d7 = digits.iter().find(|v| v.len() == 3).unwrap();
        let d8 = digits.iter().find(|v| v.len() == 7).unwrap();

        // hard
        let d9 = digits
            .iter()
            .find(|v| v.len() == 6 && contains_chars(v, d4))
            .unwrap();
        let d0 = digits
            .iter()
            .find(|v| v.len() == 6 && *v != d9 && contains_chars(v, d1))
            .unwrap();
        let d6 = digits
            .iter()
            .find(|v| v.len() == 6 && *v != d9 && *v != d0)
            .unwrap();

        let d3 = digits
            .iter()
            .find(|v| v.len() == 5 && contains_chars(v, d1))
            .unwrap();
        let d5 = digits
            .iter()
            .find(|v| v.len() == 5 && *v != d3 && contains_chars(d9, *v))
            .unwrap();
        let d2 = digits
            .iter()
            .find(|v| v.len() == 5 && *v != d3 && *v != d5)
            .unwrap();

        let solution = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

        let mut value = 0;
        for digit in &entries.outputs[i] {
            let idx = solution.iter().position(|&d| *d == *digit).unwrap();
            value = 10 * value + idx;
        }
        sum += value;
    }

    return sum;
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
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(solve_internal_p1(&String::from(input)), 26);
        assert_eq!(solve_internal_p2(&String::from(input)), 61229);
    }
}
