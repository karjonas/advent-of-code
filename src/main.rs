// Year 2015
extern crate year2015day01;
extern crate year2015day02;
extern crate year2015day03;
extern crate year2015day04;
extern crate year2015day05;
extern crate year2015day06;
extern crate year2015day07;
extern crate year2015day08;
extern crate year2015day09;
extern crate year2015day10;
extern crate year2015day11;
extern crate year2015day12;
extern crate year2015day13;
extern crate year2015day14;
extern crate year2015day15;
extern crate year2015day16;
extern crate year2015day17;
extern crate year2015day18;
extern crate year2015day19;
extern crate year2015day20;
extern crate year2015day21;
extern crate year2015day22;
extern crate year2015day23;
extern crate year2015day24;
extern crate year2015day25;

// Year 2016
extern crate year2016day01;
extern crate year2016day02;
extern crate year2016day03;
extern crate year2016day04;
extern crate year2016day05;
extern crate year2016day06;
extern crate year2016day07;
extern crate year2016day08;
extern crate year2016day09;
extern crate year2016day10;
extern crate year2016day11;
extern crate year2016day12;
extern crate year2016day13;
extern crate year2016day14;
extern crate year2016day15;
extern crate year2016day16;
extern crate year2016day17;
extern crate year2016day18;
extern crate year2016day19;
extern crate year2016day20;
extern crate year2016day21;
extern crate year2016day22;
extern crate year2016day23;
extern crate year2016day24;
extern crate year2016day25;

// Year 2017
extern crate year2017day01;
extern crate year2017day02;
extern crate year2017day03;
extern crate year2017day04;
extern crate year2017day05;
extern crate year2017day06;
extern crate year2017day07;
extern crate year2017day08;
extern crate year2017day09;
extern crate year2017day10;
extern crate year2017day11;
extern crate year2017day12;
extern crate year2017day13;
extern crate year2017day14;
extern crate year2017day15;
extern crate year2017day16;
extern crate year2017day17;
extern crate year2017day18;
extern crate year2017day19;
extern crate year2017day20;
extern crate year2017day21;
extern crate year2017day22;
extern crate year2017day23;
extern crate year2017day24;
extern crate year2017day25;

// Year 2018
extern crate year2018day01;
extern crate year2018day02;
extern crate year2018day03;
extern crate year2018day04;
extern crate year2018day05;
extern crate year2018day06;
extern crate year2018day07;
extern crate year2018day08;
extern crate year2018day09;
extern crate year2018day10;
extern crate year2018day11;
extern crate year2018day12;
extern crate year2018day13;
extern crate year2018day14;
extern crate year2018day15;
extern crate year2018day16;
extern crate year2018day17;
extern crate year2018day18;
extern crate year2018day19;
extern crate year2018day20;
extern crate year2018day21;
extern crate year2018day22;
extern crate year2018day23;
extern crate year2018day24;
extern crate year2018day25;

// Year 2019
extern crate year2019day01;
extern crate year2019day02;
extern crate year2019day03;
extern crate year2019day04;
extern crate year2019day05;
extern crate year2019day06;
extern crate year2019day07;
extern crate year2019day08;
extern crate year2019day09;
extern crate year2019day10;
extern crate year2019day11;
extern crate year2019day12;
extern crate year2019day13;
extern crate year2019day14;
extern crate year2019day15;
extern crate year2019day16;
extern crate year2019day17;
extern crate year2019day18;
extern crate year2019day19;
extern crate year2019day20;
extern crate year2019day21;
extern crate year2019day22;
extern crate year2019day23;
extern crate year2019day24;
extern crate year2019day25;

extern crate common;

extern crate clap;

use clap::{App, Arg};

