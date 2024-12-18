pub mod mat;

use mat::Matrix;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

    let (guard, obstacles) = parse_input(&matrix);

    println!("Fin")
}

fn parse_input(grid: &Matrix<char>) -> (Position, Obstacles) {
    let mut guard = Position { x: 0, y: 0 };
    let mut obstacles = Obstacles {
        per_row: HashMap::new(),
        per_col: HashMap::new(),
    };

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            match grid.get(row, col) {
                '^' => guard = Position { x: row, y: col },
                '#' => {
                    if !obstacles.per_row.contains_key(&row) {
                        obstacles.per_row.insert(row, Vec::new());
                    }
                    obstacles.per_row.get_mut(&row).unwrap().push(col);

                    if !obstacles.per_col.contains_key(&col) {
                        obstacles.per_col.insert(col, Vec::new());
                    }
                    obstacles.per_col.get_mut(&col).unwrap().push(row);
                }
                _ => (),
            }
        }
    }

    (guard, obstacles)
}
