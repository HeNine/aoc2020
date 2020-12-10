use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);
    let mut lines = input_buffer.lines();

    let mut buffer = VecDeque::with_capacity(25);
    let mut list = Vec::new();

    for _i in 0..25 {
        let input = lines.next().unwrap().unwrap().parse::<u32>().unwrap();
        buffer.push_back(input);
        list.push(input);
    }

    let mut code = 0;

    for line in lines {
        let input = line.unwrap().parse::<u32>().unwrap();

        if !is_sum(&buffer, input) {
            println!("Code: {}", input);
            code = input;
            break;
        }

        buffer.push_back(input);
        buffer.pop_front();
        list.push(input);
    }

    let mut start = 0;
    let mut end = 1;
    let mut sum = list[start] + list[end];

    loop {
        if sum == code {
            println!("Sum: {}",
                   list[start..end + 1].iter().fold(0, |x,y| max(x,*y)) +
                       list[start..end + 1].iter().fold(list[start], |x,y|min(x,*y)));
            break;
        } else if sum > code {
            sum -= list[start];
            start += 1;
        } else {
            end += 1;
            sum += list[end];
        }
    }
}

fn is_sum(buffer: &VecDeque<u32>, input: u32) -> bool {
    for i in 0..buffer.len() - 1 {
        for j in 1..buffer.len() {
            if buffer[i] + buffer[j] == input {
                return true;
            }
        }
    }

    false
}