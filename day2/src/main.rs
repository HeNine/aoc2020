use std::fs::File;
use std::io::{BufReader, BufRead};

#[macro_use]
extern crate scan_fmt;

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);


    let mut count = 0;

    for line_result in input_buffer.lines() {
        let line = line_result.unwrap();
        if match scan_fmt!(&line, "{d}-{d} {}: {}", i32,i32,char, String) {
            Ok((min_occ, max_occ, letter, password)) => is_correct2(min_occ as usize, max_occ as usize, letter, &password),
            Err(e) => panic!(e)
        } {
            count += 1;
        }
    }

    println!("{}", count);
}

fn is_correct(min_occ: i32, max_occ: i32, letter: char, password: &String) -> bool {
    let mut count = 0;

    for c in password.chars() {
        if c == letter {
            count += 1;
        }
    }


    count >= min_occ && count <= max_occ
}

fn is_correct2(p1: usize, p2: usize, letter: char, password: &String) -> bool {
    (password.as_bytes()[p1-1] == letter as u8) ^ (password.as_bytes()[p2-1] == letter as u8)
}