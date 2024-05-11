use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;
use std::usize;

const INPUT: &[u8] = include_bytes!("input.txt");

pub fn run(max_red: usize, max_green: usize, max_blue: usize) -> Result<i64> {
    let reader = INPUT;

    let re_game = Regex::new(r"[,|:|;]").unwrap();
    let re_game_number = Regex::new(r"^\s*Game\s+(\d+)\s*$").unwrap();
    let re_number_and_color = Regex::new(r"^\s*(\d+)\s+(\w+)\s*$").unwrap();
    let mut sum_of_valid_game_number = 0;
    let mut sum_of_powers: usize = 0; // part 2 solution

    for line in reader.lines() {
        let mut max_color_counts: HashMap<String, usize> = HashMap::new();
        let line = line?;
        let mut splits = re_game.split(&line);
        let game_number_part = splits.next().unwrap();

        for part in splits {
            let caps = re_number_and_color.captures(part).unwrap();
            let number: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let color: String = caps.get(2).unwrap().as_str().to_string();

            // Unless we have seen a larger number already, we insert this color, number pair
            if let Some(max_number_seen) = max_color_counts.get(&color) {
                if max_number_seen >= &number {
                    continue;
                }
            }
            max_color_counts.insert(color, number);
        }
        sum_of_powers += max_color_counts.values().product::<usize>();

        // game is possible if red <= 12, green <=13, blue <=14
        let game_valid: bool = max_color_counts.get("red").unwrap_or(&0) <= &max_red
            && max_color_counts.get("green").unwrap_or(&0) <= &max_green
            && max_color_counts.get("blue").unwrap_or(&0) <= &max_blue;
        if game_valid {
            // replace true with determination of whether game was possible
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
    println!(
        "Total sum of valid game numbers: {}",
        sum_of_valid_game_number
    );
    println!("Sum of powers (part 2 solution): {}", sum_of_powers);
    Ok(sum_of_valid_game_number as i64)
}

fn main() -> Result<()> {
    run(12, 13, 14)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let result = run(12, 13, 14);
        assert_eq!(result.unwrap(), 2207);
    }
}

// --- Day 2: Cube Conundrum ---
// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
// You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
// For example, the record of a few games might look like this:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.
// Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
