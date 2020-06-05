extern crate common;
extern crate intcode;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct State {
    memory: Vec<i64>,
    input_numbers: VecDeque<i64>,
    index: usize,
    relative_base: i64,
    position: String,
    command: String,
    previous: String,
    output: String,
}

#[derive(Debug, Clone)]
struct Network {
    // Room -> (Room, Direction)
    network: HashMap<String, Vec<(String, String)>>,
    items: BTreeSet<(String, String)>,
    places: BTreeSet<String>,
}

const DANGEROUS_ITEMS: [&str; 5] = [
    "escape pod",
    "giant electromagnet",
    "infinite loop",
    "molten lava",
    "photons",
];

const START_AREA: &str = "== Hull Breach ==";
const END_AREA: &str = "== Pressure-Sensitive Floor ==";

fn pickup(state: State, thing: String) -> State {
    let mut result = state.clone();
    let command = "take ".to_string() + thing.as_str() + "\n";
    let (memory_next, output_numbers, index_next, relative_base_next, _halted) = intcode::run(
        state.memory.clone(),
        VecDeque::from(intcode::string_to_ascii(command.as_str())),
        state.index.clone(),
        state.relative_base.clone(),
    );

    result.memory = memory_next;
    result.output = intcode::to_ascii(Vec::from(output_numbers));
    result.index = index_next;
    result.relative_base = relative_base_next;

    return result;
}

fn run_input(state: State, input: String) -> State {
    let (memory_next, output_numbers, index_next, relative_base_next, _halted) = intcode::run(
        state.memory.clone(),
        VecDeque::from(intcode::string_to_ascii(input.as_str())),
        state.index.clone(),
        state.relative_base.clone(),
    );

    let mut result = state.clone();
    result.memory = memory_next;
    result.index = index_next;
    result.relative_base = relative_base_next;
    result.output = intcode::to_ascii(output_numbers);
    return result;
}

fn goto_place(start_state: State, network: &Network, destination: String) -> State {
    let mut stack = Vec::new();
    let mut result = start_state.clone();

    stack.push(start_state);
    let mut visited = BTreeSet::new();

    while !stack.is_empty() {
        let state = stack.pop().unwrap();
        if visited.contains(&state.position) {
            continue;
        }
        visited.insert(state.position.clone());

        let (memory_next, output_numbers, index_next, relative_base_next, _halted) = intcode::run(
            state.memory.clone(),
            state.input_numbers.clone(),
            state.index.clone(),
            state.relative_base.clone(),
        );

        let s = intcode::to_ascii(output_numbers);

        if state.position == destination {
            result.memory = memory_next;
            result.index = index_next;
            result.relative_base = relative_base_next;
            result.position = destination.clone();
            result.output = s.clone();
            return result;
        }

        let neighs = network.network.get(&state.position).unwrap();
        for (room, direction) in neighs {
            let command = intcode::string_to_ascii(direction.as_str());

            let state_next = State {
                memory: memory_next.clone(),
                input_numbers: VecDeque::from(command),
                index: index_next,
                relative_base: relative_base_next,
                position: room.clone(),
                command: direction.clone(),
                previous: state.position.clone(),
                output: s.clone(),
            };

            stack.push(state_next);
        }
    }

    panic!("Could not find place");
}

fn solve_part_one(memory: Vec<i64>, network: Network) -> usize {
    let mut state = State {
        memory: memory.clone(),
        input_numbers: VecDeque::<i64>::new(),
        index: 0,
        relative_base: 0,
        position: START_AREA.to_string(),
        command: String::new(),
        previous: String::new(),
        output: String::new(),
    };

    let mut safe_items = Vec::new();
    for (item, _place) in network.items.clone() {
        if DANGEROUS_ITEMS.contains(&item.as_str()) {
            continue;
        }

        safe_items.push(item);
    }

    // Collect all items
    for (item, place) in network.items.clone() {
        if !safe_items.contains(&item) {
            continue;
        }
        state = goto_place(state.clone(), &network, place.clone());
        state = pickup(state.clone(), item.clone());
    }

    let item_perms = common::subsequences(&safe_items, 0, &Vec::new());

    // Goto Security Checkpoint
    state = goto_place(state, &network, "== Security Checkpoint ==".to_string());
    let state_security = state.clone();

    // Check all permutations
    for items in item_perms {
        let mut state_test = state_security.clone();

        // Drop all items not in the permutation
        for item_safe in &safe_items {
            if !items.contains(&item_safe) {
                let command = format!("drop {}\n", item_safe);
                state_test = run_input(state_test.clone(), command);
            }
        }

        // Go to == Pressure-Sensitive Floor ==
        state_test = run_input(state_test.clone(), "south\n".to_string());
        // Slower but might be needed on different input
        // state_test = goto_place(state_test.clone(), &network, "== Pressure-Sensitive Floor ==".to_string());

        if !state_test.output.contains("Alert!") {
            let number = state_test.output.lines().collect::<Vec<_>>()[11]
                .split(" ")
                .collect::<Vec<_>>()[11];
            return common::string_to_usize(number);
        }
    }

    return 0;
}

