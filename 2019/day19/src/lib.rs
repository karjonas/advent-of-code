extern crate common;
extern crate intcode;

fn is_hit(memory: Vec<i64>, x: usize, y: usize) -> bool {
    let (_memory_new, output_numbers, _index_new, _relative_base_new, _halted) = intcode::run(
        memory,
        [x as i64, y as i64].to_vec(),
        0,
        0,
    );
    assert_eq!(output_numbers.len(), 1);
    let hit = *output_numbers.first().unwrap() == 1;
    return hit;
}

fn solve_part_one(memory: Vec<i64>, width: usize, height: usize) -> usize {
    let mut num_pull = 0;
    for x in 0..width {
        for y in 0..height {
            num_pull += if  is_hit(memory.clone(), x, y) { 1 } else { 0 };
        }
    }
    return num_pull;
}

fn solve_part_two(memory: Vec<i64>) -> usize {
    let mut x1 = 99;
    let mut y0 = 0;

    while y0 < std::usize::MAX {
        let x0 = x1-99;
        let y1 = y0+99;

        let hit_first = is_hit(memory.clone(), x1, y0);
        let hit_second = is_hit(memory.clone(), x0, y1);
        let hit_right = is_hit(memory.clone(), x1+1, y0);

        if hit_first && hit_second {
            return (x0*10000) + y0;
        }

        if hit_right {
            x1 += 1;
        } else {
            y0 += 1;
        }
    }

    return 0;
}

pub fn solve() {
    let input = common::read_file("2019/day19/input");
    let memory = intcode::parse_input(input.as_str());

    println!("Part one: {}", solve_part_one(memory.clone(), 50, 50));
    println!("Part two: {}", solve_part_two(memory));
}
