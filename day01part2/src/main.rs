use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use std::result::Result;

fn read_lines<R: Read>(f: R) -> Vec<i64> {
    let reader = BufReader::new(f);
    let vec: Vec<i64> = reader.lines()
        .map(|line| line.ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0))
        .collect();
    vec
}

fn main() -> Result<(), Error> {
    let f = File::open("src/input.txt")?;
    let numbers = read_lines(f);
    // println!("{:?}", numbers);

    let mut first_num = 0;
    let mut second_num = 0;
    let mut third_num = 0;
    let mut result = 0;

    let num_count: u64 = numbers.len() as u64 - 1;

    for n1 in 0..=num_count {
        let num1 = numbers[n1 as usize];
        // println!("{}", num1);
        for n2 in n1 + 1..=num_count {
            let num2 = numbers[n2 as usize];
            for n3 in n1 + 1..=num_count {
                let num3 = numbers[n3 as usize];
                // println!("{} + {} = {}", num1, num2, num1 + num2);
                if num1 + num2 + num3 == 2020 {
                    first_num = num1;
                    second_num = num2;
                    third_num = num3;
                    result = first_num * second_num * third_num;
                }
            }
        }
    }

    println!("{} x {} x {} = {}", first_num, second_num, third_num, result);

    Ok(())
}