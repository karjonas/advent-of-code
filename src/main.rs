extern crate argh;

use argh::FromArgs;

fn get_filepath(year: usize, day: usize, path: &str) -> String {
    if path.is_empty() {
        format!("{}/day{:02}/input", year, day)
    } else {
        path.to_string()
    }
}

fn solve_year_2015(day: usize, path: &str) {
    let filepath = get_filepath(2015, day, path);
    match day {
        1 => year2015day01::solve(&filepath.as_str()),
        2 => year2015day02::solve(&filepath.as_str()),
        3 => year2015day03::solve(&filepath.as_str()),
        4 => year2015day04::solve(&filepath.as_str()),
        5 => year2015day05::solve(&filepath.as_str()),
        6 => year2015day06::solve(&filepath.as_str()),
        7 => year2015day07::solve(&filepath.as_str()),
        8 => year2015day08::solve(&filepath.as_str()),
        9 => year2015day09::solve(&filepath.as_str()),
        10 => year2015day10::solve(&filepath.as_str()),
        11 => year2015day11::solve(&filepath.as_str()),
        12 => year2015day12::solve(&filepath.as_str()),
        13 => year2015day13::solve(&filepath.as_str()),
        14 => year2015day14::solve(&filepath.as_str()),
        15 => year2015day15::solve(&filepath.as_str()),
        16 => year2015day16::solve(&filepath.as_str()),
        17 => year2015day17::solve(&filepath.as_str()),
        18 => year2015day18::solve(&filepath.as_str()),
        19 => year2015day19::solve(&filepath.as_str()),
        20 => year2015day20::solve(&filepath.as_str()),
        21 => year2015day21::solve(&filepath.as_str()),
        22 => year2015day22::solve(&filepath.as_str()),
        23 => year2015day23::solve(&filepath.as_str()),
        24 => year2015day24::solve(&filepath.as_str()),
        25 => year2015day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2016(day: usize, path: &str) {
    let filepath = get_filepath(2016, day, path);
    match day {
        1 => year2016day01::solve(&filepath.as_str()),
        2 => year2016day02::solve(&filepath.as_str()),
        3 => year2016day03::solve(&filepath.as_str()),
        4 => year2016day04::solve(&filepath.as_str()),
        5 => year2016day05::solve(&filepath.as_str()),
        6 => year2016day06::solve(&filepath.as_str()),
        7 => year2016day07::solve(&filepath.as_str()),
        8 => year2016day08::solve(&filepath.as_str()),
        9 => year2016day09::solve(&filepath.as_str()),
        10 => year2016day10::solve(&filepath.as_str()),
        11 => year2016day11::solve(&filepath.as_str()),
        12 => year2016day12::solve(&filepath.as_str()),
        13 => year2016day13::solve(&filepath.as_str()),
        14 => year2016day14::solve(&filepath.as_str()),
        15 => year2016day15::solve(&filepath.as_str()),
        16 => year2016day16::solve(&filepath.as_str()),
        17 => year2016day17::solve(&filepath.as_str()),
        18 => year2016day18::solve(&filepath.as_str()),
        19 => year2016day19::solve(&filepath.as_str()),
        20 => year2016day20::solve(&filepath.as_str()),
        21 => year2016day21::solve(&filepath.as_str()),
        22 => year2016day22::solve(&filepath.as_str()),
        23 => year2016day23::solve(&filepath.as_str()),
        24 => year2016day24::solve(&filepath.as_str()),
        25 => year2016day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2017(day: usize, path: &str) {
    let filepath = get_filepath(2017, day, path);
    match day {
        1 => year2017day01::solve(&filepath.as_str()),
        2 => year2017day02::solve(&filepath.as_str()),
        3 => year2017day03::solve(&filepath.as_str()),
        4 => year2017day04::solve(&filepath.as_str()),
        5 => year2017day05::solve(&filepath.as_str()),
        6 => year2017day06::solve(&filepath.as_str()),
        7 => year2017day07::solve(&filepath.as_str()),
        8 => year2017day08::solve(&filepath.as_str()),
        9 => year2017day09::solve(&filepath.as_str()),
        10 => year2017day10::solve(&filepath.as_str()),
        11 => year2017day11::solve(&filepath.as_str()),
        12 => year2017day12::solve(&filepath.as_str()),
        13 => year2017day13::solve(&filepath.as_str()),
        14 => year2017day14::solve(&filepath.as_str()),
        15 => year2017day15::solve(&filepath.as_str()),
        16 => year2017day16::solve(&filepath.as_str()),
        17 => year2017day17::solve(&filepath.as_str()),
        18 => year2017day18::solve(&filepath.as_str()),
        19 => year2017day19::solve(&filepath.as_str()),
        20 => year2017day20::solve(&filepath.as_str()),
        21 => year2017day21::solve(&filepath.as_str()),
        22 => year2017day22::solve(&filepath.as_str()),
        23 => year2017day23::solve(&filepath.as_str()),
        24 => year2017day24::solve(&filepath.as_str()),
        25 => year2017day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2018(day: usize, path: &str) {
    let filepath = get_filepath(2018, day, path);
    match day {
        1 => year2018day01::solve(&filepath.as_str()),
        2 => year2018day02::solve(&filepath.as_str()),
        3 => year2018day03::solve(&filepath.as_str()),
        4 => year2018day04::solve(&filepath.as_str()),
        5 => year2018day05::solve(&filepath.as_str()),
        6 => year2018day06::solve(&filepath.as_str()),
        7 => year2018day07::solve(&filepath.as_str()),
        8 => year2018day08::solve(&filepath.as_str()),
        9 => year2018day09::solve(&filepath.as_str()),
        10 => year2018day10::solve(&filepath.as_str()),
        11 => year2018day11::solve(&filepath.as_str()),
        12 => year2018day12::solve(&filepath.as_str()),
        13 => year2018day13::solve(&filepath.as_str()),
        14 => year2018day14::solve(&filepath.as_str()),
        15 => year2018day15::solve(&filepath.as_str()),
        16 => year2018day16::solve(&filepath.as_str()),
        17 => year2018day17::solve(&filepath.as_str()),
        18 => year2018day18::solve(&filepath.as_str()),
        19 => year2018day19::solve(&filepath.as_str()),
        20 => year2018day20::solve(&filepath.as_str()),
        21 => year2018day21::solve(&filepath.as_str()),
        22 => year2018day22::solve(&filepath.as_str()),
        23 => year2018day23::solve(&filepath.as_str()),
        24 => year2018day24::solve(&filepath.as_str()),
        25 => year2018day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2019(day: usize, path: &str) {
    let filepath = get_filepath(2019, day, path);
    match day {
        1 => year2019day01::solve(&filepath.as_str()),
        2 => year2019day02::solve(&filepath.as_str()),
        3 => year2019day03::solve(&filepath.as_str()),
        4 => year2019day04::solve(&filepath.as_str()),
        5 => year2019day05::solve(&filepath.as_str()),
        6 => year2019day06::solve(&filepath.as_str()),
        7 => year2019day07::solve(&filepath.as_str()),
        8 => year2019day08::solve(&filepath.as_str()),
        9 => year2019day09::solve(&filepath.as_str()),
        10 => year2019day10::solve(&filepath.as_str()),
        11 => year2019day11::solve(&filepath.as_str()),
        12 => year2019day12::solve(&filepath.as_str()),
        13 => year2019day13::solve(&filepath.as_str()),
        14 => year2019day14::solve(&filepath.as_str()),
        15 => year2019day15::solve(&filepath.as_str()),
        16 => year2019day16::solve(&filepath.as_str()),
        17 => year2019day17::solve(&filepath.as_str()),
        18 => year2019day18::solve(&filepath.as_str()),
        19 => year2019day19::solve(&filepath.as_str()),
        20 => year2019day20::solve(&filepath.as_str()),
        21 => year2019day21::solve(&filepath.as_str()),
        22 => year2019day22::solve(&filepath.as_str()),
        23 => year2019day23::solve(&filepath.as_str()),
        24 => year2019day24::solve(&filepath.as_str()),
        25 => year2019day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2020(day: usize, path: &str) {
    let filepath = get_filepath(2020, day, path);
    match day {
        1 => year2020day01::solve(&filepath.as_str()),
        2 => year2020day02::solve(&filepath.as_str()),
        3 => year2020day03::solve(&filepath.as_str()),
        4 => year2020day04::solve(&filepath.as_str()),
        5 => year2020day05::solve(&filepath.as_str()),
        6 => year2020day06::solve(&filepath.as_str()),
        7 => year2020day07::solve(&filepath.as_str()),
        8 => year2020day08::solve(&filepath.as_str()),
        9 => year2020day09::solve(&filepath.as_str()),
        10 => year2020day10::solve(&filepath.as_str()),
        11 => year2020day11::solve(&filepath.as_str()),
        12 => year2020day12::solve(&filepath.as_str()),
        13 => year2020day13::solve(&filepath.as_str()),
        14 => year2020day14::solve(&filepath.as_str()),
        15 => year2020day15::solve(&filepath.as_str()),
        16 => year2020day16::solve(&filepath.as_str()),
        17 => year2020day17::solve(&filepath.as_str()),
        18 => year2020day18::solve(&filepath.as_str()),
        19 => year2020day19::solve(&filepath.as_str()),
        20 => year2020day20::solve(&filepath.as_str()),
        21 => year2020day21::solve(&filepath.as_str()),
        22 => year2020day22::solve(&filepath.as_str()),
        23 => year2020day23::solve(&filepath.as_str()),
        24 => year2020day24::solve(&filepath.as_str()),
        25 => year2020day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2021(day: usize, path: &str) {
    let filepath = get_filepath(2021, day, path);
    match day {
        1 => year2021day01::solve(&filepath.as_str()),
        2 => year2021day02::solve(&filepath.as_str()),
        3 => year2021day03::solve(&filepath.as_str()),
        4 => year2021day04::solve(&filepath.as_str()),
        5 => year2021day05::solve(&filepath.as_str()),
        6 => year2021day06::solve(&filepath.as_str()),
        7 => year2021day07::solve(&filepath.as_str()),
        8 => year2021day08::solve(&filepath.as_str()),
        9 => year2021day09::solve(&filepath.as_str()),
        10 => year2021day10::solve(&filepath.as_str()),
        11 => year2021day11::solve(&filepath.as_str()),
        12 => year2021day12::solve(&filepath.as_str()),
        13 => year2021day13::solve(&filepath.as_str()),
        14 => year2021day14::solve(&filepath.as_str()),
        15 => year2021day15::solve(&filepath.as_str()),
        16 => year2021day16::solve(&filepath.as_str()),
        17 => year2021day17::solve(&filepath.as_str()),
        18 => year2021day18::solve(&filepath.as_str()),
        19 => year2021day19::solve(&filepath.as_str()),
        20 => year2021day20::solve(&filepath.as_str()),
        21 => year2021day21::solve(&filepath.as_str()),
        22 => year2021day22::solve(&filepath.as_str()),
        23 => year2021day23::solve(&filepath.as_str()),
        24 => year2021day24::solve(&filepath.as_str()),
        25 => year2021day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2022(day: usize, path: &str) {
    let filepath = get_filepath(2022, day, path);
    match day {
        1 => year2022day01::solve(&filepath.as_str()),
        2 => year2022day02::solve(&filepath.as_str()),
        3 => year2022day03::solve(&filepath.as_str()),
        4 => year2022day04::solve(&filepath.as_str()),
        5 => year2022day05::solve(&filepath.as_str()),
        6 => year2022day06::solve(&filepath.as_str()),
        7 => year2022day07::solve(&filepath.as_str()),
        8 => year2022day08::solve(&filepath.as_str()),
        9 => year2022day09::solve(&filepath.as_str()),
        10 => year2022day10::solve(&filepath.as_str()),
        11 => year2022day11::solve(&filepath.as_str()),
        12 => year2022day12::solve(&filepath.as_str()),
        13 => year2022day13::solve(&filepath.as_str()),
        14 => year2022day14::solve(&filepath.as_str()),
        15 => year2022day15::solve(&filepath.as_str()),
        16 => year2022day16::solve(&filepath.as_str()),
        17 => year2022day17::solve(&filepath.as_str()),
        18 => year2022day18::solve(&filepath.as_str()),
        19 => year2022day19::solve(&filepath.as_str()),
        20 => year2022day20::solve(&filepath.as_str()),
        21 => year2022day21::solve(&filepath.as_str()),
        22 => year2022day22::solve(&filepath.as_str()),
        23 => year2022day23::solve(&filepath.as_str()),
        24 => year2022day24::solve(&filepath.as_str()),
        25 => year2022day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2023(day: usize, path: &str) {
    let filepath = get_filepath(2023, day, path);
    match day {
        1 => year2023day01::solve(&filepath.as_str()),
        2 => year2023day02::solve(&filepath.as_str()),
        3 => year2023day03::solve(&filepath.as_str()),
        4 => year2023day04::solve(&filepath.as_str()),
        5 => year2023day05::solve(&filepath.as_str()),
        6 => year2023day06::solve(&filepath.as_str()),
        7 => year2023day07::solve(&filepath.as_str()),
        8 => year2023day08::solve(&filepath.as_str()),
        9 => year2023day09::solve(&filepath.as_str()),
        10 => year2023day10::solve(&filepath.as_str()),
        11 => year2023day11::solve(&filepath.as_str()),
        12 => year2023day12::solve(&filepath.as_str()),
        13 => year2023day13::solve(&filepath.as_str()),
        14 => year2023day14::solve(&filepath.as_str()),
        15 => year2023day15::solve(&filepath.as_str()),
        16 => year2023day16::solve(&filepath.as_str()),
        17 => year2023day17::solve(&filepath.as_str()),
        18 => year2023day18::solve(&filepath.as_str()),
        19 => year2023day19::solve(&filepath.as_str()),
        20 => year2023day20::solve(&filepath.as_str()),
        21 => year2023day21::solve(&filepath.as_str()),
        22 => year2023day22::solve(&filepath.as_str()),
        23 => year2023day23::solve(&filepath.as_str()),
        24 => year2023day24::solve(&filepath.as_str()),
        25 => year2023day25::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_year_2025(day: usize, path: &str) {
    let filepath = get_filepath(2025, day, path);
    match day {
        1 => year2025day01::solve(&filepath.as_str()),
        2 => year2025day02::solve(&filepath.as_str()),
        3 => year2025day03::solve(&filepath.as_str()),
        4 => year2025day04::solve(&filepath.as_str()),
        5 => year2025day05::solve(&filepath.as_str()),
        6 => year2025day06::solve(&filepath.as_str()),
        7 => year2025day07::solve(&filepath.as_str()),
        8 => year2025day08::solve(&filepath.as_str()),
        9 => year2025day09::solve(&filepath.as_str()),
        10 => year2025day10::solve(&filepath.as_str()),
        11 => year2025day11::solve(&filepath.as_str()),
        12 => year2025day12::solve(&filepath.as_str()),
        _ => {}
    }
    println!("");
}

fn solve_day(year: usize, day: usize, path: &str) {
    println!("== Year {} Day {:02} ==", year, day);
    match year {
        2015 => solve_year_2015(day, path),
        2016 => solve_year_2016(day, path),
        2017 => solve_year_2017(day, path),
        2018 => solve_year_2018(day, path),
        2019 => solve_year_2019(day, path),
        2020 => solve_year_2020(day, path),
        2021 => solve_year_2021(day, path),
        2022 => solve_year_2022(day, path),
        2023 => solve_year_2023(day, path),
        2025 => solve_year_2025(day, path),
        _ => {}
    }
    println!("");
}

#[derive(FromArgs)]
/// Advent of Code solutions by Jonas Karlsson
struct AoCArgs {
    /// year to solve
    #[argh(option, short = 'y', default = "2025")]
    year: usize,

    /// day to solve
    #[argh(option, short = 'd', default = "0")]
    day: usize,

    /// input file
    #[argh(option, short = 'i', default = "String::new()")]
    input: String,
}

fn main() {
    let args: AoCArgs = argh::from_env();
    let day = args.day;
    let year = args.year;
    let input = args.input.as_str();

    if day > 25 {
        println!("Invalid day '{}'", day);
        std::process::exit(-1);
    }

    if year == 2024 || year < 2015 || year > 2025 {
        println!("Invalid year '{}'", year);
        std::process::exit(-1);
    }

    if day == 0 {
        let num_days = if year == 2025 { 13 } else { 26 };
        for i in 1..num_days {
            solve_day(year, i, "");
        }
    } else {
        solve_day(year, day, &input);
    }
}
