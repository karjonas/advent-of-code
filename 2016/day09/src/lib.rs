extern crate regex;

use regex::Regex;

fn decompress(s: String, recurse: bool) -> usize {
    let bytes = s.as_bytes();
    let regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();

    let mut output_size = 0;
    let mut i: usize = 0;
    while i < s.len() {
        if bytes[i] as char == '(' {
            let cap = regex.captures(&s[i..]).unwrap();
            let nchars = cap[1].to_string();
            let nchars_i = nchars.parse::<usize>().unwrap();

            let ntimes = cap[2].to_string();
            let ntimes_i = ntimes.parse::<usize>().unwrap();

            i += nchars.len() + ntimes.len() + 3;

            let mut slice = String::new();
            for x in i..(i + nchars_i) {
                slice.push(bytes[x] as char)
            }

            if recurse {
                output_size += ntimes_i * decompress(slice, recurse);
            } else {
                output_size += ntimes_i * slice.len();
            }

            i += nchars_i;
        } else {
            output_size += 1;
            i += 1;
        }
    }

    return output_size;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    //    let test_str0 = "ADVENT";
    //    let test_str1 = "A(1x5)BC";
    //    let test_str2 = "(3x3)XYZ";
    //    let test_str3 = "A(2x2)BCD(2x2)EFG";
    //    let test_str4 = "(6x1)(1x3)A";
    //    let test_str5 = "X(8x2)(3x3)ABCY";
    //
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str0.to_string(), true));
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str1.to_string(), true));
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str2.to_string(), true));
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str3.to_string(), true));
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str4.to_string(), true));
    //    println!("Decompressed recursive size {}",
    //             decompress(test_str5.to_string(), true));
    println!("Part 1: {}", decompress(input.clone(), false));
    println!("Part 2: {}", decompress(input, true));
}
