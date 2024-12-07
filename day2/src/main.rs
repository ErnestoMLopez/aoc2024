use std::fs;
use std::io::{self, BufRead};

fn check_report(numbers: &Vec<i32>) -> Option<usize> {
    let diffnumbers: Vec<i32> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

    let positive_steps = diffnumbers.iter().filter(|&&x| x > 0).count();
    let negative_steps = diffnumbers.iter().filter(|&&x| x < 0).count();

    let monotonic = positive_steps == diffnumbers.len() || negative_steps == diffnumbers.len();
    let bounded = diffnumbers.iter().map(|x| x.abs()).max().unwrap() <= 3;

    if monotonic && bounded {
        None
    } else {
        Some(0 as usize) // El índice del nivel conflictivo no se implementa aún
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 2");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Error parsing number"))
            .collect();

        let safety = check_report(&numbers);

        match safety {
            Some(_unsafe_level) => {
                println!("Unsafe report: {:?}", numbers);
            }
            None => safe_reports += 1,
        }
    }

    println!("Safe reports: {}", safe_reports);
}
