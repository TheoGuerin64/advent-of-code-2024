use std::path::Path;

use advent_of_code_2024::{parse_input_file, step1, step2};

fn main() {
    let reports = parse_input_file(Path::new("input.txt")).unwrap();
    println!("step1: {:?}", step1(&reports));
    println!("step2: {:?}", step2(&reports));
}
