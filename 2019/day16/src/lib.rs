extern crate common;

fn generate_pattern(phases: usize) -> Vec<Vec<i64>> {
    let base_pattern = [0, 1, 0, -1];
    let mut output = Vec::new();
    output.resize(phases, Vec::new());

    for phase in 0..phases {
        for idx in 0..4 {
            for _i in 0..(phase + 1) {
                output[phase].push(base_pattern[idx]);
            }
        }
    }
    return output;
}

fn calc_fft(input: String, phases: usize) -> usize {
    let mut numbers: Vec<i64> = input
        .chars()
        .map(|v| common::char_to_u8(v) as i64)
        .collect();
    let num_numbers = numbers.len();
    let base_patterns = generate_pattern(num_numbers);

    for _phase_idx in 0..phases {
        let mut numbers_new = numbers.clone();
        for number_idx in 0..num_numbers {
            let pattern = &base_patterns[number_idx];
            let pattern_size = pattern.len();

            let mut number_new = 0;
            for i in 0..num_numbers {
                number_new += numbers[i] * pattern[(i + 1) % pattern_size];
            }
            numbers_new[number_idx] = number_new.abs() % 10;
        }
        numbers = numbers_new;
    }

    let mut s = String::new();
    for i in 0..8 {
        let c = (numbers[i] as u8 + ('0' as u8)) as char;
        s.push(c);
    }

    return common::string_to_usize(s.as_str());
}

fn calc_fft_p2(input: String, phases: usize) -> usize {
    let mut numbers: Vec<i64> = input
        .chars()
        .map(|v| common::char_to_u8(v) as i64)
        .collect();
    let num_numbers = numbers.len();
    for _phase_idx in 0..phases {
        let mut numbers_new = numbers.clone();
        let mut sum: i64 = numbers.iter().sum();
        for number_idx in 0..num_numbers {
            numbers_new[number_idx] = sum.abs() % 10;
            sum -= numbers[number_idx];
        }
        numbers = numbers_new;
    }

    let mut s = String::new();
    for i in 0..8 {
        let c = (numbers[i] as u8 + ('0' as u8)) as char;
        s.push(c);
    }

    return common::string_to_usize(s.as_str());
}

fn solve_part_two(input: String) -> usize {
    let mut s = String::new();
    for _ in 0..10_000 {
        s += input.as_str();
    }
    let offset = common::string_to_usize(&s[..7]);
    s = String::from(&s[offset..]);
    return calc_fft_p2(s, 100);
}

pub fn solve() {
    let input = common::read_file("2019/day16/input");
    println!("Part one: {}", calc_fft(String::from(&input), 100));
    println!("Part two: {}", solve_part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_pattern_test() {
        let v = generate_pattern(4);
        assert_eq!(v[0], [0, 1, 0, -1]);
        assert_eq!(v[1], [0, 0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(v[2], [0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }

    #[test]
    fn part_one_a() {
        let v = calc_fft(String::from("12345678"), 1);
        assert_eq!(v, 48226158);
    }
    #[test]
    fn part_one_b() {
        let v = calc_fft(String::from("12345678"), 2);
        assert_eq!(v, 34040438);
    }
    #[test]
    fn part_one_c() {
        let v = calc_fft(String::from("12345678"), 3);
        assert_eq!(v, 03415518);
    }
    #[test]
    fn part_one_d() {
        let v = calc_fft(String::from("12345678"), 4);
        assert_eq!(v, 01029498);
    }
    #[test]
    fn part_one_e() {
        let v = calc_fft(String::from("80871224585914546619083218645595"), 100);
        assert_eq!(v, 24176176);
    }
    #[test]
    fn part_one_f() {
        let v = calc_fft(String::from("19617804207202209144916044189917"), 100);
        assert_eq!(v, 73745418);
    }
    #[test]
    fn part_one_g() {
        let v = calc_fft(String::from("69317163492948606335995924319873"), 100);
        assert_eq!(v, 52432133);
    }

    #[test]
    fn part_two_a() {
        let v = solve_part_two(String::from("03036732577212944063491565474664"));
        assert_eq!(v, 84462026);
    }

    #[test]
    fn part_two_b() {
        let v = solve_part_two(String::from("02935109699940807407585447034323"));
        assert_eq!(v, 78725270);
    }
    #[test]
    fn part_two_c() {
        let v = solve_part_two(String::from("03081770884921959731165446850517"));
        assert_eq!(v, 53553731);
    }
}
