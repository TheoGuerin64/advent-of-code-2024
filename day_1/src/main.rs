use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

fn parse_input_file(path: &Path) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();

    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        if let Some((first, second)) = line.split_once(' ') {
            let second_trimed = second.trim_start();
            list_1.push(first.parse::<i32>().unwrap());
            list_2.push(second_trimed.parse::<i32>().unwrap());
        }
    }

    if list_1.len() != list_2.len() {
        return Err(anyhow::anyhow!("Lists are not the same length"));
    }

    return Ok((list_1, list_2));
}

fn step1(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut distance = 0;
    for (i1, i2) in list1.iter().zip(list2) {
        distance += (i1 - i2).abs();
    }

    return distance;
}

fn step2(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut list2_count: HashMap<i32, i32> = HashMap::new();
    for i in list2.iter() {
        if let Some(count) = list2_count.get(i) {
            list2_count.insert(*i, count + 1);
        } else {
            list2_count.insert(*i, 1);
        }
    }

    let mut similarity = 0;
    for i in list1.iter() {
        similarity += list2_count.get(i).unwrap_or(&0) * i;
    }

    return similarity;
}

fn main() {
    let (mut list1, mut list2) = parse_input_file(Path::new("input.txt")).unwrap();
    list1.sort();
    list2.sort();

    println!("step1: {:?}", step1(&list1, &list2));
    println!("step2: {:?}", step2(&list1, &list2));
}
