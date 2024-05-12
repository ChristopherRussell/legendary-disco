use crate::custom_error::AocError;
use nom::InputIter;
use regex::Regex;
use std::str;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let part_numbers = parse_input(input)?;
    Ok(part_numbers.into_iter().sum::<u32>())
}

fn parse_input(
    input: &str,
) -> miette::Result<Vec<u32>, AocError> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> miette::Result<u32, AocError> {
    let forward_pattern = "1|2|3|4|5|6|7|8|9|0|one|two|three|four|five|six|seven|eight|nine|zero";
    let regex_forward =
        Regex::new(forward_pattern).unwrap();
    let first = str_to_u32(
        regex_forward
            .find(line)
            .expect("Should find something")
            .as_str(),
    );

    // Read the string backwards and while searching
    // for reversed patterns to find last number.
    // Makes sure first match is the last one,
    // avoiding need to look at windows of different
    // sizes. Also doesn't get tripped up by
    // overlapping numbers such as "twone" or
    // "eightwo". But not very efficient to iterate
    // twice plus have to reverse the line.
    let backward_pattern = &forward_pattern
        .iter_elements()
        .rev()
        .collect::<String>();
    let regex_backward =
        Regex::new(backward_pattern).unwrap();
    let reversed_line =
        line.iter_elements().rev().collect::<String>();
    let last = str_to_u32(
        regex_backward
            .find(&reversed_line)
            .expect("Should find something")
            .as_str()
            .chars()
            .rev()
            .collect::<String>()
            .as_str(),
    );
    let part_value =
        format!("{first}{last}").parse::<u32>()?;
    Ok(part_value)
}

fn str_to_u32(s: &str) -> u32 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        "zero" | "0" => 0,
        _ => panic!("Invalid number string {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, parse_line(line).unwrap())
    }

    #[rstest]
    #[case(
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        142
    )]
    #[case(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        281
    )]
    fn test_day1(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(process(input).unwrap(), expected);
    }
}
