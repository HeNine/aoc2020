// use std::collections::HashMap;

fn main() {
    // let input = vec![0, 3, 6];
    let input:Vec<i32> = vec![20, 9, 11, 0, 1, 2];

    // let mut memory: HashMap<i32, usize> = HashMap::new();
    let mut memory: Vec<Option<i32>> = Vec::with_capacity(30_000_000);
    memory.resize(30_000_000, None);
    let mut last = input[0];

    for turn in 1..30_000_000 {
        let new_last = if turn <= input.len() - 1 {
            input[turn]
        } else {
            match memory.get(last as usize) {
                Some(Some(last_turn)) => {
                    (turn as i32 - last_turn) as i32
                }
                Some(None) => {
                    0
                }
                _ => panic!()
            }
        };

        // memory.insert(last, turn);
        memory[last as usize] = Some(turn as i32);

        last = new_last;

        if turn % 1000000 == 0 {
            println!("{}/30000000 \t- {:.2}%", turn, turn as f32 / 30000000f32 * 100f32)
        }

        // println!("Turn: {}\t\t Next Number: {}", turn, last);
        // println!("{:?}", memory)
    }
    println!("Number: {}", last);
}
