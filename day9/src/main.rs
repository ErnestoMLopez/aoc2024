use std::fs;
use std::io;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Block {
    Free,
    File(i32),
}

impl Block {
    fn is_free(&self) -> bool {
        match self {
            Block::Free => true,
            _ => false,
        }
    }

    fn is_file(&self) -> bool {
        match self {
            Block::File(_) => true,
            _ => false,
        }
    }
}

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

fn get_disk_usage(buffer: &str) -> Vec<Block> {
    let blocks: Vec<Block> = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 0 {
                vec![Block::File((i / 2) as i32); x as usize]
            } else {
                vec![Block::Free; x as usize]
            }
        })
        .flatten()
        .collect();

    blocks
}

fn defragment_disk(disk: &mut Vec<Block>) {
    let length = disk.len();
    let mut i_free: usize = 0;
    let mut i_file: usize = length;

    loop {
        // Busco el primer bloque sin datos y el último bloque con datos
        i_free += 1 + disk
            .iter()
            .skip(i_free + 1)
            .position(|x| x.is_free())
            .unwrap();
        i_file -= 1 + disk
            .iter()
            .rev()
            .skip(length - i_file)
            .position(|x| x.is_file())
            .unwrap();

        if i_free >= i_file {
            break;
        }

        disk.swap(i_free, i_file);
    }
}

fn calculate_checksum(disk: &Vec<Block>) -> u64 {
    disk.iter()
        .enumerate()
        .map(|(i, block)| match block {
            Block::Free => 0,
            Block::File(id) => (i as u64) * (*id as u64),
        })
        .sum()
}
