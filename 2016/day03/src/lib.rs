fn valid_tri(a: i32, b: i32, c: i32) -> bool {
    return a + b > c && b + c > a && c + a > b;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let tris: Vec<Vec<_>> = input
        .lines()
        .map(|v| {
            v.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut tris_flip: Vec<Vec<i32>> = Vec::new();

    for i in 0..(tris.len() / 3) {
        for j in 0..3 {
            tris_flip.push(vec![
                tris[i * 3][j],
                tris[(i * 3) + 1][j],
                tris[(i * 3) + 2][j],
            ]);
        }
    }

    let num_ok = tris.iter().fold(0, |sum, t| {
        sum + if valid_tri(t[0], t[1], t[2]) { 1 } else { 0 }
    });
    let num_ok_flip = tris_flip.iter().fold(0, |sum, t| {
        sum + if valid_tri(t[0], t[1], t[2]) { 1 } else { 0 }
    });

    println!("Part 1: {}", num_ok);
    println!("Part 2: {}", num_ok_flip);
}
