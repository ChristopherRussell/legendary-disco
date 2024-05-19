use std::io::BufRead;

use crate::custom_error::AocError;
struct Race<T> {
    time: T,
    distance: T,
}

impl Race<i64> {
    fn new(time: i64, distance: i64) -> Self {
        Self { time, distance }
    }

    fn ways_to_win(&self) -> i64 {
        let mut i = 1;
        while (i * 2) <= self.time {
            if i * (self.time - i) > self.distance {
                break;
            }
            i += 1;
        }
        self.time - i - i + 1
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<i64, AocError> {
    let input = input.as_bytes();
    let (times, distances) = parse_input(input)?;

    let time2 = times
        .into_iter()
        .map(|x| x.to_string())
        .fold(String::new(), |x, y| x + &y)
        .parse::<i64>()
        .unwrap();
    let distance2 = distances
        .into_iter()
        .map(|x| x.to_string())
        .fold(String::new(), |x, y| x + &y)
        .parse::<i64>()
        .unwrap();
    let race2 = Race::new(time2, distance2);
    Ok(race2.ways_to_win())
}

fn parse_input(
    input: &[u8],
) -> miette::Result<(Vec<i64>, Vec<i64>), AocError> {
    let mut lines = input.lines();
    let first_line =
        lines.next().expect("Invalid input")?;
    let mut first_line = first_line.split_whitespace();
    let times = if let Some("Time:") = first_line.next() {
        first_line
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    } else {
        panic!("Invalid input");
    };
    let second_line =
        lines.next().expect("Invalid input")?;
    let mut second_line = second_line.split_whitespace();
    let distances =
        if let Some("Distance:") = second_line.next() {
            second_line
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        } else {
            panic!("Invalid input");
        };
    assert!(lines.next().is_none());
    assert_eq!(times.len(), distances.len());
    Ok((times, distances))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, process(input)?);
        Ok(())
    }
}
