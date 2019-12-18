extern crate common;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Material {
    name: String,
    quantity: usize,
    deps: Vec<Material>,
}

const TRILLION: usize = 1000000000000;

fn parse_input(input: &str) -> HashMap<String, Material> {
    let mut materials: HashMap<String, Material> = HashMap::new();
    for line in input.lines() {
        let v = line
            .split(" => ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        assert_eq!(v.len(), 2);
        let vr = v[1].split(' ').collect::<Vec<_>>();
        let dest_num = common::string_to_usize(vr[0]);
        let dest_name = String::from(vr[1]);

        let mut deps = Vec::new();

        for dep_string in v[0].split(", ").collect::<Vec<_>>() {
            let dep = dep_string.split(" ").collect::<Vec<_>>();
            assert_eq!(dep.len(), 2);
            let src_num = common::string_to_usize(dep[0]);
            let src_name = String::from(dep[1]);

            let m = Material {
                name: src_name,
                quantity: src_num,
                deps: Vec::new(),
            };
            deps.push(m);
        }

        let m = Material {
            name: dest_name.clone(),
            quantity: dest_num,
            deps: deps,
        };

        materials.insert(dest_name, m);
    }
    return materials;
}

fn solve_recursive(
    curr_material: String,
    num_needed: usize,
    materials: &HashMap<String, Material>,
    store: &mut HashMap<String, usize>,
    used_ore: &mut usize,
) {
    if curr_material == "ORE" {
        let num_stored = store.entry(curr_material.clone()).or_insert(0).clone();
        if num_stored < num_needed {
            let num_added = num_needed - num_stored;
            *used_ore += num_added;
            store.insert(curr_material.clone(), num_needed);
        }
        return;
    }

    let material = materials.get(&curr_material).unwrap();

    let num_stored = store.entry(curr_material.clone()).or_insert(0).clone();
    if num_stored >= num_needed {
        return;
    }
    let quotient = ((num_needed - num_stored) as f64 / material.quantity as f64).ceil() as usize;

    for dep in &material.deps {
        let mut num_mined_dep = store.entry(dep.name.clone()).or_insert(0).clone();
        let dep_tot = dep.quantity * quotient;
        if num_mined_dep < dep_tot {
            solve_recursive(dep.name.clone(), dep_tot, materials, store, used_ore);
        }
        num_mined_dep = store.entry(dep.name.clone()).or_insert(0).clone();
        let num_left = num_mined_dep - dep_tot;
        store.insert(dep.name.clone(), num_left);
    }
    let num_new = num_stored + (material.quantity * quotient);
    store.insert(curr_material.clone(), num_new);
}

fn solve_input(materials: &HashMap<String, Material>) -> usize {
    let mut used_ore = 0;
    solve_recursive(
        String::from("FUEL"),
        1,
        materials,
        &mut HashMap::new(),
        &mut used_ore,
    );
    //println!("{:?}", materials);
    return used_ore;
}

fn solve_input_p2(materials: &HashMap<String, Material>) -> usize {
    let mut fuel_low = 1;
    let mut fuel_high = TRILLION;
    let mut curr = fuel_high / 2;
    loop {
        if fuel_low == fuel_high {
            break;
        }
        let mut used_ore = 0;
        solve_recursive(
            String::from("FUEL"),
            curr,
            materials,
            &mut HashMap::new(),
            &mut used_ore,
        );

        if used_ore > TRILLION {
            fuel_high = curr - 1;
            curr = fuel_low + (fuel_high - fuel_low) / 2;
        } else {
            fuel_low = curr + 1;
            curr = fuel_low + (fuel_high - fuel_low) / 2;
        }
    }

    return fuel_high;
}

pub fn solve() {
    let input = common::read_file("2019/day14/input");
    let materials = parse_input(input.as_str());

    println!("Part one: {}", solve_input(&materials.clone()));
    println!("Part two: {}", solve_input_p2(&materials.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_part_one_a() {
        let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        assert_eq!(solve_input(&parse_input(input)), 31);
    }

    #[test]
    fn test_samples_part_one_b() {
        let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(solve_input(&parse_input(input)), 165);
    }

    #[test]
    fn test_samples_part_one_c() {
        let input = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(solve_input(&parse_input(input)), 13312);
    }
}
