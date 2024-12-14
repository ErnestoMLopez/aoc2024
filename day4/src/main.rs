pub mod mat;

use mat::Matrix;
use std::fs;
use std::io;
use std::io::Read;

fn main() {
    println!("Advent of Code 2024 - Day 4");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let matrix = Matrix::from_string(&buffer);

    let mut xmas_count = 0;

    matrix.iter_rows().for_each(|row| {
        row.iter()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
        row.iter()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
    });

    matrix.iter_cols().for_each(|col| {
        col.iter()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
        col.iter()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
    });

    matrix.iter_dw_diag().for_each(|dw_diag| {
        dw_diag
            .iter()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
        dw_diag
            .iter()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
    });

    matrix.iter_uw_diag().for_each(|uw_diag| {
        uw_diag
            .iter()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
        uw_diag
            .iter()
            .rev()
            .collect::<String>()
            .matches("XMAS")
            .for_each(|_| xmas_count += 1);
    });

    println!("XMAS count: {}", xmas_count);
}
