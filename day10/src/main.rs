use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);
    let lines = input_buffer.lines();

    let mut adapters: Vec<u32> = lines
        .map(Result::unwrap)
        .map(|x: String| x.trim().to_string())
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    adapters.sort();

    let mut hist = [0, 0, 1];
    let mut prev = 0;
    for adapter in adapters.iter() {
        hist[(adapter - prev - 1) as usize] += 1;
        prev = *adapter;
    }
    println!("{:?}", hist);

    println!("{}", hist[0] * hist[2]);

    adapters.reverse();

    let mut path_counts = Vec::with_capacity(adapters.len());
    path_counts.push(1 as u64);
    for _i in 1..adapters.len() { path_counts.push(0 as u64) };

    for adapter_i in 1..adapters.len() {
        let adapter = adapters[adapter_i];
        path_counts[adapter_i] =
            if adapter_i >= 1 && adapters[adapter_i - 1] - adapter <= 3 { path_counts[adapter_i - 1] } else { 0 } +
            if adapter_i >= 2 && adapters[adapter_i - 2] - adapter <= 3 { path_counts[adapter_i - 2] } else { 0 } +
            if adapter_i >= 3 && adapters[adapter_i - 3] - adapter <= 3 { path_counts[adapter_i - 3] } else { 0 }
    }

    let count =
        if adapters[adapters.len() - 1] <= 3 { path_counts[adapters.len() - 1] } else { 0 } +
            if adapters[adapters.len() - 2] <= 3 { path_counts[adapters.len() - 2] } else { 0 } +
            if adapters[adapters.len() - 3] <= 3 { path_counts[adapters.len() - 3] } else { 0 };

    println!("{}", count);
}
