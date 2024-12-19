pub mod mat;

use mat::Matrix;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug)]
struct Obstacles {
    per_row: HashMap<usize, Vec<usize>>,
    per_col: HashMap<usize, Vec<usize>>,
}

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

    let (guard, obstacles) = generate_map(&matrix);

    println!("Guard: {:?}", guard);
    println!("Obstacles: Row 0: {:?}", obstacles.per_row.get(&0).unwrap());
    println!("Obstacles: Col 0: {:?}", obstacles.per_col.get(&0).unwrap());
}

fn generate_map(grid: &Matrix<char>) -> (Guard, Obstacles) {
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Up,
    };
    let mut obstacles = Obstacles {
        per_row: HashMap::new(),
        per_col: HashMap::new(),
    };

    // Creo las entradas para cada fila y columna. Si no hay obstaculos en una fila/columna el
    // vector estará vacío, pero puedo buscar igual
    for row in 0..grid.rows() {
        obstacles.per_row.insert(row, Vec::new());
    }

    for col in 0..grid.cols() {
        obstacles.per_col.insert(col, Vec::new());
    }

    // Recorro la grilla buscando la posición inicial del guardia y los obstáculos: Esta forma de
    // recorrerla me asegura que los vectores quedan automáticamente ordenados de menor a mayor
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            match grid.get(row, col) {
                '^' => {
                    guard.position.x = row;
                    guard.position.y = col;
                    guard.direction = Direction::Up
                }
                '#' => {
                    obstacles.per_row.get_mut(&row).unwrap().push(col);
                    obstacles.per_col.get_mut(&col).unwrap().push(row);
                }
                _ => (),
            }
        }
    }

    (guard, obstacles)
}
