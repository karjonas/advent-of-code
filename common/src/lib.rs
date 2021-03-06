use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents.trim_end_matches('\n').to_string();
}

pub fn filled_vector<T: Clone>(size: usize, value: T) -> Vec<T> {
    let mut vec: Vec<T> = Vec::with_capacity(size);
    let t = value;
    for _ in 0..size {
        vec.push(t.clone());
    }
    return vec;
}

pub fn permute<T: Clone>(curr: &Vec<T>, rest: &Vec<T>) -> Vec<Vec<T>> {
    let mut collected = Vec::new();

    if rest.is_empty() {
        collected.push(curr.clone());
    } else {
        for i in 0..rest.len() {
            let mut next = curr.clone();
            next.push(rest[i].clone());

            let mut next_rest = rest.clone();
            next_rest.remove(i);

            let mut perms = permute(&next, &next_rest);
            collected.append(&mut perms);
        }
    }
    return collected;
}

pub fn subsequences<T: Clone>(arr: &Vec<T>, index: usize, subarr: &Vec<T>) -> Vec<Vec<T>> {
    let mut collected = Vec::new();
    // Print the subsequence when reach the leaf of recursion tree
    if index == arr.len() {
        // Condition to avoid printing empty subsequence
        if subarr.len() != 0 {
            collected.push(subarr.clone());
        }
    } else {
        // Subsequence without including the element at current index
        let mut coll = subsequences(&arr, index + 1, subarr);
        collected.append(&mut coll);

        // Subsequence including the element at current index
        let mut subarr_next = subarr.clone();
        subarr_next.push(arr[index].clone());
        let mut coll = subsequences(&arr, index + 1, &subarr_next);
        collected.append(&mut coll);
    }
    return collected;
}

pub fn zeroed_vector(size: usize) -> Vec<usize> {
    return filled_vector(size, 0);
}

pub fn uppercase(c: char) -> char {
    let c_u8 = c as u8;
    let lowercase_end = 'z' as u8;
    let lowercase_start = 'a' as u8;
    let uppercase_start = 'A' as u8;
    let uppercase_end = 'Z' as u8;

    assert!(
        (c_u8 >= lowercase_start && c_u8 <= lowercase_end)
            || (c_u8 >= uppercase_start && c_u8 <= uppercase_end)
    );

    if c_u8 >= lowercase_start && c_u8 <= lowercase_end {
        return (uppercase_start + (c_u8 - lowercase_start)) as char;
    }
    return c;
}

pub fn first_index_of(stack: &String, needle: char) -> usize {
    let mut ctr = 0;
    for c in stack.chars() {
        if needle == c {
            return ctr;
        }
        ctr += 1;
    }

    return stack.len();
}

pub fn string_to_i64(s: &str) -> i64 {
    return s.to_string().trim().parse::<i64>().unwrap();
}

pub fn string_to_usize(s: &str) -> usize {
    return s.to_string().trim().parse::<usize>().unwrap();
}

pub fn char_to_u8(c: char) -> u8 {
    assert!((c as u8) >= ('0' as u8));
    assert!((c as u8) <= ('9' as u8));

    return (c as u8) - ('0' as u8);
}

pub fn strip_characters(original: &str, to_strip: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
            result.push(c);
        }
    }
    result
}

pub fn is_number(c: &str) -> bool {
    match c.to_string().trim().parse::<i64>() {
        Ok(_) => return true,
        _ => return false,
    }
}

pub fn grow<T: Clone>(vec: &mut Vec<T>, size: usize, default_value: T) {
    if vec.len() < size {
        vec.resize(size, default_value);
    }
}

pub fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn lcm(a: i128, b: i128) -> i128 {
    return a * b / egcd(a, b).0;
}

fn mod_inv(x: i128, n: i128) -> Option<i128> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

pub fn chinese_remainder(residues: &[i128], modulii: &[i128]) -> Option<i128> {
    let prod = modulii.iter().product::<i128>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}
