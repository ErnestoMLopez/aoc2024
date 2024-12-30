pub mod mat;

use mat::Matrix;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct AntennaMap {
    antennas: HashMap<char, Vec<Position>>,
    rows: usize,
    cols: usize,
}

fn main() {
    println!("Advent of Code 2024 - Day 8");

    // Inicio el timer para calcular el tiempo de ejecución
    let now = Instant::now();

    let input_file = "input.txt";
    let file = fs::File::open(input_file).expect("Error opening file");
    let mut reader = io::BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .expect("Error reading file");

    let matrix = Matrix::from_string(&buffer);

    let antennas = get_antennas(&matrix);

    let antinodes = create_antinodes(&antennas);
    let harmonic_antinodes = create_harmonic_antinodes(&antennas);

    println!("Part 1: {:?} antinodes", antinodes.len());
    println!("Part 2: {:?} Harmonic antinodes", harmonic_antinodes.len());

    // Imprimo el tiempo de ejecución
    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);
}

fn get_antennas(matrix: &Matrix<char>) -> AntennaMap {
    let mut antennas = HashMap::new();

    for y in 0..matrix.rows() {
        for x in 0..matrix.cols() {
            let antenna = matrix.get(x, y);
            if *antenna != '.' {
                antennas
                    .entry(*antenna)
                    .or_insert(Vec::new())
                    .push(Position { x, y });
            }
        }
    }

    AntennaMap {
        antennas,
        rows: matrix.rows(),
        cols: matrix.cols(),
    }
}

fn create_antinodes(antenna_map: &AntennaMap) -> HashMap<Position, Vec<char>> {
    let mut antinodes = HashMap::new();

    antenna_map
        .antennas
        .iter()
        .for_each(|(antenna, positions)| {
            positions.iter().enumerate().for_each(|(i, p1)| {
                positions.iter().skip(i + 1).for_each(|p2| {
                    let (a1, a2) = calculate_antinodes(p1, p2, antenna_map);
                    if let Some(a1) = a1 {
                        antinodes.entry(a1).or_insert(Vec::new()).push(*antenna);
                    }
                    if let Some(a2) = a2 {
                        antinodes.entry(a2).or_insert(Vec::new()).push(*antenna);
                    }
                });
            });
        });

    antinodes
}

fn create_harmonic_antinodes(antenna_map: &AntennaMap) -> HashMap<Position, Vec<char>> {
    let mut antinodes = HashMap::new();

    antenna_map
        .antennas
        .iter()
        .for_each(|(antenna, positions)| {
            positions.iter().enumerate().for_each(|(i, p1)| {
                positions.iter().skip(i + 1).for_each(|p2| {
                    let a = calculate_harmonic_antinodes(p1, p2, antenna_map);
                    a.iter().for_each(|antinode| {
                        antinodes
                            .entry(*antinode)
                            .or_insert(Vec::new())
                            .push(*antenna);
                    });
                });
            });
        });

    antinodes
}

fn calculate_antinodes(
    pos1: &Position,
    pos2: &Position,
    antenna_map: &AntennaMap,
) -> (Option<Position>, Option<Position>) {
    let dx = pos2.x as isize - pos1.x as isize;
    let dy = pos2.y as isize - pos1.y as isize;
    let a1_x = pos2.x as isize + dx;
    let a1_y = pos2.y as isize + dy;
    let a2_x = pos1.x as isize - dx;
    let a2_y = pos1.y as isize - dy;

    let mut a1 = None;
    let mut a2 = None;

    if is_valid(a1_x, a1_y, antenna_map) {
        a1 = Some(Position {
            x: a1_x as usize,
            y: a1_y as usize,
        });
    }

    if is_valid(a2_x, a2_y, antenna_map) {
        a2 = Some(Position {
            x: a2_x as usize,
            y: a2_y as usize,
        });
    }

    (a1, a2)
}

fn is_valid(x: isize, y: isize, antenna_map: &AntennaMap) -> bool {
    x < antenna_map.rows as isize && y < antenna_map.cols as isize && x >= 0 && y >= 0
}

fn calculate_harmonic_antinodes(
    pos1: &Position,
    pos2: &Position,
    antenna_map: &AntennaMap,
) -> Vec<Position> {
    let mut antinodes = Vec::new();

    let dx = pos2.x as isize - pos1.x as isize;
    let dy = pos2.y as isize - pos1.y as isize;

    let mut n = 0;

    loop {
        let a1_x = pos2.x as isize + dx * n;
        let a1_y = pos2.y as isize + dy * n;
        let a2_x = pos1.x as isize - dx * n;
        let a2_y = pos1.y as isize - dy * n;

        if !is_valid(a1_x, a1_y, antenna_map) && !is_valid(a2_x, a2_y, antenna_map) {
            break;
        }

        if is_valid(a1_x, a1_y, antenna_map) {
            antinodes.push(Position {
                x: a1_x as usize,
                y: a1_y as usize,
            });
        }

        if is_valid(a2_x, a2_y, antenna_map) {
            antinodes.push(Position {
                x: a2_x as usize,
                y: a2_y as usize,
            });
        }

        n += 1;
    }

    antinodes
}
