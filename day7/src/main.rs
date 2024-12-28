use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Equation {
    total: i64,
    partial: i64,
    operands: VecDeque<i64>,
}

fn main() {
    println!("Advent of Code 2024 - Day 7");

    // Inicio el timer para calcular el tiempo de ejecución
    let now = Instant::now();

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let mut part1 = 0;
    let mut part2 = 0;

    for line in buffer.lines() {
        let mut equation = get_equation(line);

        if verify_sum_mul_equation(&mut equation) {
            part1 += equation.total;
        }

        if verify_sum_mul_concat_equation(&mut equation) {
            part2 += equation.total;
        }
    }

    println!("Result Part 1: {}", part1);
    println!("Result Part 2: {}", part2);

    // Imprimo el tiempo de ejecución
    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

fn get_equation(line: &str) -> Equation {
    let mut eq_iter = line.split(":");

    let equation = Equation {
        total: eq_iter.next().unwrap().parse::<i64>().unwrap(),
        partial: 0,
        operands: eq_iter
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    };

    equation
}

fn verify_sum_mul_equation(eq: &mut Equation) -> bool {
    if eq.operands.is_empty() {
        if eq.total == eq.partial {
            return true;
        } else {
            return false;
        }
    }

    // Suma
    let mut sub_eq = eq.clone();
    let operand = sub_eq.operands.pop_front().unwrap();
    sub_eq.partial += operand;
    if verify_sum_mul_equation(&mut sub_eq) {
        return true;
    }

    // Multiplicación
    let mut sub_eq = eq.clone();
    let operand = sub_eq.operands.pop_front().unwrap();
    sub_eq.partial *= operand;
    if verify_sum_mul_equation(&mut sub_eq) {
        return true;
    }

    false
}

fn verify_sum_mul_concat_equation(eq: &mut Equation) -> bool {
    if eq.operands.is_empty() {
        if eq.total == eq.partial {
            return true;
        } else {
            return false;
        }
    }

    // Suma
    let mut sub_eq = eq.clone();
    let operand = sub_eq.operands.pop_front().unwrap();
    sub_eq.partial += operand;
    if verify_sum_mul_concat_equation(&mut sub_eq) {
        return true;
    }

    // Multiplicación
    let mut sub_eq = eq.clone();
    let operand = sub_eq.operands.pop_front().unwrap();
    sub_eq.partial *= operand;
    if verify_sum_mul_concat_equation(&mut sub_eq) {
        return true;
    }

    // Concatenación
    let mut sub_eq = eq.clone();
    let operand = sub_eq.operands.pop_front().unwrap();
    sub_eq.partial = concat(sub_eq.partial, operand);
    if verify_sum_mul_concat_equation(&mut sub_eq) {
        return true;
    }

    false
}

fn concat(a: i64, b: i64) -> i64 {
    let mut num = b.abs();
    let mut dec: u32 = 0;

    while num > 0 {
        dec += 1;
        num /= 10;
    }

    a * 10_i64.pow(dec) + b
}
