#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;


fn main() {
    let file = match File::open("input") {
        Ok(file) => file,
        Err(e) => panic!(e)
    };

    let input_buffer = BufReader::new(&file);

    let mut count: u32 = 0;

    let mut passport = HashMap::new();

    for rline in input_buffer.lines() {
        let line = rline.unwrap();
        let trimmed_line = line.trim();

        if trimmed_line.len() == 0 {
            count += verify(&passport);

//            println!("{}", verify(&passport));

            passport = HashMap::new();

            continue;
        }

        let pairs = trimmed_line.split(" ");

        for pair in pairs {
            print!("{},", pair);
            let (key, value) = pair.split_at(4);
            passport.insert(key[0..3].to_string(), value.to_string());
        }
        println!("");
    }

    count += verify(&passport);

    println!("{}", count);
}


fn verify(passport: &HashMap<String, String>) -> u32 {
    lazy_static! {
        static ref COLOR_MATCH: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
    }
    lazy_static! {
    static ref PID_MATCH: Regex = Regex::new("[0-9]{9}").unwrap();
    }
    lazy_static! {
    static ref EYE_COLORS: HashSet<String> = {
        let mut s = HashSet::new();
        s.insert("amb".to_string());
        s.insert("blu".to_string());
        s.insert("brn".to_string());
        s.insert("gry".to_string());
        s.insert("grn".to_string());
        s.insert("hzl".to_string());
        s.insert("oth".to_string());
        s
        };
    }

    if passport.contains_key("byr") &&
        (1920..2003).contains(&(passport.get("byr").unwrap().parse::<u32>().unwrap())) &&
        passport.contains_key("iyr") &&
        (2010..2021).contains(&(passport.get("iyr").unwrap().parse::<u32>().unwrap())) &&
        passport.contains_key("eyr") &&
        (2020..2031).contains(&(passport.get("eyr").unwrap().parse::<u32>().unwrap())) &&
        passport.contains_key("hgt") &&
        (passport.get("hgt").unwrap().ends_with("cm") &&
            (150..194).contains(&passport.get("hgt").unwrap().trim_end_matches("cm").parse::<u32>().unwrap())
            ||
            passport.get("hgt").unwrap().ends_with("in") &&
                (59..76).contains(&passport.get("hgt").unwrap().trim_end_matches("in").parse::<u32>().unwrap())
        ) &&
        passport.contains_key("ecl") && EYE_COLORS.contains(passport.get("ecl").unwrap()) &&
        passport.contains_key("hcl") && COLOR_MATCH.is_match(passport.get("hcl").unwrap()) &&
        passport.contains_key("pid") && PID_MATCH.is_match(passport.get("pid").unwrap()) {
        1
    } else {
        0
    }
}