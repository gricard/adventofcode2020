use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

// structure to contain the parsed password entry
struct PWEntry {
    first: u8,
    second: u8,
    letter: String,
    password: String,
}

// implementation of the from_str function to parse a string into the struct
impl FromStr for PWEntry {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // separate the major parts of the line (first/second, character, full password)
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        // parse the first/second values
        let positions: Vec<&str> = parts[0].split('-').collect();
        let first = u8::from_str_radix(positions[0], 10)?;
        let second = u8::from_str_radix(positions[1], 10)?;

        // split the : off the end of the character
        let letter: Vec<&str> = parts[1].split(':').collect();
        let letter = letter[0].clone().to_string();

        let password = parts[2].clone().to_string();

        // return the parsed entry
        Ok(PWEntry { first, second, letter, password })
    }
}

fn is_password_valid(first: u8, second: u8, letter: &String, password: &String) -> bool {
    let letter = letter.chars().nth(0).unwrap();
    let pos1 = password.chars().nth((first - 1).into()).unwrap();
    let pos2 = password.chars().nth((second - 1).into()).unwrap();

    (pos1 == letter && pos2 != letter) || (pos1 != letter && pos2 == letter)
}

fn main() {
    let file = File::open("./src/input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut num_valid = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        match PWEntry::from_str(&line) {
            Ok(entry) => {
                let is_valid = if is_password_valid(entry.first, entry.second, &entry.letter, &entry.password) { true } else { false };
                if is_valid {
                    num_valid += 1;
                }
                // println!("first: {} second: {} letter: {} pw: {} valid: {}", entry.first, entry.second, entry.letter, entry.password, is_valid);
            }
            Err(_) => {
                println!("Invalid line: {}", line);
            }
        }
    }

    println!("# valid passwords: {}", num_valid);
}
