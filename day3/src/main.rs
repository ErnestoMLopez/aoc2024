use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::usize;

#[derive(PartialEq, Eq, Hash)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
    Invalid,
}

fn search_next_operation(buffer: &mut String) -> Option<(Operation, usize)> {
    let next_mul = buffer.find("mul(");
    let next_do = buffer.find("do()");
    let next_dont = buffer.find("don't()");

    if next_mul.is_none() && next_do.is_none() && next_dont.is_none() {
        return None;
    }

    let op_table = HashMap::from([
        (Operation::Mul(0, 0), next_mul.unwrap_or(usize::MAX)),
        (Operation::Do, next_do.unwrap_or(usize::MAX)),
        (Operation::Dont, next_dont.unwrap_or(usize::MAX)),
    ]);

    let op = op_table.iter().find_map(|(key, &value)| {
        if value == *op_table.values().min().unwrap() {
            Some(key)
        } else {
            None
        }
    });

    match op {
        Some(Operation::Mul(_, _)) => {
            let start = next_mul.unwrap() + 4;
            let limit: usize;
            if buffer.len() < start + 8 {
                limit = buffer.len() - 1;
            } else {
                limit = start + 7;
            }

            let end = buffer[start..=limit].find(")");
            match end {
                Some(j) => {
                    let args = &buffer[start..start + j];
                    let parts: Vec<&str> = args.split(',').collect();
                    if parts.len() == 2 {
                        if let (Ok(num1), Ok(num2)) =
                            (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                        {
                            return Some((Operation::Mul(num1, num2), start + j));
                        }
                    }
                    Some((Operation::Invalid, start))
                }
                None => Some((Operation::Invalid, start)),
            }
        }
        Some(Operation::Do) => Some((Operation::Do, next_do.unwrap() + 3)),
        Some(Operation::Dont) => Some((Operation::Dont, next_dont.unwrap() + 6)),
        _ => None,
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 3");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer_part1 = String::new();
    reader
        .read_to_string(&mut buffer_part1)
        .expect("Error reading file");

    let mut buffer_part2 = buffer_part1.clone();

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    loop {
        let result = search_next_operation(&mut buffer_part1);

        match result {
            Some((Operation::Mul(a, b), end)) => {
                total_part1 += a * b;
                buffer_part1.replace_range(0..=end, "");
            }
            Some((_, end)) => {
                buffer_part1.replace_range(0..=end, "");
            }
            None => {
                break;
            }
        }
    }

    let mut mul_enabled = true;

    loop {
        let result = search_next_operation(&mut buffer_part2);

        match result {
            Some((Operation::Mul(a, b), end)) => {
                if mul_enabled {
                    total_part2 += a * b;
                }
                buffer_part2.replace_range(0..=end, "");
            }
            Some((Operation::Do, end)) => {
                mul_enabled = true;
                buffer_part2.replace_range(0..=end, "");
            }
            Some((Operation::Dont, end)) => {
                mul_enabled = false;
                buffer_part2.replace_range(0..=end, "");
            }
            Some((Operation::Invalid, end)) => {
                buffer_part2.replace_range(0..=end, "");
            }
            None => {
                break;
            }
        }
    }

    println!("Resultado parte 1 = {}", total_part1);
    println!("Resultado parte 2 = {}", total_part2);
}
