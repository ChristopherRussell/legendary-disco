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
    let result = parse_input(input)
        .into_iter()
        .map(get_prediction_part1)
        .sum();
    println!("Result: {}", result);
    Ok(result)
}

fn part2(input: &[u8]) -> Result<i64> {
    let result = parse_input(input)
        .into_iter()
        .map(get_prediction_part2)
        .sum();
    println!("Result: {}", result);
    Ok(result)
}

fn parse_input(input: &[u8]) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| parse_one_line(&line.unwrap()))
        .collect_vec()
}

fn parse_one_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec()
}

fn vec_diff(v: &[i64]) -> Vec<i64> {
    if v.len() < 2 {
        panic!("Vector must have at least 2 elements to diff. If the vec is not all zeros yet then perhaps something has gone wrong.")
    }
    v.iter()
        .skip(1)
        .zip(v.iter())
        .map(|(next, prev)| next - prev)
        .collect_vec()
}

fn is_all_zero(v: &[i64]) -> bool {
    v.iter().all(|&x| x == 0)
}

fn get_prediction_part1(v: Vec<i64>) -> i64 {
    // No need to calculate the next values for every vector.
    // The next prediction is the sum of the last values in each vector.
    let mut prediction = *v.last().unwrap();
    let mut diff = v;
    loop {
        diff = vec_diff(&diff);

        if is_all_zero(&diff) {
            println!("all zero diff, returning prediction: {}", prediction);
            break;
        }
        prediction += diff.last().unwrap();
    }
    prediction
}

fn get_prediction_part2(v: Vec<i64>) -> i64 {
    // No need to calculate the prev values for every vector.
    // Can just return the sum x1 - x2 + x3 - x4 ... of first values with alternating sign.
    let mut prediction = 0;
    let mut diff = v;
    let mut subtract_from_prediction = false;
    loop {
        if subtract_from_prediction {
            prediction -= diff.first().unwrap();
        } else {
            prediction += diff.first().unwrap();
        }
        subtract_from_prediction = !subtract_from_prediction;

        diff = vec_diff(&diff);
        println!("diff: {:?}", diff);

        if is_all_zero(&diff) {
            println!("all zero diff, returning prediction: {}", prediction);
            break;
        }
    }
    prediction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_line() {
        let line = "1 2 3 4";
        let expected = vec![1, 2, 3, 4];
        assert_eq!(parse_one_line(line), expected);
    }

    #[test]
    fn test_vec_diff() {
        let v = vec![1, 2, 3, 4];
        let expected = vec![1, 1, 1];
        assert_eq!(vec_diff(&v), expected);
    }

    #[test]
    fn test_is_all_zero() {
        let v = vec![0, 0, 0];
        assert!(is_all_zero(&v));
        let v = vec![0, 0, 1];
        assert!(!is_all_zero(&v));
    }

    const TEST_EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1_examaple() {
        let expected = 114;
        assert_eq!(part1(TEST_EXAMPLE_INPUT.as_bytes()).unwrap(), expected);
    }

    #[test]
    fn test_part2_examaple() {
        let expected = 2;
        assert_eq!(part2(TEST_EXAMPLE_INPUT.as_bytes()).unwrap(), expected);
    }
}
