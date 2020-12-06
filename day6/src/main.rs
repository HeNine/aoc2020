use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::result::Result::{Err, Ok};

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);

    let mut sum = 0;

    let mut seen_chars = HashSet::from_iter((b'a'..=b'z').map(char::from));
    // seen_chars.extend();
    for rline in input_buffer.lines() {
        let line = rline.unwrap();

        if line.trim() == "" {
            sum += seen_chars.len();
            seen_chars = HashSet::from_iter((b'a'..=b'z').map(char::from));
            continue;
        }

        let line_chars: HashSet<char> = HashSet::from_iter(line.chars());
        seen_chars = HashSet::from_iter(seen_chars.intersection(&line_chars).map(|x|*x));
    }

    sum += seen_chars.len();

    println!("{}", sum);
}
