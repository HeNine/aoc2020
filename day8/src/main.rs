use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::{Err, Ok};
use std::marker::Copy;

struct CPUState {
    pc: usize,
    acc: i32,
}

trait Executable {
    fn execute(&self, state: CPUState) -> CPUState;
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    NOP { immediate: i32 },
    JMP { offset: i32 },
    ACC { immediate: i32 },
}

impl Executable for Instruction {
    fn execute(&self, mut state: CPUState) -> CPUState {
        match self {
            Instruction::NOP { immediate: _ } => {
                state.pc += 1;
                state
            }
            Instruction::JMP { offset } => {
                state.pc = (state.pc as i32 + offset) as usize;
                state
            }
            Instruction::ACC { immediate } => {
                state.pc += 1;
                state.acc += immediate;
                state
            }
        }
    }
}

fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);

    let mut instructions: Vec<Instruction> = input_buffer.lines()
        .map(
            |rline| {
                let line = rline.unwrap();
                match &line[0..3] {
                    "nop" => Instruction::NOP { immediate: line[4..line.len()].to_string().parse().unwrap() },
                    "jmp" => Instruction::JMP { offset: line[4..line.len()].to_string().parse().unwrap() },
                    "acc" => Instruction::ACC { immediate: line[4..line.len()].to_string().parse().unwrap() },
                    _e => panic!()
                }
            }
        ).collect();

    let mut executed: Vec<bool> = Vec::with_capacity(instructions.len());
    for _i in 0..instructions.len() { executed.push(false) }

    let mut state = CPUState { acc: 0, pc: 0 };

    'found: for i in 0..instructions.len() {
        instructions[i] = flip(&instructions[i]);

        loop {
            if state.pc >= instructions.len() { break 'found}
            if executed[state.pc] { break; }
            executed[state.pc] = true;
            state = instructions[state.pc].execute(state);
        }

        for j in 0..instructions.len() { executed[j] = false }

        state = CPUState { acc: 0, pc: 0 };

        instructions[i] = flip(&instructions[i]);
    }

    println!("{}", state.acc);
}

fn flip(instruction: &Instruction) -> Instruction {
    match instruction {
        Instruction::NOP { immediate } => Instruction::JMP { offset: *immediate },
        Instruction::JMP { offset } => Instruction::NOP { immediate: *offset },
        instruction => *instruction
    }
}