extern crate common;

type Value = (usize, usize);

fn parse_snails(input: &String) -> Vec<Value> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut depth = 0;
    let mut i = 0;
    let mut output = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c == '[' {
            depth += 1;
            i += 1;
            continue;
        }
        if c == ']' {
            depth -= 1;
            i += 1;
            continue;
        }
        if c == ',' {
            i += 1;
            continue;
        }

        let v0 = c as i32 - '0' as i32;
        let v1 = chars[i + 1] as i32 - '0' as i32;
        let v;
        if v1 >= 0 && v1 <= 9 {
            v = v0 * 10 + v1;
            i += 1;
        } else {
            v = v0;
        }

        output.push((v as usize, depth as usize));
        i += 1;
    }

    return output;
}

fn reduce(input: &Vec<Value>) -> Vec<Value> {
    let mut result: Vec<Value> = input.clone();
    for i in 0..input.len() {
        let (value, depth) = input[i];

        if depth > 4 {
            // explode
            if i > 0 {
                (&mut result[i - 1]).0 += value;
            }
            if i + 2 < input.len() {
                (&mut result[i + 2]).0 += result[i + 1].0;
            }
            result.remove(i);
            (&mut result[i]).0 = 0;
            (&mut result[i]).1 = depth - 1;
            return result;
        }
    }

    for i in 0..input.len() {
        let (value, depth) = input[i];

        if value >= 10 {
            let v_half = value as f32 * 0.5;
            let v0 = v_half.floor() as usize;
            let v1 = v_half.ceil() as usize;
            assert_eq!(v0 + v1, value);
            (&mut result[i]).0 = v0;
            (&mut result[i]).1 = depth + 1;
            result.insert(i + 1, (v1, depth + 1));
            return result;
        }
    }

    return result;
}

fn add(a: &Vec<Value>, b: &Vec<Value>) -> Vec<Value> {
    let mut result = Vec::new();
    for &(value, depth) in a {
        result.push((value, depth + 1));
    }
    for &(value, depth) in b {
        result.push((value, depth + 1));
    }
    return result;
}

fn solve_addition(lines: &Vec<Vec<Value>>) -> Vec<Value> {
    let mut curr = lines[0].clone();

    for line in &lines[1..lines.len()] {
        let next = line;
        let mut merged = add(&curr, &next);
        let mut last = Vec::new();

        while merged != last {
            let reduced = reduce(&merged);
            last = merged.clone();
            merged = reduced;
        }
        curr = merged;
    }

    return curr;
}

fn solve_internal_addition(input: &String) -> Vec<Value> {
    let lines: Vec<_> = input
        .lines()
        .map(|v| parse_snails(&v.to_string()))
        .collect();
    return solve_addition(&lines);
}

fn calc_magnitude(input: &Vec<(usize, usize)>) -> usize {
    let mut curr = input.clone();
    let mut last = Vec::new();
    while curr != last {
        let deepest = *curr.iter().map(|(_v, d)| d).max().unwrap();
        last = curr.clone();

        if curr.len() == 1 {
            break;
        }
        for i in 0..curr.len() {
            if curr[i].1 == deepest {
                assert_eq!(curr[i].1, curr[i + 1].1);
                let left = 3 * curr[i].0;
                let right = 2 * curr[i + 1].0;
                (&mut curr[i]).0 = left + right;
                (&mut curr[i]).1 -= 1;
                curr.remove(i + 1);
                break;
            }
        }
    }

    return curr[0].0;
}

fn solve_internal_p1(input: &String) -> usize {
    return calc_magnitude(&solve_internal_addition(&input));
}

fn solve_internal_p2(input: &String) -> usize {
    let lines: Vec<_> = input.lines().map(|v| v.to_string()).collect();
    let mut max = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            let line_i = parse_snails(&lines[i]);
            let line_j = parse_snails(&lines[j]);
            let lines = [line_i, line_j].to_vec();
            let mag0 = calc_magnitude(&solve_addition(&lines));
            let mag1 = calc_magnitude(&solve_addition(&lines));
            max = std::cmp::max(max, std::cmp::max(mag0, mag1));
        }
    }
    return max;
}

