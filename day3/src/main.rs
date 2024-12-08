use std::fs;
use std::io;
use std::io::Read;

fn search_next_mul(buffer: &mut String) -> Option<(bool, i32, i32, usize)> {
    match buffer.find("mul(") {
        Some(i) => {
            let start = i + 4;
            let limit: usize;
            if buffer.len() < start + 7 {
                limit = buffer.len();
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
                            return Some((true, num1, num2, start + j));
                        }
                    }
                    Some((false, 0, 0, start + 4))
                }
                None => Some((false, 0, 0, start + 4)),
            }
        }
        None => None,
    }
}

fn main() {
    println!("Advent of Code 2024 - Day 3");

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let mut total = 0;

    loop {
        let result = search_next_mul(&mut buffer);

        match result {
            Some((valid, a, b, end)) => {
                if valid {
                    total += a * b;
                }

                buffer.replace_range(0..end, "");
            }
            None => {
                break;
            }
        }
    }

    println!("Resultado final = {}", total);
}
