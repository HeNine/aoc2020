#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

#[derive(Clone, Debug)]
struct Field {
    name: String,
    range1: Range<i32>,
    range2: Range<i32>,
}

impl Field {
    fn is_valid(&self, val: &i32) -> bool {
        self.range1.contains(val) || self.range2.contains(val)
    }
}

impl FromStr for Field {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();
        }

        let matches = RE.captures(s).unwrap();

        Ok(Field {
            name: matches.get(1).unwrap().as_str().to_string(),
            range1: matches.get(2).unwrap().as_str().parse().unwrap()..matches.get(3).unwrap().as_str().parse::<i32>().unwrap() + 1,
            range2: matches.get(4).unwrap().as_str().parse().unwrap()..matches.get(5).unwrap().as_str().parse::<i32>().unwrap() + 1,
        })
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let file_buffer = BufReader::new(file);
    let mut lines = file_buffer.lines();

    let mut fields: Vec<Field> = Vec::new();

    loop {
        let line = lines.next().unwrap().unwrap();

        if line.trim() == "" { break; }

        fields.push(line.parse().unwrap())
    }

    lines.next(); // Your ticket
    let my_ticket: Vec<i32> = lines.next().unwrap().unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect(); // Ticket numbers
    lines.next(); // \n
    lines.next(); // Nearby tickets

    // let mut definitely_not_error_rate_thats_not_what_that_means = 0;
    //
    // while let Some(Ok(line)) = lines.next() {
    //     definitely_not_error_rate_thats_not_what_that_means += line.split(",")
    //         .map(|n| n.parse::<i32>().unwrap())
    //         .filter(
    //             |num: &i32| {
    //                 !(&fields).iter()
    //                     .any(|field| field.is_valid(&num))
    //             }
    //         )
    //         .sum::<i32>();
    //     // println!("{}", definitely_not_error_rate_thats_not_what_that_means)
    // }

    let mut tickets = Vec::with_capacity(fields.len());
    tickets.resize(fields.len(), Vec::new());

    while let Some(Ok(line)) = lines.next() {
        let ticket: Vec<i32> = line.split(",")
            .map(|n| n.parse::<i32>().unwrap()).collect();

        if !ticket.iter().all(|num: &i32| {
            (&fields).iter()
                .any(|field| field.is_valid(&num))
        }) {
            continue;
        }

        for (i, v) in ticket.iter().enumerate() {
            tickets[i].push(*v);
        }
    }

    let mut valid_field = Vec::new();
    for field in fields.iter() {
        let mut cols = Vec::new();

        for (ci, col) in tickets.iter().enumerate() {
            if col.iter().all(|n: &i32| field.is_valid(n)) {
                cols.push(
                    ci as i32
                )
            }
        }
        valid_field.push(cols)
    }

    println!("{:?}", valid_field);

    println!("{:?}", fields);

    // println!("{:?}", tickets);

    let mut assignment = assign(&mut valid_field, &mut Vec::new()).unwrap();

    assignment.reverse();

    let mut p = 1i64;
    for (i, f) in fields.iter().enumerate(){
        if f.name.starts_with("departure") {
            p *= my_ticket[assignment[i] as usize] as i64;
        }
    }

    println!("{:?}", assignment);

    println!("{}", p);

    // println!("{}", definitely_not_error_rate_thats_not_what_that_means)
}

fn assign(valid_field: &mut Vec<Vec<i32>>, assignment: &mut Vec<i32>) -> Option<Vec<i32>> {
    if valid_field.is_empty() {
        return Some(assignment.clone());
    }

    let valid_for = valid_field.pop().unwrap();

    for f in valid_for.iter() {
        if assignment.contains(&f) {
            continue;
        }
        assignment.push(*f);
        match assign(valid_field, assignment) {
            Some(full_assignment) => return Some(full_assignment),
            None => {
                assignment.pop();
            }
        }
    }

    valid_field.push(valid_for);

    None
}