pub fn solve() {
    let input = common::read_file("2021/day18/input");
    println!("Part one: {}", solve_internal_p1(&input));
    println!("Part two: {}", solve_internal_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let explode = "[[[[[9,8],1],2],3],4]";
        let explode1 = "[7,[6,[5,[4,[3,2]]]]]";
        let explode2 = "[[6,[5,[4,[3,2]]]],1]";
        let explode3 = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
        let explode4 = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        let split = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";

        assert_eq!(
            reduce(&parse_snails(&String::from(explode))),
            parse_snails(&"[[[[0,9],2],3],4]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from(explode1))),
            parse_snails(&"[7,[6,[5,[7,0]]]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from(explode2))),
            parse_snails(&"[[6,[5,[7,0]]],3]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from(explode3))),
            parse_snails(&"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from(explode4))),
            parse_snails(&"[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from(split))),
            parse_snails(&"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"))),
            parse_snails(&"[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]"))),
            parse_snails(&"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"))),
            parse_snails(&"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string())
        );
        assert_eq!(
            reduce(&parse_snails(&String::from("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"))),
            parse_snails(&"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string())
        );

        let input = "[1,1]
[2,2]
[3,3]
[4,4]";

        let input1 = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";

        let input2 = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";

        let input3 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

        let add0 = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let goal0 = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";
        let add1 = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]";
        let goal1 = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]";
        let add2 = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]";
        let goal2 = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]";
        let add3 = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]
[7,[5,[[3,8],[1,4]]]]";
        let goal3 = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]";
        let add4 = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]
[[2,[2,2]],[8,[8,1]]]";
        let goal4 = "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]";
        let add5 = "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]
[2,9]";
        let goal5 = "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]";
        let add6 = "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
[1,[[[9,3],9],[[9,0],[0,7]]]]";
        let goal6 = "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]";
        let add7 = "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]
[[[5,[7,4]],7],1]";
        let goal7 = "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]";
        let add8 = "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]
[[[[4,2],2],6],[8,7]]";
        let goal8 = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        assert_eq!(
            solve_internal_addition(&add0.to_string()),
            parse_snails(&goal0.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add1.to_string()),
            parse_snails(&goal1.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add2.to_string()),
            parse_snails(&goal2.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add3.to_string()),
            parse_snails(&goal3.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add4.to_string()),
            parse_snails(&goal4.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add5.to_string()),
            parse_snails(&goal5.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add6.to_string()),
            parse_snails(&goal6.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add7.to_string()),
            parse_snails(&goal7.to_string())
        );
        assert_eq!(
            solve_internal_addition(&add8.to_string()),
            parse_snails(&goal8.to_string())
        );

        assert_eq!(
            solve_internal_addition(&String::from(input)),
            parse_snails(&"[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string())
        );
        assert_eq!(
            solve_internal_addition(&String::from(input1)),
            parse_snails(&"[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string())
        );
        assert_eq!(
            solve_internal_addition(&String::from(input2)),
            parse_snails(&"[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string())
        );

        assert_eq!(
            solve_internal_addition(&String::from(input3)),
            parse_snails(&"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string())
        );

        let calc0 = "[[1,2],[[3,4],5]]";
        let result0 = 143;
        let calc1 = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let result1 = 1384;
        let calc2 = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        let result2 = 445;
        let calc3 = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        let result3 = 791;
        let calc4 = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        let result4 = 1137;
        let calc5 = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        let result5 = 3488;

        assert_eq!(calc_magnitude(&parse_snails(&calc0.to_string())), result0);
        assert_eq!(calc_magnitude(&parse_snails(&calc1.to_string())), result1);
        assert_eq!(calc_magnitude(&parse_snails(&calc2.to_string())), result2);
        assert_eq!(calc_magnitude(&parse_snails(&calc3.to_string())), result3);
        assert_eq!(calc_magnitude(&parse_snails(&calc4.to_string())), result4);
        assert_eq!(calc_magnitude(&parse_snails(&calc5.to_string())), result5);
    }
}
