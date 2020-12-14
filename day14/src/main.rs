use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Mask {
    mask0: u64,
    mask1: u64,
    n_x: u32,
    p_x: Vec<u64>,
}

fn main() {
    let file = File::open("input").unwrap();

    let input_buffer = BufReader::new(&file);
    let lines = input_buffer.lines();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    // let mut mask0 = 0u64;
    // let mut mask1 = 0u64;
    let mut mask = Mask {
        mask0: 0,
        mask1: 0,
        n_x: 0,
        p_x: vec![],
    };

    for rline in lines {
        let line = rline.unwrap();
        let mut spline = line.split(" = ");

        let command = spline.next().unwrap().to_owned();
        // let command_prefix = command[0..4].to_owned();
        let param = spline.next().unwrap();

        match &command[0..4] {
            "mask" => {
                mask = parse_mask(param);
                // mask0 = new_mask0;
                // mask1 = new_mask1;
            }
            "mem[" => memory = update_mem(command, param, &mask, memory),
            e => panic!(e.to_owned())
        }
    }

    let sum: u64 = memory.iter().map(|(_, v)| v).sum();

    println!("{}", sum)
}

fn update_mem(command: String, param: &str, mask: &Mask, mut memory: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let location: u64 = command[4..command.len() - 1].parse().unwrap();
    let parameter: u64 = param.trim().parse().unwrap();

    // memory.insert(location, (parameter | mask1) & !mask0);
    for i in 0..1 << mask.n_x {
        let mut address = location & mask.mask0 | mask.mask1;
        let mut bi = 0;
        for p in mask.p_x.iter() {
            address |= ((i >> bi) & 1) << p;
            bi += 1
        }
        memory.insert(address, parameter);
    }

    memory
}

fn parse_mask(param: &str) -> Mask {
    let mut mask0 = 0;
    let mut mask1 = 0;
    // let mut maskX = 0;
    let mut p_x = Vec::new();
    let mut n_x = 0;

    let mut p:i32 = 35;
    for char in param.chars() {
        match char {
            'X' => {
                mask0 <<= 1;
                mask1 <<= 1;
                p_x.push(p as u64);
                n_x += 1;
                p-=1
            }
            '1' => {
                mask0 <<= 1;
                mask1 <<= 1;
                mask1 += 1;
                p -= 1
            }
            '0' => {
                mask0 <<= 1;
                mask0 += 1;
                mask1 <<= 1;
                p -= 1
            }
            e => panic!(e)
        }
    }

    Mask { mask1, mask0, n_x, p_x }
}
