pub mod mat;

use mat::Matrix;
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

impl Guard {
    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Obstacles {
    per_row: HashMap<usize, Vec<usize>>,
    per_col: HashMap<usize, Vec<usize>>,
    rows: usize,
    cols: usize,
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

    let (mut guard, obstacles) = generate_map(&matrix);

    while move_guard(&mut guard, &obstacles) {
        println!(
            "Guard (X = {:?}, Y = {:?})",
            guard.position.x, guard.position.y
        );
    }
}

fn generate_map(grid: &Matrix<char>) -> (Guard, Obstacles) {
    let mut guard = Guard {
        position: Position { x: 0, y: 0 },
        direction: Direction::Up,
    };
    let mut obstacles = Obstacles {
        per_row: HashMap::new(),
        per_col: HashMap::new(),
        rows: grid.rows(),
        cols: grid.cols(),
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

/// Mueve el guardia en la dirección en la que está mirando hasta el último espacio previo a un
/// obstáculo y acto seguido rota el guardia. En caso de no encontrarse ningún obstáculo en la
/// fila/columna, se detiene en el borde de la grilla. Retorna true si se encontró un obstáculo y la
/// patrulla sigue, false si se finalizó la patrulla y el guardia salió de la grilla.
fn move_guard(guard: &mut Guard, obstacles: &Obstacles) -> bool {
    let (row, col) = (guard.position.x, guard.position.y);

    match guard.direction {
        Direction::Up => {
            match obstacles
                .per_col
                .get(&col)
                .unwrap()
                .iter()
                .rev()
                .find(|obs_row| **obs_row < row)
            {
                Some(obs_row) => {
                    guard.position.x = *obs_row + 1;
                    guard.rotate();
                    return true;
                }
                None => {
                    guard.position.x = 0;
                    return false;
                }
            }
        }
        Direction::Down => {
            match obstacles
                .per_col
                .get(&col)
                .unwrap()
                .iter()
                .find(|obs_row| **obs_row > row)
            {
                Some(obs_row) => {
                    guard.position.x = *obs_row - 1;
                    guard.rotate();
                    return true;
                }
                None => {
                    guard.position.x = obstacles.rows - 1;
                    return false;
                }
            }
        }
        Direction::Left => {
            match obstacles
                .per_row
                .get(&row)
                .unwrap()
                .iter()
                .rev()
                .find(|obs_col| **obs_col < col)
            {
                Some(obs_col) => {
                    guard.position.y = *obs_col + 1;
                    guard.rotate();
                    return true;
                }
                None => {
                    guard.position.y = 0;
                    return false;
                }
            }
        }
        Direction::Right => {
            match obstacles
                .per_row
                .get(&row)
                .unwrap()
                .iter()
                .find(|obs_col| **obs_col > col)
            {
                Some(obs_col) => {
                    guard.position.y = *obs_col - 1;
                    guard.rotate();
                    return true;
                }
                None => {
                    guard.position.y = obstacles.cols - 1;
                    return false;
                }
            }
        }
    }
}
