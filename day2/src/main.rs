use std::fs;
use std::io::{self, BufRead};

fn check_report(numbers: &Vec<i32>) -> Option<usize> {
    let diffnumbers: Vec<i32> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

    let unbounded = diffnumbers.iter().map(|x| x.abs()).position(|x| x > 3);

    if let Some(index) = unbounded {
        return Some(index);
    }

    let positive_steps = diffnumbers.iter().filter(|&&x| x > 0).count();
    let negative_steps = diffnumbers.iter().filter(|&&x| x < 0).count();
    let monotonic = positive_steps == diffnumbers.len() || negative_steps == diffnumbers.len();

    if monotonic {
        None
    } else {
        if positive_steps < negative_steps {
            return Some(diffnumbers.iter().position(|&x| x >= 0).unwrap());
        } else {
            return Some(diffnumbers.iter().position(|&x| x <= 0).unwrap());
        }
    }
}

fn remove_level(report: &Vec<i32>, level: usize) -> Vec<i32> {
    let mut new_report = report.clone();

    if level >= report.len() {
        return new_report;
    }

    new_report.remove(level);
    new_report
}

fn main() {
    println!("Advent of Code 2024 - Day 2");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;
    let mut near_safe_reports = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Error parsing number"))
            .collect();

        let safety = check_report(&report);

        match safety {
            Some(unsafe_level) => {
                // Pruebo eliminando uno de los dos posibles niveles que causaron el problema
                let new_report1 = remove_level(&report, unsafe_level);
                let new_report2 = remove_level(&report, unsafe_level + 1);
                let new_safety1 = check_report(&new_report1);
                let new_safety2 = check_report(&new_report2);

                if new_safety1.is_none() || new_safety2.is_none() {
                    near_safe_reports += 1;
                }
            }
            None => safe_reports += 1,
        }
    }

    println!("Safe reports: {}", safe_reports);
    println!(
        "Potentially safe reports: {}",
        safe_reports + near_safe_reports
    );
}
