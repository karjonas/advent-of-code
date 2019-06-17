extern crate common;

fn step(recipes: &mut Vec<usize>, elves: &mut Vec<usize>) {
    let new_recipe = recipes[elves[0]] + recipes[elves[1]];
    let recipe_a = new_recipe / 10;
    let recipe_b = new_recipe % 10;

    if recipe_a != 0 {
        recipes.push(recipe_a);
    }
    recipes.push(recipe_b);

    for i in 0..elves.len() {
        elves[i] = (elves[i] + recipes[elves[i]] + 1) % recipes.len();
    }
}

fn part_one(recipes: &mut Vec<usize>, elves: &mut Vec<usize>) -> String {
    let input_part_one = 702831;
    for _i in 0..input_part_one + 10 {
        step(recipes, elves);
    }

    let mut result = String::new();
    for i in input_part_one..input_part_one + 10 {
        result += &recipes[i].to_string();
    }

    return result;
}

fn part_two(recipes: &mut Vec<usize>, elves: &mut Vec<usize>) -> String {
    let input_part_two = vec![7, 0, 2, 8, 3, 1];
    let goal_len = input_part_two.len();
    loop {
        let num_recipes_last = recipes.len();
        step(recipes, elves);
        let num_added = recipes.len() - num_recipes_last;
        for k in 0..num_added {
            if recipes.len() > goal_len + k {
                let mut done = true;
                for i in 0..goal_len {
                    let j = (recipes.len() - goal_len) + i - k;
                    if input_part_two[i] != recipes[j] {
                        done = false;
                        break;
                    }
                }
                if done {
                    return (recipes.len() - goal_len - k).to_string();
                }
            }
        }
    }
}

pub fn solve() {
    let recipes: Vec<usize> = vec![3, 7];
    let elves: Vec<usize> = vec![0, 1];

    println!(
        "Part one: {}",
        part_one(&mut recipes.clone(), &mut elves.clone())
    );

    println!(
        "Part two: {}",
        part_two(&mut recipes.clone(), &mut elves.clone())
    );
}
