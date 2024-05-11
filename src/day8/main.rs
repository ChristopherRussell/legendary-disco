use anyhow::Result;
use core::panic;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
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

struct Input<T> {
    instructions: String,
    left_map: HashMap<T, T>,
    right_map: HashMap<T, T>,
}

fn is_finished_part1(position: &str) -> bool {
    position == "ZZZ"
}

fn is_finished_part2(position: &str) -> bool {
    position.ends_with('Z')
}

pub fn part1(input: &[u8]) -> Result<i64> {
    let Input {
        instructions,
        left_map,
        right_map,
    } = parse_input(input)?;
    let start_position = "AAA";

    get_number_of_steps_to_target(
        &instructions,
        start_position,
        is_finished_part1,
        &left_map,
        &right_map,
    )
}
pub fn part2(input: &[u8]) -> Result<i64> {
    let Input {
        instructions,
        left_map,
        right_map,
    } = parse_input(input)?;
    let start_positions = left_map.keys().filter(|k| k.ends_with('A'));
    let mut steps_taken = Vec::new();

    for start_position in start_positions {
        println!("Starting at {}", start_position);
        steps_taken.push(get_number_of_steps_to_target(
            &instructions,
            start_position,
            is_finished_part2,
            &left_map,
            &right_map,
        )?)
    }
    let lcm_steps_taken = steps_taken.iter().fold(1i64, |acc, x| lcm(acc, *x));
    println!("LCM of steps taken: {}", lcm_steps_taken);
    Ok(lcm_steps_taken)
}

fn get_number_of_steps_to_target(
    instructions: &str,
    start_position: &str,
    is_finished: fn(&str) -> bool,
    left_map: &HashMap<String, String>,
    right_map: &HashMap<String, String>,
) -> Result<i64> {
    let mut current_position = start_position;
    let mut positions_seen_at_first_instruction = vec![];
    for (steps_taken, (instruction_nr, instruction)) in instructions
        .as_bytes()
        .iter()
        .enumerate()
        .cycle()
        .enumerate()
    {
        if instruction_nr == 0 {
            if positions_seen_at_first_instruction.contains(&current_position) {
                panic!("Stuck in a loop!")
            }
            positions_seen_at_first_instruction.push(current_position)
        }

        if is_finished(current_position) {
            println!("Reached target, steps taken: {}", steps_taken);
            return Ok(steps_taken as i64);
        }

        match instruction {
            b'L' => current_position = left_map.get(current_position).unwrap(),
            b'R' => current_position = right_map.get(current_position).unwrap(),
            _ => panic!("unexpected instruction {:?}", instruction),
        }
    }
    unreachable!()
}

fn parse_input(input: &[u8]) -> Result<Input<String>> {
    // LHLHLLLHLHHLLHLL
    //
    // ABC = (DEF, XYZ)
    // ABC = (DEF, XYZ)
    // ...
    let mut lines = input.lines();
    let instructions = lines.next().expect("First line for instructions")?;
    assert_eq!(
        lines
            .next()
            .expect("Second line should exist")
            .unwrap()
            .len(),
        0
    );

    let node_name_capture: Regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();
    let mut left_map = HashMap::new();
    let mut right_map = HashMap::new();

    lines.for_each(|line| {
        let line = line.unwrap().to_owned();
        if let Some(caps) = node_name_capture.captures(&line) {
            let name = caps.get(1).unwrap().as_str().to_owned();
            let left = caps.get(2).unwrap().as_str().to_owned();
            let right = caps.get(3).unwrap().as_str().to_owned();

            if left_map.insert(name.clone(), left).is_some() {
                panic!("Unexpected overwrite of key")
            };
            if right_map.insert(name, right).is_some() {
                panic!("Unexpected overwrite of key")
            };
        } else {
            panic!("Capture failed for line: {}", line)
        }
    });
    Ok(Input {
        instructions,
        left_map,
        right_map,
    })
}
