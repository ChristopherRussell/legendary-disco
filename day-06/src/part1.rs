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
    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let mut ways_to_win = Vec::new();
    for (time, distance) in
        times.iter().zip(distances.iter())
    {
        let race = Race::new(*time, *distance);
        ways_to_win.push(race.ways_to_win());
    }
    println!("ways_to_win: {:?}", ways_to_win);
    let ways_to_win_product =
        ways_to_win.iter().product::<i64>();

    println!(
        "ways_to_win_product: {:?}",
        ways_to_win_product
    );
    Ok(ways_to_win_product)
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
        assert_eq!(288, process(input)?);
        Ok(())
    }
}
