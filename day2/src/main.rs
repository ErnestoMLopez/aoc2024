use std::fs;
use std::io::{self, BufRead};

fn main() {
    println!("Advent of Code 2024 - Day 2");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let reader = io::BufReader::new(file);

    let mut safe_report = 0;

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Error parsing number"))
            .collect();

        let diffnumbers: Vec<i32> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

        let positive_steps = diffnumbers.iter().filter(|&&x| x > 0).count();
        let negative_steps = diffnumbers.iter().filter(|&&x| x < 0).count();

        let monotonic = positive_steps == diffnumbers.len() || negative_steps == diffnumbers.len();
        let bounded = diffnumbers.iter().map(|x| x.abs()).max().unwrap() <= 3;

        if monotonic && bounded {
            safe_report += 1;
        }
    }

    println!("Safe reports: {}", safe_report);
}
