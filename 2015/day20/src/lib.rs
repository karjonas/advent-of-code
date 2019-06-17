extern crate common;

const INPUT: usize = 36000000;

fn solve_internal(presents_per_elf: usize, houses_per_elf: usize) -> usize {
    let num_houses = 1000 * 1000 * 10;
    let mut houses = common::filled_vector(num_houses, 0);

    for i in 1..num_houses {
        let steps = std::cmp::min(num_houses / i, houses_per_elf);
        for quot in 1..steps {
            let idx = quot * i;
            houses[idx] += i * presents_per_elf;
        }
        if houses[i] >= INPUT {
            return i;
        }
    }
    return 0;
}

pub fn solve() {
    println!("Part one: {}", solve_internal(10, std::usize::MAX));
    println!("Part two: {}", solve_internal(11, 50));
}
