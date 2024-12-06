use std::fs;
use std::io::{self, BufRead};

fn main() {

    println!("Advent of Code 2024 - Day 1");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");

        let numbers: (i32, i32) = {
            let mut iter = line.split_whitespace().map(|s| s.parse().expect("Error parsing number"));
            (iter.next().unwrap(), iter.next().unwrap())
        };

        column1.push(numbers.0);
        column2.push(numbers.1);
    }

    column1.sort();
    column2.sort();

    let mut distance = 0;
    
    for i in 0..column1.len() {
        if column1[i] < column2[i] {
            distance += column2[i] - column1[i];
        } else {
            distance += column1[i] - column2[i];
        }            
    }
    
    let similarity : i32 = column1.iter().map(|&x| x * column2.iter().filter(|&&y| y == x).count() as i32).sum();
    
    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);

}