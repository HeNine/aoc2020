use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);


    let mut count: [u32; 5] = [0, 0, 0, 0, 0];

    let mut line_n = 0;

    for line_result in input_buffer.lines() {
        let line_string = line_result.unwrap();
        let line = line_string.as_bytes();

        if line[(1 * line_n) % line.len()] == '#' as u8 {
            count[0] += 1;
        }

        if line[(3 * line_n) % line.len()] == '#' as u8 {
            count[1] += 1;
        }

        if line[(5 * line_n) % line.len()] == '#' as u8 {
            count[2] += 1;
        }

        if line[(7 * line_n) % line.len()] == '#' as u8 {
            count[3] += 1;
        }

        if line_n % 2 == 0 && line[(1 * line_n/2) % line.len()] == '#' as u8 {
            count[4] += 1;
        }

        line_n += 1;
    }

    println!("{}", count[0] * count[1] * count[2] * count[3] * count[4]);
}
