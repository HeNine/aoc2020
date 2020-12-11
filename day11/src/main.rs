use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Square::{Floor, Occupied, Empty};

#[derive(Clone, Debug)]
enum Square {
    Floor,
    Occupied { neighbors: Vec<(usize, usize)> },
    Empty { neighbors: Vec<(usize, usize)> },
}

fn main() {
    let file = File::open("input").unwrap();

    let input_buffer = BufReader::new(&file);
    let lines = input_buffer.lines();

    let mut airport: Vec<Vec<Square>> = lines.map(
        |line| line.unwrap().chars().map(
            |char| match char {
                '.' => Floor,
                'L' => Empty { neighbors: Vec::new() },
                '#' => Occupied { neighbors: Vec::new() },
                e => panic!(e)
            }
        ).collect()
    ).collect();

    print_airport(&airport);
    println!();

    for row in 0..airport.len() {
        for col in 0..airport[row].len() {
            airport[row][col] = match airport[row][col] {
                Floor => Floor,
                Occupied { .. } => Occupied { neighbors: get_neighbors(row, col, &airport) },
                Empty { .. } => Empty { neighbors: get_neighbors(row, col, &airport) },
            }
        }
    }

    loop {
        let (new_airport, change) = update(&airport);
        if !change { break; }
        airport = new_airport;
        print_airport(&airport);
        println!();
    }

    let count: i32 = airport.iter().map(
        |row| row.iter().map(|square| match square {
            Occupied { .. } => 1,
            _ => 0
        }).sum::<i32>()
    ).sum();

    println!("{}", count);
}

fn update(airport: &Vec<Vec<Square>>) -> (Vec<Vec<Square>>, bool) {
    let mut new_airport: Vec<Vec<Square>> = airport.clone();

    let width = new_airport[0].len();
    let height = new_airport.len();

    let mut change = false;

    for row in 0..height {
        for col in 0..width {
            match airport[row][col] {
                Floor => continue,
                _ => ()
            }

            let mut filled_count = 0;

            // Part 1
            // for i in [-1 as i32, 0, 1].iter() {
            //     let r =
            //         match airport.get((row as i32 + i) as usize) {
            //             Some(r) => r,
            //             None => continue
            //         };
            //
            //     for j in [-1 as i32, 0, 1].iter() {
            //         if *i == 0 && *j == 0 { continue; }
            //         filled_count +=
            //             match r.get((col as i32 + j) as usize) {
            //                 Some(Occupied { .. }) => 1,
            //                 _ => 0
            //             };
            //     }
            // }

            let neighbors = match &airport[row][col] {
                Occupied { neighbors } => neighbors,
                Empty { neighbors } => neighbors,
                Floor => panic!()
            };

            for (nr, nc) in neighbors {
                filled_count += match airport[*nr][*nc] {
                    Occupied {..} => 1,
                    _ => 0
                }
            }

            if filled_count == 0 {
                match new_airport[row][col] {
                    Empty { .. } => change = true,
                    _ => ()
                }
                new_airport[row][col] = match &airport[row][col] {
                    Occupied { neighbors } => Occupied { neighbors: neighbors.clone() },
                    Empty { neighbors } => Occupied { neighbors: neighbors.clone() },
                    _ => panic!()
                }
            } else if filled_count >= 5 {
                match new_airport[row][col] {
                    Occupied { .. } => change = true,
                    _ => ()
                }
                new_airport[row][col] = match &airport[row][col] {
                    Occupied { neighbors } => Empty { neighbors: neighbors.clone() },
                    Empty { neighbors } => Empty { neighbors: neighbors.clone() },
                    _ => panic!()
                }
            }
        }
    }

    (new_airport, change)
}

fn print_airport(airport: &Vec<Vec<Square>>) {
    for row in airport {
        for col in row {
            match col {
                Occupied { .. } => print!("#"),
                Empty { .. } => print!("L"),
                Floor => print!(".")
            }
        }
        println!()
    }
}

fn get_neighbors(row: usize, col: usize, airport: &Vec<Vec<Square>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let mut i = 1;
    let row_vec = &airport[row];
    loop {
        match row_vec.get(col + i) {
            Some(square) => match square {
                Floor => {
                    i += 1;
                    continue;
                }
                _ => {
                    neighbors.push((row, col + i));
                    break;
                }
            },
            None => break
        }
    }
    i = 1;
    loop {
        if col as i32 - i as i32 >= 0 {
            match row_vec.get(col - i) {
                Some(square) => match square {
                    Floor => {
                        i += 1;
                        continue;
                    }
                    _ => {
                        neighbors.push((row, col - i));
                        break;
                    }
                },
                None => break
            }
        } else { break; }
    }

    neighbors.extend(find_neighbor(1, 0, row, col, airport));
    neighbors.extend(find_neighbor(-1, 0, row, col, airport));
    neighbors.extend(find_neighbor(1, 1, row, col, airport));
    neighbors.extend(find_neighbor(-1, 1, row, col, airport));
    neighbors.extend(find_neighbor(1, -1, row, col, airport));
    neighbors.extend(find_neighbor(-1, -1, row, col, airport));

    neighbors
}

fn find_neighbor(dir_y: i32, dir_x: i32, row: usize, col: usize, airport: &Vec<Vec<Square>>) -> Vec<(usize, usize)> {
    let mut i = 1;
    loop {
        match airport.get((row as i32 + i * dir_y) as usize) {
            Some(row_vec) => match row_vec.get((col as i32 + i * dir_x) as usize) {
                Some(square) => match square {
                    Floor => {
                        i += 1
                    }
                    _ => return [((row as i32 + i * dir_y) as usize, (col as i32 + i * dir_x) as usize)].to_vec()
                }
                None => return Vec::new()
            },
            None => return Vec::new()
        }
    }
}