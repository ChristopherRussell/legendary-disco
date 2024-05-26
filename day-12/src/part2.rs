use core::panic;

use hashbrown::HashMap;
use rayon::{
    *,
    iter::{IntoParallelIterator, ParallelIterator},
    str::ParallelString,
};
use regex::Regex;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

struct SpringProblem<'a> {
    springs: &'a [Spring],
    record: &'a [usize],
}

impl<'a> SpringProblem<'a> {
    fn arrangements(
        &self,
        memo: &mut HashMap<
            (&'a [Spring], &'a [usize]),
            u64,
        >,
    ) -> u64 {
        if self.record.is_empty() {
            if self.springs.iter().all(|spring| {
                matches!(
                    spring,
                    Spring::Unknown | Spring::Operational
                )
            }) {
                memo.insert(
                    (self.springs, self.record),
                    1,
                );
                return 1;
            }
            memo.insert(
                (self.springs, self.record),
                0,
            );
            return 0;
        }

        let mut position = 0;
        for (i, spring) in self.springs.iter().enumerate() {
            if matches!(
                spring,
                Spring::Damaged | Spring::Unknown
            ) {
                position = i;
                break;
            }
        }
        let springs = &self.springs[position..];
        if let Some(&count) =
            memo.get(&(springs, self.record))
        {
            return count;
        }

        let spring_len = springs.len();
        let rec = self.record[0];
        let record = &self.record[1..];

        let mut solution_count = 0;
        for (i, spring) in springs.iter().enumerate() {
            if i + rec > spring_len {
                break;
            }
            // The next `rec` springs must be unknown or
            // damaged, whilst the spring after
            // that must be unknown or operational, if it
            // exists.
            let valid =
                springs[i..i + rec].iter().all(|spring| {
                    matches!(
                        spring,
                        Spring::Unknown | Spring::Damaged
                    )
                });
            solution_count +=
                match (valid, springs.get(i + rec)) {
                    (
                        true,
                        Some(
                            Spring::Unknown
                            | Spring::Operational,
                        ),
                    ) => {
                        SpringProblem {
                            springs: &springs[i + rec + 1..],
                            record,
                        }
                            .arrangements(memo)
                    }
                    _ => 0
                };

            // If we see a damaged spring at first position,
            // then the contiguous sequence of
            // damaged springs cannot start
            // later, so break.
            if matches!(spring, Spring::Damaged) {
                break;
            }
        }
        memo.insert(
            (self.springs, self.record),
            solution_count,
        );
        solution_count
    }
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let mut pieces = line.split(" ");
    let springs = pieces.next().expect("No springs found");
    let springs: String =
        std::iter::repeat(springs.chars())
            .take(5)
            .intersperse("?".chars())
            .flatten()
            .collect();
    let mut springs = replace_dot_series_with_one(
        springs.to_string().as_str(),
    );
    // ensuring a . at the end simplifies the logic
    // since we don't need to check if we are
    // out of bounds after finding the last damaged
    // spring
    match (&springs).chars().last() {
        Some('.') => (),
        _ => springs += ".",
    };
    let springs = springs
        .chars()
        .map(|c| match c {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<Spring>>();

    let record = pieces
        .next()
        .expect("No record found")
        .split(",")
        .map(|s| s.parse().expect("Invalid number"))
        .collect::<Vec<usize>>();
    let record = std::iter::repeat(record)
        .take(5)
        .flatten()
        .collect::<Vec<usize>>();
    assert!(pieces.next().is_none());
    (springs, record)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u64, AocError> {
    Ok(input
        .par_lines()
        .into_par_iter()
        .map(|line| {
            let (springs, record) = parse_line(line);
            let memo = &mut HashMap::new();
            SpringProblem {
                springs: &springs,
                record: &record,
            }
                .arrangements(memo)
        })
        .sum())
}

fn replace_dot_series_with_one(s: &str) -> String {
    Regex::new(r"\.{2,}")
        .unwrap()
        .replace_all(s, ".")
        .to_string()
}

#[cfg(test)]
mod tests {
    use hashbrown::HashMap;

    use super::*;

    #[test]
    fn test_replace_dot_series_with_one() {
        assert_eq!(".", replace_dot_series_with_one("."));
        assert_eq!(".", replace_dot_series_with_one(".."));
        assert_eq!(".", replace_dot_series_with_one("..."));
        assert_eq!(
            ".xx.",
            replace_dot_series_with_one("..xx..")
        );
        assert_eq!(
            ".xx.x.",
            replace_dot_series_with_one("..xx..x..")
        );
        assert_eq!(
            ".x.xx.x.",
            replace_dot_series_with_one("....x..xx...x.")
        );
        assert_eq!(
            ".x.x.x.x.",
            replace_dot_series_with_one(
                "..x............x........x.....x."
            )
        );
    }

    #[test]
    fn test_parse_line() {
        let line = "?..#? 1,1";
        let (springs, record) = parse_line(line);
        assert_eq!(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            record
        );
        assert_eq!(
            vec![
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown,
                Spring::Operational,
            ],
            springs
        );
    }

    #[test]
    fn test_arrangements() {
        let line = ".??..??...?##. 1,1,3";
        let expected = 16384;
        let (springs, record) = parse_line(line);
        let memo = &mut HashMap::new();
        let problem = SpringProblem {
            springs: &springs,
            record: &record,
        };
        let solution_count = problem.arrangements(memo);
        assert_eq!(solution_count, expected);
    }

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(525152, process(TEST_INPUT)?);
        Ok(())
    }
}