fn create_network(memory: Vec<i64>) -> Network {
    let mut network = Network {
        network: HashMap::new(),
        items: BTreeSet::new(),
        places: BTreeSet::new(),
    };

    let mut stack = Vec::new();

    stack.push(State {
        memory: memory.clone(),
        input_numbers: VecDeque::<i64>::new(),
        index: 0,
        relative_base: 0,
        position: START_AREA.to_string(),
        command: String::new(),
        previous: String::new(),
        output: String::new(),
    });

    let mut visited = BTreeSet::new();

    while !stack.is_empty() {
        let state = stack.pop().unwrap();
        if visited.contains(&(state.position.clone(), state.command.clone())) {
            continue;
        }

        visited.insert((state.position.clone(), state.command.clone()));
        if !state.position.is_empty() {
            network.places.insert(state.position.clone());
        }

        let (memory_next, output_numbers, index_next, relative_base_next, _halted) = intcode::run(
            state.memory,
            state.input_numbers,
            state.index,
            state.relative_base,
        );

        let lines = intcode::to_ascii(output_numbers)
            .lines()
            .map(|v| String::from(v))
            .collect::<Vec<String>>();
        let mut looking_name = true;
        let mut looking_dirs = false;
        let mut storing_dir = false;
        let mut looking_items = false;
        let mut storing_items = false;

        let mut name = String::new();
        //let mut objects_found = Vec::new();
        let mut dirs_found = Vec::new();

        if lines[3] == END_AREA {
            if !state.position.is_empty() {
                let node = network
                    .network
                    .entry(state.position.clone())
                    .or_insert(Vec::new());
                node.push((END_AREA.to_string(), state.command.clone()));

                // Hack to make sure dead ends are in list
                network
                    .network
                    .entry(END_AREA.to_string())
                    .or_insert(Vec::new());
            }
            continue;
        }

        for line in lines {
            if looking_name && !line.is_empty() {
                name = line.clone();
                looking_dirs = true;
                looking_name = false;
            } else if looking_dirs && line == "Doors here lead:" {
                storing_dir = true;
                looking_dirs = false;
            } else if storing_dir {
                if line.is_empty() {
                    looking_items = true;
                    storing_dir = false;
                    continue;
                }

                let dir = String::from(line.split(" ").collect::<Vec<_>>()[1]);
                dirs_found.push(dir);
            } else if looking_items && line == "Items here:" {
                storing_items = true;
                looking_items = false;
            } else if storing_items && !line.is_empty() && line != "Command?" {
                let item = String::from(line.split("-").collect::<Vec<_>>()[1].trim());
                network.items.insert((item, name.clone()));
            }
        }

        if !state.position.is_empty() {
            let node = network
                .network
                .entry(state.position.clone())
                .or_insert(Vec::new());
            node.push((name.clone(), state.command.clone()));

            // Hack to make sure dead ends are in list
            network.network.entry(name.clone()).or_insert(Vec::new());
        }

        for dir in dirs_found {
            let mut dir_fixed = dir.clone();
            dir_fixed.push('\n');
            let state_next = State {
                memory: memory_next.clone(),
                input_numbers: VecDeque::from(intcode::string_to_ascii(dir_fixed.as_str())),
                index: index_next,
                relative_base: relative_base_next,
                position: name.clone(),
                command: dir_fixed.clone(),
                previous: state.position.clone(),
                output: String::new(),
            };

            stack.push(state_next);
        }
    }

    return network;
}

pub fn solve() {
    let input = common::read_file("2019/day25/input");
    let memory = intcode::parse_input(input.as_str());
    let network = create_network(memory.clone());

    println!("Part one: {}", solve_part_one(memory, network));
}
