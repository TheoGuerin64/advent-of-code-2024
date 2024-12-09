use std::fs::read_to_string;

use advent_of_code_2024::{step1, step2};

fn main() {
    let input = read_to_string("input.txt").expect("Input file not found");
    println!("step1: {:?}", step1(&input));
    println!("step2: {:?}", step2(&input));
}
