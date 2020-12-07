use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("could not read input");
    let lines: Vec<_> = contents.split("\n\n").map(|line| line.replace("\n", " ")).collect::<Vec<_>>();
    // println!("{:?}", lines);

    let mut num_valid = 0;

    for line in lines {
        let mut fields = HashMap::new();
        let pairs: Vec<&str> = line.trim().split(" ").collect();
        for pair in pairs {
            let key_val: Vec<_> = pair.split(":").collect();
            fields.insert(key_val[0], key_val[1]);
        }

        // TODO This smells. There's got to be a better way to check this.
        // Is there a way to pattern match on HashMap key lists? I couldn't find one
        if fields.contains_key("ecl")
            && fields.contains_key("pid")
            && fields.contains_key("eyr")
            && fields.contains_key("hcl")
            && fields.contains_key("byr")
            && fields.contains_key("iyr")
            && fields.contains_key("hgt")
        {
            num_valid += 1;
        }
    }

    println!("# valid: {}", num_valid);

}
