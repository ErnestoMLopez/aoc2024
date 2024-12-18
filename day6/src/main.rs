pub mod mat;

use mat::Matrix;
use std::fs;
use std::io;
use std::io::Read;

fn main() {
    println!("Advent of Code 2024 - Day 4");

    let input_file = "sample.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let matrix = Matrix::from_string(&buffer);

    println!("Fin")
}
