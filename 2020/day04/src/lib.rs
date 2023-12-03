extern crate common;
#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;

type Passports = Vec<HashMap<String, String>>;

const KEYS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn check_byr(input: &String) -> bool {
    let v = input.parse::<usize>().unwrap_or(0);
    return v >= 1920 && v <= 2002;
}

fn check_iyr(input: &String) -> bool {
    let v = input.parse::<usize>().unwrap_or(0);
    return v >= 2010 && v <= 2020;
}

fn check_eyr(input: &String) -> bool {
    let v = input.parse::<usize>().unwrap_or(0);
    return v >= 2020 && v <= 2030;
}

fn check_hgt(input: &String) -> bool {
    let (len, unit) = scan_fmt!(input, "{d}{}", usize, String).unwrap_or((0, String::new()));
    return (unit == "cm" && len >= 150 && len <= 193) || (unit == "in" && len >= 59 && len <= 76);
}

fn check_hcl(input: &String) -> bool {
    return scan_fmt!(input, "#{6[a-z0-9]}", String).is_ok();
}

fn check_ecl(input: &String) -> bool {
    return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&input.as_str());
}

fn check_pid(input: &String) -> bool {
    let v = input.parse::<usize>().unwrap_or(0);
    return v != 0 && input.len() == 9;
}

fn check_cid(_input: &String) -> bool {
    return true;
}

fn parse_input(input: &String) -> Passports {
    let mut curr_map = HashMap::new();
    let mut passports = Passports::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(curr_map.clone());
            curr_map.clear();
        } else {
            for key_value in line.split_whitespace() {
                let (key, value) = scan_fmt!(key_value, "{}:{}", String, String).unwrap();
                curr_map.insert(key, value);
            }
        }
    }
    passports.push(curr_map.clone());
    return passports;
}

fn part_two(passports: &Passports) -> usize {
    return passports
        .iter()
        .map(|passport| {
            for key in KEYS.iter() {
                if !passport.contains_key(&String::from(*key)) {
                    return 0;
                }
                let value = passport.get(&String::from(*key)).unwrap();

                let valid = match *key {
                    "byr" => check_byr(value),
                    "iyr" => check_iyr(value),
                    "eyr" => check_eyr(value),
                    "hgt" => check_hgt(value),
                    "hcl" => check_hcl(value),
                    "ecl" => check_ecl(value),
                    "pid" => check_pid(value),
                    "cid" => check_cid(value),
                    _ => false,
                };

                if !valid {
                    return 0;
                }
            }
            return 1;
        })
        .sum();
}

fn part_one(passports: &Passports) -> usize {
    let mut num_valid = 0;
    for passport in passports {
        let mut ok = true;
        for key in KEYS.iter() {
            if !passport.contains_key(&String::from(*key)) {
                ok = false;
                break;
            }
        }
        num_valid += ok as usize;
    }
    return num_valid;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let grid = parse_input(&input);
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        let input = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .join("\n");
        assert_eq!(part_one(&parse_input(&input)), 2);
    }

    #[test]
    fn test_samples_part_two() {
        let input_invalid = [
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ]
        .join("\n");

        let input_valid = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .join("\n");

        assert_eq!(part_two(&parse_input(&input_invalid)), 0);
        assert_eq!(part_two(&parse_input(&input_valid)), 4);
    }
}
