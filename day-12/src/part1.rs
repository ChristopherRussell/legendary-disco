use core::panic;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn arrangements(&self, mut solution_count: u32) -> u32 {
        if self.record.is_empty() {
            if self.springs.iter().all(|spring| {
                matches!(
                    spring,
                    Spring::Unknown | Spring::Operational
                )
            }) {
                return solution_count + 1;
            }
            return solution_count;
        }

        let rec = self.record[0];
        let record = &self.record[1..];
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
        let spring_len = springs.len();

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
            solution_count = match (
                valid,
                springs.get(i + rec),
                record.is_empty(),
            ) {
                (true, None, true) => solution_count + 1,
                (
                    true,
                    Some(
                        Spring::Unknown
                        | Spring::Operational,
                    ),
                    _,
                ) => SpringProblem {
                    springs: &springs[i + rec + 1..],
                    record,
                }
                    .arrangements(solution_count),
                _ => solution_count,
            };

            // If we see a damaged spring at first position,
            // then the continguous sequence of
            // damaged springs cannot start
            // later, so break.
            if matches!(spring, Spring::Damaged) {
                break;
            }
        }
        solution_count
    }
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    dbg!(line);
    let mut pieces = line.split(" ");
    let springs = pieces
        .next()
        .expect("No springs found")
        .chars()
        .map(|c| match c {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<Spring>>();
    let record = pieces.next().expect("No record found");
    let record = record
        .split(",")
        .map(|s| s.parse().expect("Invalid number"))
        .collect::<Vec<usize>>();
    assert!(pieces.next().is_none());
    (springs, record)
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let (springs, record) = parse_line(line);
            SpringProblem {
                springs: &springs,
                record: &record,
            }
                .arrangements(0)
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[rstest]
    #[case("? 1", 1)]
    #[case("?.###. 1,3", 1)]
    #[case("??.###. 1,3", 2)]
    #[case(".??.###. 1,3", 2)]
    #[case(".??.#?#. 1,3", 2)]
    #[case(".??.?##. 1,2", 2)]
    #[case(".??..??..?##. 1,2", 4)]
    #[case("#?#?#?#? 1,6", 1)]
    #[case("?# 1", 1)]
    fn test_arrangements(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        let (springs, record) = parse_line(line);
        let problem = SpringProblem {
            springs: &springs,
            record: &record,
        };
        let solution_count = problem.arrangements(0);
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
        assert_eq!(21, process(TEST_INPUT)?);
        Ok(())
    }
}
