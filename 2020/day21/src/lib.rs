extern crate common;

use std::collections::BTreeMap;
use std::collections::HashMap;

type Food = (Vec<String>, Vec<String>);

fn parse_input(input: &String) -> Vec<Food> {
    let mut foods = Vec::new();
    for line in input.lines() {
        let words: Vec<_> = line.split(" ").collect();
        let mut is_reading_ingredients = true;
        let mut ingredients = Vec::new();
        let mut allergents = Vec::new();
        for word in words {
            let word_trimmed = word
                .chars()
                .filter(|c| !['(', ')', ','].contains(c))
                .collect::<String>();

            if word_trimmed == "contains" {
                is_reading_ingredients = false;
                continue;
            }

            if is_reading_ingredients {
                ingredients.push(word_trimmed);
            } else {
                allergents.push(word_trimmed);
            }
        }

        foods.push((ingredients, allergents));
    }

    return foods;
}

fn solve_both(input: &Vec<Food>) -> (usize, String) {
    let mut allergen_to_ingredients: HashMap<String, Vec<String>> = HashMap::new();
    for (ingredients, allergens) in input {
        for allergen in allergens {
            allergen_to_ingredients
                .entry(allergen.clone())
                .or_insert(ingredients.clone())
                .retain(|v| ingredients.contains(v));
        }
    }

    // Go through allergens and find ones with only one ingredient, remove ingredient and repeat.
    let mut bad_ingredients = Vec::new();
    let mut allergen_to_bad_ingredient = BTreeMap::new();
    let mut running = true;
    while running {
        running = false;
        let mut allergen_to_ingredients_next = allergen_to_ingredients.clone();

        for (allergen, ingredients) in &allergen_to_ingredients {
            if ingredients.len() == 1 {
                let ingredient = &ingredients[0];
                running = true;

                bad_ingredients.push(ingredient.clone());
                allergen_to_bad_ingredient.insert(allergen.clone(), ingredient.clone());

                allergen_to_ingredients_next.remove(allergen);
                for (_allergen, ingredients) in &mut allergen_to_ingredients_next {
                    ingredients.retain(|v| v != ingredient);
                }
            }
        }
        allergen_to_ingredients = allergen_to_ingredients_next;
    }

    // Sum good ingredients
    let good_sum = input
        .iter()
        .map(|(ingredients, _)| {
            ingredients
                .iter()
                .map(|ingredient| !bad_ingredients.contains(ingredient) as usize)
                .sum::<usize>()
        })
        .sum::<usize>();

    // Concat bad ingredients
    let bad_concat = allergen_to_bad_ingredient
        .iter()
        .map(|(_, ingredient)| ingredient.clone())
        .collect::<Vec<_>>()
        .join(",");

    return (good_sum, bad_concat);
}

fn part_one(input: &Vec<Food>) -> usize {
    return solve_both(input).0;
}

fn part_two(input: &Vec<Food>) -> String {
    return solve_both(input).1;
}

pub fn solve() {
    let input = parse_input(&common::read_file("2020/day21/input"));

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let input = [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ]
        .join("\n");

        assert_eq!(part_one(&parse_input(&input)), 5);
        assert_eq!(part_two(&parse_input(&input)), "mxmxvkd,sqjhc,fvjkl");
    }
}
