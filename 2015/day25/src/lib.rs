extern crate common;

const INPUT_ROW: usize = 2978;
const INPUT_COLUMN: usize = 3083;

fn calc_value(row: usize, column: usize) -> usize {
    let mut r = 1;
    let mut c = 0;
    let mut prev = 20151125;

    if row == 1 && column == 1 {
        return prev;
    }

    loop {
        let curr = (prev * 252533) % 33554393;
        if r == (row - 1) && c == (column - 1) {
            return curr;
        }

        if r == 0 {
            r = c + 1;
            c = 0;
        } else {
            r -= 1;
            c += 1;
        }
        prev = curr;
    }
}

pub fn solve() {
    println!("Part one: {}", calc_value(INPUT_ROW, INPUT_COLUMN));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        assert_eq!(calc_value(1, 1), 20151125);
        assert_eq!(calc_value(2, 1), 31916031);
        assert_eq!(calc_value(1, 2), 18749137);
        assert_eq!(calc_value(6, 6), 27995004);
    }
}
