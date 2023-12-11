use crate::util::get_input_file_reader;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::io::{BufRead, Lines};

pub fn run() -> Result<()> {
    let reader = get_input_file_reader("input4")?;
    let lines = reader.lines();
    // create iterator of lines split by whitespace
    let score = process_lines(lines)?;
    println!("Scratchcard total score: {}", score);
    Ok(())
}

fn process_lines(lines: Lines<BufReader<File>>) -> Result<i32> {
    let mut score = 0;
    for line_result in lines {
        let line = line_result?;
        score += calculate_game_score(line);
    }
    Ok(score)
}

fn calculate_game_score(line: String) -> i32 {
    let parts = line.split_whitespace();
    let mut winning_nums: HashSet<i32> = HashSet::new();
    let mut matched_numbers = 0;
    let mut seen_all_winning_numbers = false;

    for part in parts {
        if let Ok(digit) = part.parse::<i32>() {
            if seen_all_winning_numbers {
                if winning_nums.contains(&digit) {
                    matched_numbers += 1;
                }
            } else {
                winning_nums.insert(digit);
            }
        } else if part == "|" {
            seen_all_winning_numbers = true;
        }
    }
    if matched_numbers == 0 {
        return 0;
    }
    2_i32.pow(matched_numbers - 1)
}
