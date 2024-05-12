use crate::custom_error::AocError;

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
    let mut digits =
        line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().expect("At least one digit");
    let part_value = if let Some(last) = digits.last() {
        format!("{first}{last}")
    } else {
        format!("{first}{first}")
    }
    .parse::<u32>()?;
    Ok(part_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_day1() {
        assert_eq!(process(EXAMPLE).unwrap(), 142);
    }
}
