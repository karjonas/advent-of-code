use std::collections::HashSet;

fn solve_internal() -> (usize, usize) {
    let mut r2 = 0;

    let mut halting_numbers = HashSet::new();
    let mut last_num = 0;
    let mut first_num = 0;
    loop {
        let mut r4 = r2 | 65536;
        r2 = 6718165;
        loop {
            let mut r3 = r4 & 255;
            r2 = r2 + r3;
            r2 = r2 & 16777215;
            r2 = r2 * 65899;
            r2 = r2 & 16777215;
            if 256 > r4 {
                if first_num == 0 {
                    first_num = r2;
                }
                if halting_numbers.contains(&r2) {
                    return (first_num, last_num);
                }
                last_num = r2;
                halting_numbers.insert(r2);

                break;
            }

            r3 = 0;
            loop {
                if (r3 + 1) * 256 > r4 {
                    break;
                }
                r3 += 1;
            }
            r4 = r3;
        }
    }
}

pub fn solve() {
    let (p0, p1) = solve_internal();
    println!("Part one: {}", p0);
    println!("Part two: {}", p1);
}
