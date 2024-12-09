use anyhow::Result;
use std::fs::read_to_string;
use std::path::Path;

pub fn parse_input_file(path: &Path) -> Result<String> {
    let text: String = read_to_string(path)?;
    return Ok(text);
}

fn multiply(text: &str, mut index: usize) -> Option<i32> {
    index += 4;

    let next_comma = text[index..].find(',')?;
    let Ok(arg1) = text[index..index + next_comma].parse::<i32>() else {
        return None;
    };
    index += next_comma + 1;

    let next_closing_bracket = text[index..].find(')')?;
    let Ok(arg2) = text[index..index + next_closing_bracket].parse::<i32>() else {
        return None;
    };

    return Some(arg1 * arg2);
}

pub fn step1(text: &str) -> i32 {
    let mut sum: i32 = 0;
    for (index, char) in text.chars().enumerate() {
        if char == 'm' && text[index + 1..].starts_with("ul(") {
            if let Some(product) = multiply(text, index) {
                sum += product;
            }
        }
    }

    return sum;
}

enum Value {
    Do,
    Dont,
    Mul(i32),
}

fn parse(text: &str) -> Vec<Value> {
    let mut values = Vec::new();
    for (index, char) in text.chars().enumerate() {
        match char {
            'm' => {
                if text[index + 1..].starts_with("ul(") {
                    if let Some(product) = multiply(text, index) {
                        values.push(Value::Mul(product));
                    }
                }
            }
            'd' => {
                if text[index + 1..].starts_with("o()") {
                    values.push(Value::Do);
                } else if text[index + 1..].starts_with("on't()") {
                    values.push(Value::Dont);
                }
            }
            _ => {}
        }
    }

    return values;
}

pub fn step2(text: &str) -> i32 {
    let values = parse(text);

    let mut sum = 0;
    let mut enabled = true;
    for value in values {
        match value {
            Value::Do => {
                enabled = true;
            }
            Value::Dont => {
                enabled = false;
            }
            Value::Mul(product) => {
                if enabled {
                    sum += product;
                }
            }
        }
    }

    return sum;
}
