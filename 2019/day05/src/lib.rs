extern crate common;
extern crate intcode;

fn run_input(input_str: &str, input_number: i64) -> Vec<i64> {
    let memory = intcode::parse_input(input_str);
    let (_memory, output_numbers, _index, _relative_base, _halted) =
        intcode::run(memory, [input_number].to_vec(), 0, 0);
    return output_numbers;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    println!("Part one: {}", run_input(input.as_str(), 1).last().unwrap());
    println!("Part two: {}", run_input(input.as_str(), 5).last().unwrap());
}
