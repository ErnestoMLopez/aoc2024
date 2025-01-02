use std::fs;
use std::io;
use std::io::Read;
use std::time::Instant;

fn main() {
    println!("Advent of Code 2024 - Day 9");

    // Inicio el timer para calcular el tiempo de ejecución
    let now = Instant::now();

    let input_file = "input.txt";

    let buffer = parse_input(input_file);
    let mut disk = get_disk_usage(&buffer);
    defragment_disk(&mut disk);
    let checksum = calculate_checksum(&disk);

    println!("Part 1: Checksum = {:?}", checksum);

    // Imprimo el tiempo de ejecución
    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

fn parse_input(file: &str) -> String {
    let file = fs::File::open(file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    buffer
}

fn get_disk_usage(buffer: &str) -> Vec<i32> {
    let blocks: Vec<i32> = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                vec![(i / 2) as i32; x as usize]
            } else {
                vec![-1 as i32; x as usize]
            }
        })
        .flatten()
        .collect();

    blocks
}

fn defragment_disk(disk: &mut Vec<i32>) {
    loop {
        // Busco el primer bloque sin datos y el último bloque con datos
        let i_free = disk.iter().position(|&x| x == -1).unwrap();
        let i_file = disk.len() - disk.iter().rev().position(|&x| x != -1).unwrap() - 1;

        if i_free >= i_file {
            break;
        }

        disk.swap(i_free, i_file);
    }
}

fn calculate_checksum(disk: &Vec<i32>) -> u64 {
    disk.iter()
        .enumerate()
        .filter(|(_, &x)| x >= 0)
        .map(|(i, &x)| (i as u64) * (x as u64))
        .sum()
}
