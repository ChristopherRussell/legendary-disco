use crate::util::get_input_file_reader;
use anyhow::Result;
use std::collections::HashSet;
use std::io::BufRead;

pub fn run() -> Result<i32> {
    // let reader = get_input_file_reader("input3_test")?;
    let reader = get_input_file_reader("input3")?;

    // Stores number seen which have not yet been identified as adjacent to non-period symbol.
    // We only need to store until we have finished processing the next line.
    // Recent symbols stores locations of recent non-period symbols. Again, we only need to store
    // last and current line.

    let mut last_row_numbers: HashSet<(u32, u32, u32)> = HashSet::new();
    let mut last_row_symbols: HashSet<u32> = HashSet::new();
    let mut answer = 0;
    let mut lines = reader.lines().peekable();

    let first_line_length = match lines.peek() {
        Some(Ok(line)) => line.len() as u32,
        _ => 0, // No lines or an error on the first line
    };

    for line in lines {
        let mut num_being_read: u32 = 0;
        let mut nr_digits_current_num: u32 = 0;
        let mut current_row_numbers: HashSet<(u32, u32, u32)> = HashSet::new();
        let mut current_row_symbols: HashSet<u32> = HashSet::new();
        for (line_position, char) in line?.chars().enumerate() {
            let line_position = line_position as u32;
            if char.is_ascii_digit() {
                num_being_read = num_being_read * 10 + char.to_digit(10).unwrap();
                nr_digits_current_num += 1;
                continue;
            }
            maybe_store_complete_number(
                &mut num_being_read,
                &mut current_row_numbers,
                line_position,
                &mut nr_digits_current_num,
                &current_row_symbols,
                &last_row_symbols,
                &mut answer,
            );
            if char == '.' {
                continue;
            }
            process_symbol(
                &mut last_row_numbers,
                line_position,
                &mut answer,
                &mut current_row_numbers,
                &mut current_row_symbols,
            );
        }
        maybe_store_complete_number(
            &mut num_being_read,
            &mut current_row_numbers,
            first_line_length - 1,
            &mut nr_digits_current_num,
            &current_row_symbols,
            &last_row_symbols,
            &mut answer,
        );
        last_row_numbers = current_row_numbers;
        last_row_symbols = current_row_symbols;
    }
    println!("The sum of part numbers is {}", { answer });
    Ok(answer as i32)
}

fn process_symbol(
    last_row_numbers: &mut HashSet<(u32, u32, u32)>,
    line_position: u32,
    answer: &mut u32,
    current_row_numbers: &mut HashSet<(u32, u32, u32)>,
    current_row_symbols: &mut HashSet<u32>,
) {
    // When we see a non-period non-digit symbol, we check whether we saw any numbers in the
    // four locations above and to the left of the symbol location, i.e. the X's below:
    //
    // XXX
    // X@.
    // ...
    //
    search_previous_row_numbers_near_position(last_row_numbers, line_position, answer);
    search_current_row_numbers_before_position(current_row_numbers, line_position, answer);
    current_row_symbols.insert(line_position);
}

fn search_current_row_numbers_before_position(
    current_row_numbers: &mut HashSet<(u32, u32, u32)>,
    line_position: u32,
    answer: &mut u32,
) {
    let mut item_to_remove: (u32, u32, u32) = (0, 0, 0);
    // TODO Optimize: Seems a bit silly to search full row when we know it can only be the last thing we saw.
    for (num_end, nr_digits, num) in current_row_numbers.iter() {
        if *num_end == line_position - 1 {
            item_to_remove = (*num_end, *nr_digits, *num);
            break;
        }
    }
    let num = item_to_remove.2;
    if num > 0 {
        *answer += num;
        current_row_numbers.remove(&item_to_remove);
    }
}

fn search_previous_row_numbers_near_position(
    last_row_numbers: &mut HashSet<(u32, u32, u32)>,
    line_position: u32,
    answer: &mut u32,
) {
    let mut to_remove = Vec::new();

    // Look for numbers overlapping with line_position - 1 .. line_position + 1 in the previous row
    for (num_end, nr_digits, num) in last_row_numbers.iter() {
        let num_start = num_end + 1 - nr_digits;

        // look for num_start - 1 <= line_position <= num_end + 1
        if num_start > line_position + 1 || line_position > *num_end + 1 {
            continue;
        }
        to_remove.push((*num_end, *nr_digits, *num));
    }

    for item in to_remove {
        *answer += item.2;
        last_row_numbers.remove(&item);
    }
}

fn maybe_store_complete_number(
    num_being_read: &mut u32,
    current_row_numbers: &mut HashSet<(u32, u32, u32)>,
    line_position: u32,
    nr_digits_current_num: &mut u32,
    current_row_symbols: &HashSet<u32>,
    last_row_symbols: &HashSet<u32>,
    answer: &mut u32,
) {
    if *num_being_read == 0 {
        return;
    }

    process_or_store_number(
        line_position,
        nr_digits_current_num,
        current_row_symbols,
        answer,
        num_being_read,
        last_row_symbols,
        current_row_numbers,
    );
    *num_being_read = 0;
    *nr_digits_current_num = 0;
}

fn process_or_store_number(
    line_position: u32,
    nr_digits_current_num: &mut u32,
    current_row_symbols: &HashSet<u32>,
    answer: &mut u32,
    num_being_read: &mut u32,
    last_row_symbols: &HashSet<u32>,
    current_row_numbers: &mut HashSet<(u32, u32, u32)>,
) {
    // check previous symbol in current line
    let position_before_number = line_position.saturating_sub(*nr_digits_current_num + 1);
    if current_row_symbols.contains(&position_before_number) {
        *answer += *num_being_read;
        return;
    }
    let num_start = line_position - *nr_digits_current_num;
    let num_end = line_position - 1;
    // check for non-period symbols in previous line that are above the current number
    for position in last_row_symbols {
        // num_start - 1 <= position <= num_end + 1
        if num_start < *position + 2 && *position < num_end + 2 {
            *answer += *num_being_read;
            return;
        }
    }

    // no adjacent non-period symbols yet - store incase we find one later
    current_row_numbers.insert((num_end, *nr_digits_current_num, *num_being_read));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let result = run();
        assert_eq!(result.unwrap(), 556057);
    }
}

// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
// "Aaah!"
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
// Here is an example engine schematic:
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
