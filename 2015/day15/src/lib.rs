extern crate common;

#[derive(Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn enumerate_amounts(amounts: &Vec<usize>, index: usize, units_left: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    if index == amounts.len() {
        if units_left == 0 {
            result.push(amounts.clone());
        }
        return result;
    }

    for i in 0..units_left + 1 {
        let mut amounts_curr = amounts.clone();
        amounts_curr[index] = i;
        let mut result_recursive = enumerate_amounts(&amounts_curr, index + 1, units_left - i);
        result.append(&mut result_recursive);
    }

    return result;
}

fn solve_internal(input: &String) -> (usize, usize) {
    let input_strip = common::strip_characters(input.as_str(), ":,");

    let mut ingredients = Vec::new();

    for line in input_strip.lines() {
        let words = line
            .split_whitespace()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
        let ingredient = Ingredient {
            capacity: common::string_to_i64(words[2].as_str()),
            durability: common::string_to_i64(words[4].as_str()),
            flavor: common::string_to_i64(words[6].as_str()),
            texture: common::string_to_i64(words[8].as_str()),
            calories: common::string_to_i64(words[10].as_str()),
        };

        ingredients.push(ingredient);
    }

    let possible_amounts = enumerate_amounts(&common::filled_vector(ingredients.len(), 0), 0, 100);

    let mut best_score = 0;
    let mut best_score_lowcal = 0;

    for amounts in &possible_amounts {
        let mut capacity: i64 = 0;
        let mut durability: i64 = 0;
        let mut flavor: i64 = 0;
        let mut texture: i64 = 0;
        let mut calories: i64 = 0;

        for i in 0..amounts.len() {
            capacity += ingredients[i].capacity * (amounts[i] as i64);
            durability += ingredients[i].durability * (amounts[i] as i64);
            flavor += ingredients[i].flavor * (amounts[i] as i64);
            texture += ingredients[i].texture * (amounts[i] as i64);
            calories += ingredients[i].calories * (amounts[i] as i64);
        }

        capacity = std::cmp::max(0, capacity);
        durability = std::cmp::max(0, durability);
        flavor = std::cmp::max(0, flavor);
        texture = std::cmp::max(0, texture);

        let score = capacity * durability * flavor * texture;
        best_score = std::cmp::max(best_score, score);

        if calories == 500 {
            best_score_lowcal = std::cmp::max(best_score_lowcal, score);
        }
    }

    return (best_score as usize, best_score_lowcal as usize);
}

pub fn solve() {
    let input = common::read_file("2015/day15/input");
    let (p1, p2) = solve_internal(&input);
    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        assert_eq!(solve_internal(&input.to_string()), (62842880, 57600000));
    }
}
