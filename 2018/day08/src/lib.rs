extern crate common;

// Returns next index in input list, the parsed nodes value and metadata summed value
fn find_node_recursive(index: usize, input: &Vec<usize>) -> (usize, usize, usize) {
    let num_children = input[index];
    let num_metadata = input[index + 1];
    let mut child_value: Vec<usize> = common::filled_vector(num_children, 0);
    let mut metadata: Vec<usize> = common::filled_vector(num_children, 0);
    let mut value = 0;
    let mut metadata_sum_total = 0;
    let mut index_next = index + 2;

    for i in 0..num_children {
        let (next, value, metadata_sum) = find_node_recursive(index_next, input);
        index_next = next;
        child_value[i] = value;
        metadata_sum_total += metadata_sum;
    }

    for _ in 0..num_metadata {
        let meta_value = input[index_next];
        metadata.push(meta_value);
        index_next += 1;

        if meta_value < num_children + 1 {
            value += child_value[meta_value - 1];
        }
    }

    metadata_sum_total += metadata.iter().fold(0, |sum, v| sum + v);

    if num_children == 0 {
        value = metadata_sum_total;
    }

    return (index_next, value, metadata_sum_total);
}

pub fn solve() {
    let input: Vec<usize> = common::read_file("2018/day08/input")
        .split_whitespace()
        .map(|v| v.to_string().parse::<usize>().unwrap())
        .collect();
    let (_, value_part_two, value_part_one) = find_node_recursive(0, &input);
    println!("Part one: {:?}", value_part_one);
    println!("Part two: {:?}", value_part_two);
}
