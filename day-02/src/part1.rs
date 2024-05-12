use crate::custom_error::AocError;
use regex::Regex;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<i64, AocError> {
    run(input, 12, 13, 14)
}

pub fn run(
    input: &str,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
) -> Result<i64, AocError> {
    let re_game = Regex::new(r"[,|:|;]").unwrap();
    let re_game_number =
        Regex::new(r"^\s*Game\s+(\d+)\s*$").unwrap();
    let re_number_and_color =
        Regex::new(r"^\s*(\d+)\s+(\w+)\s*$").unwrap();
    let mut sum_of_valid_game_number = 0;

    for line in input.lines() {
        let mut max_color_counts: HashMap<String, usize> =
            HashMap::new();
        let mut splits = re_game.split(line);
        let game_number_part = splits.next().unwrap();

        for part in splits {
            let caps =
                re_number_and_color.captures(part).unwrap();
            let number: usize = caps
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let color: String =
                caps.get(2).unwrap().as_str().to_string();

            // Unless we have seen a larger number already,
            // we insert this color, number pair
            if let Some(max_number_seen) =
                max_color_counts.get(&color)
            {
                if max_number_seen >= &number {
                    continue;
                }
            }
            max_color_counts.insert(color, number);
        }

        // game is possible if red <= 12, green <=13, blue
        // <=14
        let game_valid: bool = max_color_counts
            .get("red")
            .unwrap_or(&0)
            <= &max_red
            && max_color_counts.get("green").unwrap_or(&0)
                <= &max_green
            && max_color_counts.get("blue").unwrap_or(&0)
                <= &max_blue;
        if game_valid {
            // replace true with determination of whether
            // game was possible
            let game_number: usize = re_game_number
                .captures(game_number_part)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            sum_of_valid_game_number += game_number;
        }
    }
    Ok(sum_of_valid_game_number as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
