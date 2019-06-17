extern crate common;

#[derive(Debug)]
struct Dimension {
    l: usize,
    w: usize,
    h: usize,
}

fn parse_input() -> Vec<Dimension> {
    let input = common::read_file("2015/day02/input");
    let mut list = Vec::new();
    for line in input.lines() {
        let vals: Vec<_> = line.split('x').collect();
        let l = common::string_to_i64(vals[0]) as usize;
        let w = common::string_to_i64(vals[1]) as usize;
        let h = common::string_to_i64(vals[2]) as usize;
        list.push(Dimension { l: l, w: w, h: h });
    }
    return list;
}

pub fn solve() {
    let dims = parse_input();
    let part_one = dims.iter().fold(0, |sum, v| sum + surface_area(v));
    let part_two = dims.iter().fold(0, |sum, v| sum + ribbon_length(v));
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

fn surface_area(d: &Dimension) -> usize {
    let side_a = d.l * d.w;
    let side_b = d.w * d.h;
    let side_c = d.h * d.l;
    let side_min = std::cmp::min(std::cmp::min(side_a, side_b), side_c);

    side_min + 2 * (side_a + side_b + side_c)
}

fn ribbon_length(d: &Dimension) -> usize {
    let mut s = [d.l, d.w, d.h];
    s.sort();
    s[0] * s[1] * s[2] + 2 * (s[0] + s[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one() {
        assert_eq!(surface_area(&Dimension { l: 2, w: 3, h: 4 }), 58);
        assert_eq!(surface_area(&Dimension { l: 1, w: 1, h: 10 }), 43);
    }

    #[test]
    fn test_samples_part_two() {
        assert_eq!(ribbon_length(&Dimension { l: 2, w: 3, h: 4 }), 34);
        assert_eq!(ribbon_length(&Dimension { l: 1, w: 1, h: 10 }), 14);
    }

}
