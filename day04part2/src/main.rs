use std::fs;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[macro_use] extern crate lazy_static;

fn is_valid_year(year: &str, min: u32, max: u32) -> bool {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^\d{4}$").unwrap();
    }

    let re_match = re.is_match(year);
    let year: u32 = year.parse().ok().unwrap();

    re_match && year >= min && year <= max
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("could not read input");
    let lines: Vec<_> = contents.split("\n\n").map(|line| line.replace("\n", " ")).collect::<Vec<_>>();
    // println!("{:?}", lines);

    let mut num_valid = 0;

    // let four_digits_re = Regex::new(r"^\d{4}$").unwrap();
    let height_re = Regex::new(r"^(\d*)(cm|in)$").unwrap();
    let hair_color_re = Regex::new(r"^#[a-f,0-9]*$").unwrap();
    let eye_color_re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let nine_digits_re = Regex::new(r"^\d{9}$").unwrap();

    for line in lines {
        let mut fields = HashMap::new();
        let pairs: Vec<&str> = line.trim().split(" ").collect();
        for pair in pairs {
            let key_val: Vec<_> = pair.split(":").collect();
            fields.insert(key_val[0], key_val[1]);
        }

        // TODO This smells. There's got to be a better way to check this.
        // Is there a way to pattern match on HashMap key lists? I couldn't find one
        if !fields.contains_key("ecl")
            || !fields.contains_key("pid")
            || !fields.contains_key("eyr")
            || !fields.contains_key("hcl")
            || !fields.contains_key("byr")
            || !fields.contains_key("iyr")
            || !fields.contains_key("hgt")
        {
            // println!("does not have all fields");
            continue;
        }

        // birth year must be four digits between 1920 and 2002
        let byr = fields.get("byr").unwrap();
        if !is_valid_year(byr, 1920, 2002) {
            // println!("byr is not valid: {}", byr);
            continue;
        }

        // issue year must be between 2010 and 2020
        let iyr = fields.get("iyr").unwrap();
        if !is_valid_year(iyr, 2010, 2020) {
            // println!("iyr is not valid: {}", iyr);
            continue;
        }

        // exp year must be between 2010 and 2020
        let eyr = fields.get("eyr").unwrap();
        if !is_valid_year(eyr, 2020, 2030) {
            // println!("eyr is not valid: {}", eyr);
            continue;
        }

        // height must be 150-193cm or 59-76in
        let hgt = fields.get("hgt").unwrap();
        // validate format
        if !height_re.is_match(hgt) {
            // println!("hgt is not valid: {}", hgt);
            continue;
        }
        // pull out parts and check specific values
        let parts = height_re.captures(hgt).unwrap();
        let digits: u32 = parts.get(1).unwrap().as_str().parse().ok().unwrap();
        let units = parts.get(2).unwrap().as_str();

        if (units == "cm" && (digits < 150 || digits > 193)) || (units == "in" && (digits < 59 || digits > 76)) {
            // println!("hgt invalid: {}", hgt);
            continue;
        }

        // hair color must match format
        let hcl = fields.get("hcl").unwrap();
        if !hair_color_re.is_match(hcl) {
            // println!("hcl invalid {}", hcl);
            continue;
        }

        // eye color must be specific values
        let ecl = fields.get("ecl").unwrap();
        if !eye_color_re.is_match(ecl) {
            // println!("ecl invalid {}", ecl);
            continue;
        }

        // passport id must be 9 digits
        let pid = fields.get("pid").unwrap();
        if !nine_digits_re.is_match(pid) {
            // println!("pid invalid {}", pid);
            continue;
        }

        num_valid += 1;
    }

    println!("# valid: {}", num_valid);

}
