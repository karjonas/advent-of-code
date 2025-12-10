extern crate good_lp;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel, Variable};

fn parse_input(input: &String) -> Vec<(usize, (Vec<usize>, Vec<Vec<usize>>), Vec<usize>)> {
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    let mut result = Vec::new();
    for line in input.lines() {
        let mut goal = 0;
        let mut buttons: (Vec<usize>, Vec<Vec<usize>>) = (Vec::new(), Vec::new());
        let mut requirements = Vec::new();

        for part_str in line.split_ascii_whitespace() {
            let mut part = part_str.to_string();
            part.pop();
            let first_char = part.remove(0);

            let chars: Vec<char> = part.chars().collect();
            if first_char == '[' {
                for i in 0..chars.len() {
                    let bit = (chars[i] == '#') as usize;
                    goal |= bit << i;
                }
            } else if first_char == '(' {
                let mut button = Vec::new();
                let mut button_bits = 0;
                for i in part.split(',').map(|v| v.parse::<usize>().unwrap()) {
                    button_bits |= 1 << i;
                    button.push(i);
                }
                buttons.0.push(button_bits);
                buttons.1.push(button);
            } else if first_char == '{' {
                for v in part.split(',').map(|v| v.parse::<usize>().unwrap()) {
                    requirements.push(v);
                }
            }
        }
        result.push((goal, buttons, requirements));
    }
    return result;
}

fn part_one(machines: &Vec<(usize, (Vec<usize>, Vec<Vec<usize>>), Vec<usize>)>) -> usize {
    let mut result = 0;
    for (goal, (buttons, _), _) in machines.clone() {
        let mut hit_states: HashSet<usize> = HashSet::new();

        // (state, num moves)
        let mut states: VecDeque<(usize, usize)> = VecDeque::new();
        states.reserve(100);
        states.push_back((0, 0));
        let mut done = false;

        while !states.is_empty() {
            let (state, moves) = states.pop_front().unwrap();
            if hit_states.contains(&state) {
                continue;
            }
            hit_states.insert(state);

            for button in &buttons {
                let state_next = state ^ *button;
                let moves_next = moves + 1;

                if state_next == goal {
                    result += moves_next;
                    done = true;
                    break;
                }

                states.push_back((state_next, moves_next));
            }

            if done {
                break;
            }
        }
    }
    return result;
}

fn part_two(machines: &Vec<(usize, (Vec<usize>, Vec<Vec<usize>>), Vec<usize>)>) -> usize {
    let mut result = 0;
    for (_, (_, button_indices), joltage_goal) in machines.clone() {
        let mut vars = variables!();
        let mut var_map: HashMap<String, Variable> = HashMap::new();
        let mut total_presses_objective = Expression::from(0);

        let num_buttons = button_indices.len();

        for i in 0..num_buttons {
            let name = format!("button_{}", i);
            let v = vars.add(variable().min(0).integer());
            var_map.insert(name, v);
            total_presses_objective += v;
        }

        let mut constraints_goal_map: Vec<Vec<String>> = vec![Vec::new(); joltage_goal.len()];

        for var_idx in 0..num_buttons {
            let affected_counters_indices = &button_indices[var_idx];
            let var_name = format!("button_{}", var_idx);
            for &goal_idx in affected_counters_indices {
                constraints_goal_map[goal_idx].push(var_name.clone());
            }
        }

        let mut generated_constraints = Vec::new();

        for goal_idx in 0..joltage_goal.len() {
            let goal_value = joltage_goal[goal_idx];
            let variables_for_this_goal = &constraints_goal_map[goal_idx];

            let mut constraint_lhs = Expression::from(0);

            for var_name in variables_for_this_goal {
                if let Some(&var_handle) = var_map.get(var_name) {
                    constraint_lhs += var_handle;
                }
            }

            generated_constraints.push(constraint_lhs.eq(goal_value as u32));
        }

        let mut model = vars
            .minimise(total_presses_objective.clone())
            .using(default_solver);

        for constraint in generated_constraints {
            model = model.with(constraint);
        }

        let solution = model.solve().unwrap();
        let total_clicks = solution.eval(&total_presses_objective) as usize;
        result += total_clicks;
    }

    return result;
}

pub fn solve(filepath: &str) {
    let input = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end_matches('\n')
        .to_string();
    let machines = parse_input(&input);
    println!("Part one: {}", part_one(&machines));
    println!("Part two: {}", part_two(&machines));
}
