use anyhow::Result;
use std::io::{self, BufRead};
use std::{fs::File, path::Path};

type Report = Vec<i32>;
type Reports = Vec<Report>;

pub fn parse_input_file(path: &Path) -> Result<Reports> {
    let file = File::open(path)?;
    let lines = io::BufReader::new(file).lines();

    let mut reports: Reports = Vec::new();
    for line in lines.flatten() {
        let report: Report = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
    }

    return Ok(reports);
}

fn is_ascending_report_safe(report: &Report) -> bool {
    for window in report.windows(2) {
        let distance = window[1] - window[0];
        if distance < 1 || distance > 3 {
            return false;
        }
    }
    return true;
}

pub fn step1(reports: &Reports) -> i32 {
    let mut safe_count = 0;
    for report in reports {
        let reversed_report: Vec<i32> = report.iter().rev().copied().collect();
        if is_ascending_report_safe(report) || is_ascending_report_safe(&reversed_report) {
            safe_count += 1;
        }
    }

    return safe_count;
}

fn find_ascending_unsafe_level(report: &Report) -> Option<usize> {
    for (index, window) in report.windows(2).enumerate() {
        let distance = window[1] - window[0];
        if distance < 1 || distance > 3 {
            return Some(index);
        }
    }

    return None;
}

fn is_report_safe_with_one_margin(report: &Report) -> bool {
    let reversed_report: Vec<i32> = report.iter().rev().copied().collect();
    for current_report in [report, &reversed_report] {
        let Some(index) = find_ascending_unsafe_level(current_report) else {
            return true;
        };
        for offset in 0..2 {
            let mut report_without_index = current_report.clone();
            report_without_index.remove(index + offset);
            if is_ascending_report_safe(&report_without_index) {
                return true;
            }
        }
    }

    return false;
}

pub fn step2(reports: &Reports) -> i32 {
    let mut safe_count = 0;
    for report in reports {
        if is_report_safe_with_one_margin(report) {
            safe_count += 1;
        }
    }

    return safe_count;
}
