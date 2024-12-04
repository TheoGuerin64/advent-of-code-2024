use anyhow::Result;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn parse_input_file(path: &Path) -> Result {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines.flatten() {}

    return Ok();
}

fn step1() {}

fn step2() {}

fn main() {
    let data = parse_input_file(Path::new("input.txt")).unwrap();

    println!("step1: {:?}", step1());
    println!("step2: {:?}", step2());
}
