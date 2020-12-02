use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

// structure to contain the parsed password entry
struct PWEntry {
    min: u8,
    max: u8,
    letter: String,
    password: String,
}

// implementation of the from_str function to parse a string into the struct
impl FromStr for PWEntry {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // separate the major parts of the line (min/max, character, full password)
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        // parse the min/max values
        let min_max: Vec<&str> = parts[0].split('-').collect();
        let min = u8::from_str_radix(min_max[0], 10)?;
        let max = u8::from_str_radix(min_max[1], 10)?;

        // split the : off the end of the character
        let letter: Vec<&str> = parts[1].split(':').collect();
        let letter = letter[0].clone().to_string();

        let password = parts[2].clone().to_string();

        // return the parsed entry
        Ok(PWEntry { min, max, letter, password })
    }
}

fn count_chars(char: &String, word: &String) -> u32 {
    let matches: Vec<&str> = word.matches(char).collect();
    matches.len() as u32
}

fn is_password_valid(min: u8, max: u8, letter: &String, password: &String) -> bool {
    let count = count_chars(letter, password);
    count >= min.into() && count <= max.into()
}

fn main() {
    let file = File::open("./src/input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut num_valid = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        match PWEntry::from_str(&line) {
            Ok(entry) => {
                let is_valid = if is_password_valid(entry.min, entry.max, &entry.letter, &entry.password) { true } else { false };
                if is_valid {
                    num_valid += 1;
                }
                // println!("min: {} max: {} letter: {} pw: {} valid: {}", entry.min, entry.max, entry.letter, entry.password, is_valid);
            }
            Err(e) => {
                println!("Invalid line: {}", line);
            }
        }
    }

    println!("# valid passwords: {}", num_valid);
}
