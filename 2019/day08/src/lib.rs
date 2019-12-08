extern crate common;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const LAYER_SIZE: usize = IMAGE_HEIGHT * IMAGE_WIDTH;

const PIXEL_BLACK: u8 = 0;
const PIXEL_WHITE: u8 = 1;
// const PIXEL_TRANS: u8 = 2;

fn parse_input() -> Vec<Vec<u8>> {
    let input: Vec<u8> = common::read_file("2019/day08/input")
        .chars()
        .map(|v| common::char_to_u8(v))
        .collect();

    let mut layers = Vec::new();
    assert_eq!(input.len() % LAYER_SIZE, 0);
    let num_layers = input.len() / LAYER_SIZE;

    for layer_idx in 0..num_layers {
        let mut layer = Vec::new();
        layer.reserve(LAYER_SIZE);
        for pixel_idx in 0..LAYER_SIZE {
            layer.push(input[layer_idx * LAYER_SIZE + pixel_idx]);
        }
        layers.push(layer);
    }
    return layers;
}

fn solve_part_one() -> usize {
    let layers = parse_input();
    let mut layer_id_fewest_zeroes = 0;
    let mut num_zeroes = std::usize::MAX;
    for layer_idx in 0..layers.len() {
        let num_zeroes_layer = layers[layer_idx]
            .iter()
            .fold(0, |sum, v| sum + if v.clone() == 0 { 1 } else { 0 });

        if num_zeroes_layer < num_zeroes {
            num_zeroes = num_zeroes_layer;
            layer_id_fewest_zeroes = layer_idx;
        }
    }

    let num_ones = layers[layer_id_fewest_zeroes]
        .iter()
        .fold(0, |sum, v| sum + if v.clone() == 1 { 1 } else { 0 });
    let num_twos = layers[layer_id_fewest_zeroes]
        .iter()
        .fold(0, |sum, v| sum + if v.clone() == 2 { 1 } else { 0 });
    return num_ones * num_twos;
}

fn solve_part_two() -> String {
    let layers = parse_input();
    let mut result = String::new();

    for i in 0..LAYER_SIZE {
        let mut pixel = layers[0][i];
        for layer_idx in 1..layers.len() {
            if pixel == PIXEL_BLACK || pixel == PIXEL_WHITE {
                break;
            }
            pixel = layers[layer_idx][i];
        }

        if i % IMAGE_WIDTH == 0 {
            result.push('\n');
        }
        let c = if pixel == PIXEL_BLACK { ' ' } else { '#' };
        result.push(c);
    }

    return result;
}

pub fn solve() {
    println!("Part one: {}", solve_part_one());
    println!("Part two:{}", solve_part_two());
}
