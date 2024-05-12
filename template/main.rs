use anyhow::Result;
use core::panic;
use itertools::Itertools;
use std::io::BufRead;

const INPUT: &[u8] = include_bytes!("input.txt");

fn main() {
    if let Err(e) = part1(INPUT) {
        eprintln!("Part 1 Error: {}", e);
    }
    if let Err(e) = part2(INPUT) {
        eprintln!("Part 2 Error: {}", e);
    }
}

fn part1(input: &[u8]) -> Result<i64> {
    todo!()
}

fn part2(input: &[u8]) -> Result<i64> {
    todo!()
}

fn parse_input(input: &[u8]) -> Vec<Vec<i64>> {
    todo!()
}
