extern crate skiplist;
use skiplist::SkipList;

fn solve_internal(input: usize, remove_across: bool) -> usize {
    let mut elves: SkipList<usize> = SkipList::new();

    for i in 0..input {
        elves.push_back(i);
    }

    while elves.len() > 1 {
        let mut num_elves = elves.len();
        let mut i = 0;
        while i < num_elves {
            let to_remove = if remove_across {
                (i + num_elves / 2) % num_elves
            } else {
                (i + 1) % num_elves
            };
            elves.remove(to_remove);
            num_elves = elves.len();
            if to_remove > i {
                i += 1;
            }
        }
    }

    return elves[0] + 1;
}

pub fn solve() {
    println!("Part 1: {}", solve_internal(3001330, false));
    println!("Part 2: {}", solve_internal(3001330, true));
}
