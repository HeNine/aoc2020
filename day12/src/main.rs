use std::fs::File;
use std::io::{BufRead, BufReader};
// use crate::Direction::{North, East, West, South};
use crate::Command::{N, R, L, W, S, E, F};

// #[derive(Copy, Clone, Debug)]
// enum Direction {
//     East(i32),
//     North(i32),
//     West(i32),
//     South(i32),
// }

// impl Direction {
//     fn turn(&self, degrees: i32) -> Direction {
//         match self {
//             East(d) => Direction::new((d + degrees + 360) % 360),
//             North(d) => Direction::new((d + degrees + 360) % 360),
//             West(d) => Direction::new((d + degrees + 360) % 360),
//             South(d) => Direction::new((d + degrees + 360) % 360),
//         }
//     }
//
//     fn new(degrees: i32) -> Direction {
//         match degrees {
//             0 => East(0),
//             90 => North(90),
//             180 => West(180),
//             270 => South(270),
//             n => panic!(n)
//         }
//     }
// }

#[derive(Copy, Clone, Debug)]
struct Ship {
    // direction: Direction,
    v: Velocity,
    x: i32,
    y: i32,
}

impl Ship {
    fn new() -> Self {
        Self {
            // direction: Direction::new(0),
            v: Velocity::new(),
            x: 0,
            y: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

impl Velocity {
    fn new() -> Self {
        Velocity { x: 10, y: 1 }
    }

    fn rotate(self, angle: i32) -> Velocity {
        match angle {
            0 => self,
            90 => Velocity { x: -self.y, y: self.x },
            180 => Velocity { x: -self.x, y: -self.y },
            270 => Velocity { x: self.y, y: -self.x },
            // -0 => self,
            -90 => Velocity { x: self.y, y: -self.x },
            -180 => Velocity { x: -self.x, y: -self.y },
            -270 => Velocity { x: -self.y, y: self.x },
            n => panic!(n)
        }
    }
}

enum Command {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Command {
    fn parse(command: String) -> Self {
        match &command[0..1] {
            "N" => N(command[1..command.len()].parse::<i32>().unwrap()),
            "S" => S(command[1..command.len()].parse::<i32>().unwrap()),
            "E" => E(command[1..command.len()].parse::<i32>().unwrap()),
            "W" => W(command[1..command.len()].parse::<i32>().unwrap()),
            "L" => L(command[1..command.len()].parse::<i32>().unwrap()),
            "R" => R(command[1..command.len()].parse::<i32>().unwrap()),
            "F" => F(command[1..command.len()].parse::<i32>().unwrap()),
            e => panic!(e.to_string())
        }
    }

    fn execute(&self, ship: Ship) -> Ship {
        match self {
            N(dist) => Ship { v: Velocity { x: ship.v.x, y: ship.v.y + dist }, x: ship.x, y: ship.y },
            S(dist) => Ship { v: Velocity { x: ship.v.x, y: ship.v.y - dist }, x: ship.x, y: ship.y },
            E(dist) => Ship { v: Velocity { x: ship.v.x + dist, y: ship.v.y }, x: ship.x, y: ship.y },
            W(dist) => Ship { v: Velocity { x: ship.v.x - dist, y: ship.v.y }, x: ship.x, y: ship.y },
            L(angle) => Ship { v: ship.v.rotate(*angle), x: ship.x, y: ship.y },
            R(angle) => Ship { v: ship.v.rotate(-*angle), x: ship.x, y: ship.y },
            F(dist) => Ship { v: ship.v, x: ship.x + dist * ship.v.x, y: ship.y + dist * ship.v.y }
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();

    let input_buffer = BufReader::new(&file);
    let lines = input_buffer.lines();

    let final_position: Ship = lines
        .map(Result::unwrap)
        .map(|x: String| x.trim().to_string())
        .map(Command::parse)
        .fold(Ship::new(), |ship, command| {
            println!("{:?}", ship);
            command.execute(ship)
        });
    println!("{:?}", final_position);


    println!("{}", final_position.x.abs() + final_position.y.abs())
}
