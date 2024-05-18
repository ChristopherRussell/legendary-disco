use crate::custom_error::AocError;
use std::collections::HashSet;
use std::str;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<i64, AocError> {
    let lines = input.lines().map(|x| x.as_bytes());
    // create iterator of lines split by whitespace
    let mut score = 0;
    let mut matches = Vec::new();
    for line_result in lines {
        let line = line_result;
        let game_matches = calculate_game_nr_matches(line);
        let game_score = get_score_from_number_of_matches(
            game_matches as u32,
        );
        score += game_score;
        matches.push(game_matches);
    }
    Ok(score as i64)
}
fn calculate_game_nr_matches(line: &[u8]) -> i32 {
    let parts =
        str::from_utf8(line).unwrap().split_whitespace();
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
    matched_numbers
}

fn get_score_from_number_of_matches(
    matched_numbers: u32,
) -> i32 {
    if matched_numbers == 0 {
        return 0;
    }
    2_i32.pow(matched_numbers - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input)?);
        Ok(())
    }
}
