use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::result::Result;

fn read_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).expect("could not read line");
    let reader = BufReader::new(f);
    reader.lines()
        .map(|line| line.ok().unwrap().trim().to_string())
        .collect()
}

fn main() -> Result<(), Error> {
    let lines = read_lines("src/input.txt");
    // println!("{:?}", lines[0]);

    let right_moves = 3;
    let down_moves = 1;

    let num_rows = lines.len() as u32;
    let num_cols = lines[0].chars().count();

    // println!("rows {}", num_rows);

    let mut cur_row = 0;
    let mut cur_col = 0;

    let mut num_trees = 0;

    while cur_row < num_rows {
        if lines[cur_row as usize].chars().nth(cur_col).unwrap() == '#' {
            num_trees += 1;
        }

        // println!("{} x {} - line: {} - trees: {}", cur_row, cur_col, lines[cur_row as usize], num_trees);

        cur_row += down_moves;
        cur_col += right_moves;

        if cur_col >= num_cols {
            let leftover_cols = cur_col - num_cols;
            cur_col = 0 + leftover_cols;
        }
    }

    println!("Num trees: {}", num_trees);

    Ok(())
}