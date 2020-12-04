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

fn count_trees(lines: &Vec<String>, down_moves: usize, right_moves: usize) -> u64 {
    let num_rows = lines.len();
    let num_cols = lines[0].chars().count();

    // println!("rows {}", num_rows);

    let mut cur_row: usize = 0;
    let mut cur_col: usize = 0;

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

    num_trees
}

fn main() -> Result<(), Error> {
    let lines = read_lines("src/input.txt");
    // println!("{:?}", lines[0]);

    let trees1 = count_trees(&lines, 1,1);
    let trees2 = count_trees(&lines, 1,3);
    let trees3 = count_trees(&lines, 1,5);
    let trees4 = count_trees(&lines, 1,7);
    let trees5 = count_trees(&lines, 2,1);

    println!("Num trees: {}", trees1 * trees2 * trees3 * trees4 * trees5);

    Ok(())
}