use std::fs::File;
use std::io::{BufReader, BufRead};
use std::result::Result::{Err, Ok};
use std::cmp::{max, min};
use std::collections::HashSet;


fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);

    let mut row = 0;
    let mut column = 0;

    let mut max_id = 0;

    let mut filled_seats = HashSet::new();
    let mut minrow = 200;
    let mut maxrow = -1;

    for rline in input_buffer.lines() {
        let line = match rline {
            Ok(line) => line,
            Err(e) => panic!(e)
        };

        for char in line.chars() {
            match char {
                'F' => {
                    // row_i -= 1;
                    row <<= 1;
                }
                'B' => {
                    row <<= 1;
                    row += 1;
                }
                'R' => {
                    column <<= 1;
                    column += 1;
                }
                'L' => {
                    column <<= 1;
                }
                _ => panic!()
            }
        }

        let id = 8 * &row + &column;

        max_id = max(id, max_id);
        minrow = min(row, minrow);
        maxrow = max(row, maxrow);

        println!("{}: row {} column {} id {}", line, row, column, id);
        filled_seats.insert((row, column));

        row = 0;
        column = 0;
    }

    for r in minrow..maxrow {
        for c in 0..8 {
            if !filled_seats.contains(&(r, c)) {
                println!("My id: {}", 8 * r + c);
            }
        }
    }

    println!("Max id: {}", max_id);
}
