use std::vec::Vec;
use std::io;

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_n) => {
                println!("{}", line);
                numbers.push(line.trim().parse().unwrap());
                line.truncate(0);
            }
            Err(error) => println!("error: {}", error),
        }
    }

    for i in 0..numbers.len() - 2 {
        for j in i + 1..numbers.len() - 1 {
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    println!("{}", numbers[i] * numbers[j] * numbers[k]);
                };
            }
        }
    }
}
