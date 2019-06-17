extern crate common;
extern crate serde_json;

use serde_json::Value;

fn sum_recursive(value: &Value, ignore_red: bool) -> i64 {
    match value {
        &Value::Number(ref x) => x.as_i64().unwrap(),
        &Value::Array(ref arr) => arr
            .iter()
            .fold(0, |sum, v| sum + sum_recursive(v, ignore_red)),
        &Value::Object(ref map) => {
            let values = map.values().collect::<Vec<_>>();
            let ignore = ignore_red
                && values
                    .iter()
                    .any(|x| x.is_string() && x.as_str().unwrap() == "red");

            return if ignore {
                0
            } else {
                values
                    .iter()
                    .fold(0, |sum, v| sum + sum_recursive(v, ignore_red))
            };
        }
        _ => 0,
    }
}

pub fn solve() {
    let input = common::read_file("2015/day12/input");
    println!(
        "Part one: {}",
        sum_recursive(&serde_json::from_str(&input.as_str()).unwrap(), false)
    );
    println!(
        "Part two: {}",
        sum_recursive(&serde_json::from_str(&input.as_str()).unwrap(), true)
    );
}
