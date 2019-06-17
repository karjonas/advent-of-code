use std::collections::LinkedList;

const GEN_A_SEED: usize = 277;
const GEN_B_SEED: usize = 349;
const FACTOR_A: usize = 16807;
const FACTOR_B: usize = 48271;
const DIV_VALUE: usize = 2147483647;
const NUM_REPS: usize = 40000000;

fn generate(mod_a: usize, mod_b: usize) -> usize {
    let mut last_value_a = GEN_A_SEED;
    let mut last_value_b = GEN_B_SEED;

    let mut hits = 0;

    let mut gens_a = LinkedList::<usize>::new();
    let mut gens_b = LinkedList::<usize>::new();

    for _ in 0..NUM_REPS {
        let next_value_a = (last_value_a * FACTOR_A) % DIV_VALUE;
        let next_value_b = (last_value_b * FACTOR_B) % DIV_VALUE;

        if (next_value_a % mod_a) == 0 {
            let lower_bits_a = next_value_a & 0xffff;
            gens_a.push_back(lower_bits_a);
        }

        if (next_value_b % mod_b) == 0 {
            let lower_bits_b = next_value_b & 0xffff;
            gens_b.push_back(lower_bits_b);
        }

        last_value_a = next_value_a;
        last_value_b = next_value_b;

        if !gens_a.is_empty() && !gens_b.is_empty() {
            let v_a = gens_a.pop_front();
            let v_b = gens_b.pop_front();

            if v_a == v_b {
                hits += 1;
            }
        }
    }

    return hits;
}

pub fn solve() {
    println!("Part one: {}", generate(1, 1));
    println!("Part two: {}", generate(4, 8));
}