fn solve_day(year: usize, day: usize) {
    println!("== Year {} Day {:02} ==", year, day);
    match day {
        1 => {
            if year == 2015 {
                year2015day01::solve();
            }
            if year == 2016 {
                year2016day01::solve();
            }
            if year == 2017 {
                year2017day01::solve();
            }
            if year == 2018 {
                year2018day01::solve();
            }
            if year == 2019 {
                year2019day01::solve();
            }
        }
        2 => {
            if year == 2015 {
                year2015day02::solve();
            }
            if year == 2016 {
                year2016day02::solve();
            }
            if year == 2017 {
                year2017day02::solve();
            }
            if year == 2018 {
                year2018day02::solve();
            }
            if year == 2019 {
                year2019day02::solve();
            }
        }
        3 => {
            if year == 2015 {
                year2015day03::solve();
            }
            if year == 2016 {
                year2016day03::solve();
            }
            if year == 2017 {
                year2017day03::solve();
            }
            if year == 2018 {
                year2018day03::solve();
            }
            if year == 2019 {
                year2019day03::solve();
            }
        }
        4 => {
            if year == 2015 {
                year2015day04::solve();
            }
            if year == 2016 {
                year2016day04::solve();
            }
            if year == 2017 {
                year2017day04::solve();
            }
            if year == 2018 {
                year2018day04::solve();
            }
            if year == 2019 {
                year2019day04::solve();
            }
        }
        5 => {
            if year == 2015 {
                year2015day05::solve();
            }
            if year == 2016 {
                year2016day05::solve();
            }
            if year == 2017 {
                year2017day05::solve();
            }
            if year == 2018 {
                year2018day05::solve();
            }
            if year == 2019 {
                year2019day05::solve();
            }
        }
        6 => {
            if year == 2015 {
                year2015day06::solve();
            }
            if year == 2016 {
                year2016day06::solve();
            }
            if year == 2017 {
                year2017day06::solve();
            }
            if year == 2018 {
                year2018day06::solve();
            }
            if year == 2019 {
                year2019day06::solve();
            }
        }
        7 => {
            if year == 2015 {
                year2015day07::solve();
            }
            if year == 2016 {
                year2016day07::solve();
            }
            if year == 2017 {
                year2017day07::solve();
            }
            if year == 2018 {
                year2018day07::solve();
            }
            if year == 2019 {
                year2019day07::solve();
            }
        }
        8 => {
            if year == 2015 {
                year2015day08::solve();
            }
            if year == 2016 {
                year2016day08::solve();
            }
            if year == 2017 {
                year2017day08::solve();
            }
            if year == 2018 {
                year2018day08::solve();
            }
            if year == 2019 {
                year2019day08::solve();
            }
        }
        9 => {
            if year == 2015 {
                year2015day09::solve();
            }
            if year == 2016 {
                year2016day09::solve();
            }
            if year == 2017 {
                year2017day09::solve();
            }
            if year == 2018 {
                year2018day09::solve();
            }
            if year == 2019 {
                year2019day09::solve();
            }
        }
        10 => {
            if year == 2015 {
                year2015day10::solve();
            }
            if year == 2016 {
                year2016day10::solve();
            }
            if year == 2017 {
                year2017day10::solve();
            }
            if year == 2018 {
                year2018day10::solve();
            }
            if year == 2019 {
                year2019day10::solve();
            }
        }
        11 => {
            if year == 2015 {
                year2015day11::solve();
            }
            if year == 2016 {
                year2016day11::solve();
            }
            if year == 2017 {
                year2017day11::solve();
            }
            if year == 2018 {
                year2018day11::solve();
            }
            if year == 2019 {
                year2019day11::solve();
            }
        }
        12 => {
            if year == 2015 {
                year2015day12::solve();
            }
            if year == 2016 {
                year2016day12::solve();
            }
            if year == 2017 {
                year2017day12::solve();
            }
            if year == 2018 {
                year2018day12::solve();
            }
            if year == 2019 {
                year2019day12::solve();
            }
        }
        13 => {
            if year == 2015 {
                year2015day13::solve();
            }
            if year == 2016 {
                year2016day13::solve();
            }
            if year == 2017 {
                year2017day13::solve();
            }
            if year == 2018 {
                year2018day13::solve();
            }
            if year == 2019 {
                year2019day13::solve();
            }
        }
        14 => {
            if year == 2015 {
                year2015day14::solve();
            }
            if year == 2016 {
                year2016day14::solve();
            }
            if year == 2017 {
                year2017day14::solve();
            }
            if year == 2018 {
                year2018day14::solve();
            }
            if year == 2019 {
                year2019day14::solve();
            }
        }
        15 => {
            if year == 2015 {
                year2015day15::solve();
            }
            if year == 2016 {
                year2016day15::solve();
            }
            if year == 2017 {
                year2017day15::solve();
            }
            if year == 2018 {
                year2018day15::solve();
            }
            if year == 2019 {
                year2019day15::solve();
            }
        }
        16 => {
            if year == 2015 {
                year2015day16::solve();
            }
            if year == 2016 {
                year2016day16::solve();
            }
            if year == 2017 {
                year2017day16::solve();
            }
            if year == 2018 {
                year2018day16::solve();
            }
            if year == 2019 {
                year2019day16::solve();
            }
        }
        17 => {
            if year == 2015 {
                year2015day17::solve();
            }
            if year == 2016 {
                year2016day17::solve();
            }
            if year == 2017 {
                year2017day17::solve();
            }
            if year == 2018 {
                year2018day17::solve();
            }
            if year == 2019 {
                year2019day17::solve();
            }
        }
        18 => {
            if year == 2015 {
                year2015day18::solve();
            }
            if year == 2016 {
                year2016day18::solve();
            }
            if year == 2017 {
                year2017day18::solve();
            }
            if year == 2018 {
                year2018day18::solve();
            }
            if year == 2019 {
                year2019day18::solve();
            }
        }
        19 => {
            if year == 2015 {
                year2015day19::solve();
            }
            if year == 2016 {
                year2016day19::solve();
            }
            if year == 2017 {
                year2017day19::solve();
            }
            if year == 2018 {
                year2018day19::solve();
            }
            if year == 2019 {
                year2019day19::solve();
            }
        }
        20 => {
            if year == 2015 {
                year2015day20::solve();
            }
            if year == 2016 {
                year2016day20::solve();
            }
            if year == 2017 {
                year2017day20::solve();
            }
            if year == 2018 {
                year2018day20::solve();
            }
            if year == 2019 {
                year2019day20::solve();
            }
        }
        21 => {
            if year == 2015 {
                year2015day21::solve();
            }
            if year == 2016 {
                year2016day21::solve();
            }
            if year == 2017 {
                year2017day21::solve();
            }
            if year == 2018 {
                year2018day21::solve();
            }
            if year == 2019 {
                year2019day21::solve();
            }
        }
        22 => {
            if year == 2015 {
                year2015day22::solve();
            }
            if year == 2016 {
                year2016day22::solve();
            }
            if year == 2017 {
                year2017day22::solve();
            }
            if year == 2018 {
                year2018day22::solve();
            }
            if year == 2019 {
                year2019day22::solve();
            }
        }
        23 => {
            if year == 2015 {
                year2015day23::solve();
            }
            if year == 2016 {
                year2016day23::solve();
            }
            if year == 2017 {
                year2017day23::solve();
            }
            if year == 2018 {
                year2018day23::solve();
            }
            if year == 2019 {
                year2019day23::solve();
            }
        }
        24 => {
            if year == 2015 {
                year2015day24::solve();
            }
            if year == 2016 {
                year2016day24::solve();
            }
            if year == 2017 {
                year2017day24::solve();
            }
            if year == 2018 {
                year2018day24::solve();
            }
            if year == 2019 {
                year2019day24::solve();
            }
        }
        25 => {
            if year == 2015 {
                year2015day25::solve();
            }
            if year == 2016 {
                year2016day25::solve();
            }
            if year == 2017 {
                year2017day25::solve();
            }
            if year == 2018 {
                year2018day25::solve();
            }
            if year == 2019 {
                year2019day25::solve();
            }
        }
        _ => {}
    }
    println!("");
}

fn main() {
    let matches = App::new("Advent of Code")
        .version("0.1.0")
        .author("Karlsson Jonas")
        .about("Advent of Code solutions")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .help("Year to solve"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("Day to solve"),
        )
        .get_matches();

    let day_str = matches.value_of("day").unwrap_or("");
    let day = if common::is_number(day_str) {
        common::string_to_usize(day_str)
    } else {
        0
    };

    let year_str = matches.value_of("year").unwrap_or("");
    let year = if common::is_number(year_str) {
        common::string_to_usize(year_str)
    } else {
        2019
    };

    if day > 25 {
        println!("Invalid day '{}'", day);
        std::process::exit(-1);
    }

    if year > 2019 || year < 2015 {
        println!("Invalid year '{}'", year);
        std::process::exit(-1);
    }

    if day == 0 {
        for i in 1..26 {
            solve_day(year, i);
        }
    } else {
        solve_day(year, day);
    }
}
