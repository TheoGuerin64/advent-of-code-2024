use anyhow::Result;
use std::collections::HashMap;

fn parse(text: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();
    for line in text.lines() {
        if let Some((first, second)) = line.split_once(' ') {
            let second_trimed = second.trim_start();
            list_1.push(first.parse::<i32>().unwrap());
            list_2.push(second_trimed.parse::<i32>().unwrap());
        }
    }

    return Ok((list_1, list_2));
}

pub fn step1(text: &str) -> i32 {
    let (mut list1, mut list2) = parse(text).unwrap();
    list1.sort();
    list2.sort();

    let mut distance = 0;
    for (i1, i2) in list1.iter().zip(list2) {
        distance += (i1 - i2).abs();
    }

    return distance;
}

pub fn step2(text: &str) -> i32 {
    let (list1, list2) = parse(text).unwrap();

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
