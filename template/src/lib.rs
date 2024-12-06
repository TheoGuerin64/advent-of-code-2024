use anyhow::Result;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

pub type Data = String;

pub fn parse_input_file(path: &Path) -> Result<Data> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    for line in lines.flatten() {}

    return Ok();
}

pub fn step1(data: &Data) {}

pub fn step2(data: &Data) {}
