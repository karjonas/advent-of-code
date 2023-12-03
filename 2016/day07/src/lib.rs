struct Entry {
    free_strs: Vec<String>,
    bracket_strs: Vec<String>,
    is_ssl: bool,
}

fn add_abas(v: &mut Vec<String>, s: &String, flip: bool) {
    let bts = s.as_bytes();
    for i in 0..(s.len() - 2) {
        if bts[i] != bts[i + 1] && bts[i] == bts[i + 2] {
            let mut aba_str = String::new();
            if flip {
                aba_str.push(bts[i + 1] as char);
                aba_str.push(bts[i] as char);
                aba_str.push(bts[i + 1] as char);
            } else {
                aba_str.push(bts[i] as char);
                aba_str.push(bts[i + 1] as char);
                aba_str.push(bts[i] as char);
            };

            v.push(aba_str);
        }
    }
}

fn parse_entry(line: &str) -> Entry {
    let mut entry = Entry {
        free_strs: Vec::new(),
        bracket_strs: Vec::new(),
        is_ssl: false,
    };

    let mut abas: Vec<String> = Vec::new();
    let mut babs: Vec<String> = Vec::new();
    let mut str_curr = String::new();

    for c in line.chars() {
        if c == '[' {
            entry.free_strs.push(str_curr.clone());
            add_abas(&mut abas, &str_curr, false);
            str_curr.clear();
        } else if c == ']' {
            entry.bracket_strs.push(str_curr.clone());
            add_abas(&mut babs, &str_curr, true);
            str_curr.clear();
        } else {
            str_curr.push(c);
        }
    }

    add_abas(&mut abas, &str_curr, false);
    entry.free_strs.push(str_curr);

    abas.sort();
    abas.dedup();
    babs.sort();
    babs.dedup();
    for s0 in &abas {
        for s1 in &babs {
            if s0 == s1 {
                entry.is_ssl = true;
                break;
            }
        }
    }

    return entry;
}

fn has_abba(s: &String) -> bool {
    let bts = s.as_bytes();
    for i in 0..(s.len() - 3) {
        if bts[i] != bts[i + 1] && bts[i] == bts[i + 3] && bts[i + 1] == bts[i + 2] {
            return true;
        }
    }

    return false;
}

fn verify_entry(entry: &Entry) -> bool {
    let num_abbas_free =
        entry.free_strs.iter().fold(
            0,
            |sum, string| if has_abba(string) { sum + 1 } else { sum },
        );
    let num_abbas_boxed =
        entry.bracket_strs.iter().fold(
            0,
            |sum, string| if has_abba(string) { sum + 1 } else { sum },
        );

    return num_abbas_free > 0 && num_abbas_boxed == 0;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    let mut num_ok = 0;
    let mut num_ssl = 0;

    for line in input.lines() {
        let entry = parse_entry(line);
        let ok = verify_entry(&entry);
        num_ok += if ok { 1 } else { 0 };
        num_ssl += if entry.is_ssl { 1 } else { 0 };
    }

    println!("Part 1: {:?}", num_ok);
    println!("Part 2: {:?}", num_ssl);
}
