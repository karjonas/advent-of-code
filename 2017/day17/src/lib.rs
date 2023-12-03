pub fn solve(_filepath: &str) {
    let input = 312;

    {
        let mut buffer = vec![0];
        let mut curr_pos = 0;

        for i in 1..2018 {
            let num_vals = i;
            curr_pos = (input + curr_pos) % num_vals + 1;
            buffer.insert(curr_pos, i);
        }

        let next_value = buffer[(curr_pos + 1) % buffer.len()];
        println!("Part one: {}", next_value);
    }

    {
        let mut curr_pos = 0;
        let mut goal_value = 0;

        for i in 1..50000000 {
            let num_vals = i;
            curr_pos = (input + curr_pos) % num_vals + 1;

            // Value zero is always at zeroeth index so only store
            // when index 1 is written.
            if curr_pos == 1 {
                goal_value = i;
            }
        }

        println!("Part two: {}", goal_value);
    }
}
