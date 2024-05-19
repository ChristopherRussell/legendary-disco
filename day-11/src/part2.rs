use std::collections::HashSet;

use glam::U64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{take, take_until, take_while},
    multi::many0,
    IResult,
};
use nom_locate::{position, LocatedSpan};
use tracing::debug;

#[derive(Debug)]
struct Galaxy {
    position: U64Vec2,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<u64> {
    _process(input, 1_000_000)
}

fn _process(
    input: &str,
    factor: u64,
) -> miette::Result<u64> {
    let input = input.as_bytes();
    let (input_remainder, galaxies) =
        parse_input(input).unwrap();
    assert_eq!(input_remainder.fragment(), b"");
    debug!(?galaxies, "parsed galaxies");
    let expanded_galaxies =
        expand_galaxies(&galaxies, factor);
    debug!(?expanded_galaxies, "expanded galaxies");
    let all_pairs_distance_sum = expanded_galaxies
        .iter()
        .enumerate()
        .cartesian_product(
            expanded_galaxies.iter().enumerate(),
        )
        .filter_map(|((i, g1), (j, g2))| match i.cmp(&j) {
            std::cmp::Ordering::Less => {
                let dx =
                    g1.position.x.abs_diff(g2.position.x);
                let dy =
                    g1.position.y.abs_diff(g2.position.y);
                Some(dx + dy)
            }
            _ => None,
        })
        .sum::<u64>();
    Ok(all_pairs_distance_sum)
}

fn expand_galaxies(
    galaxies: &[Galaxy],
    factor: u64,
) -> Vec<Galaxy> {
    let rows_with_galaxies = galaxies
        .iter()
        .map(|g| g.position.x)
        .collect::<HashSet<u64>>();
    let cols_with_galaxies = galaxies
        .iter()
        .map(|g| g.position.y)
        .collect::<HashSet<u64>>();
    let max_row = *rows_with_galaxies.iter().max().unwrap();
    let max_col = *cols_with_galaxies.iter().max().unwrap();

    let mut expanded_rows_count = 0;
    let mut expanded_cols_count = 0;

    let row_expansion_amount = (1..=max_row)
        .map(|row| {
            if !rows_with_galaxies.contains(&row) {
                expanded_rows_count += factor - 1;
            }
            expanded_rows_count
        })
        .collect::<Vec<u64>>();
    let col_expansion_amount = (1..=max_col)
        .map(|col| {
            if !cols_with_galaxies.contains(&col) {
                expanded_cols_count += factor - 1;
            }
            expanded_cols_count
        })
        .collect::<Vec<u64>>();

    galaxies
        .iter()
        .map(|g| Galaxy {
            position: U64Vec2::new(
                g.position.x
                    + row_expansion_amount
                        [(g.position.x - 1) as usize],
                g.position.y
                    + col_expansion_amount
                        [(g.position.y - 1) as usize],
            ),
        })
        .collect()
}

fn parse_input(
    input: &[u8],
) -> IResult<LocatedSpan<&[u8]>, Vec<Galaxy>> {
    let span = LocatedSpan::new(input);
    let (span, galaxies) =
        many0(parse_galaxy)(span).expect("");
    let (span, _) = take_while(|c| c == b'.')(span)?;
    Ok((span, galaxies))
}

fn parse_galaxy(
    span: LocatedSpan<&[u8]>,
) -> IResult<LocatedSpan<&[u8]>, Galaxy> {
    let (span, _) = take_until("#")(span)?;
    let (span, pos) = position(span)?;
    let (span, _) = take(1usize)(span)?;
    let pos_vec = U64Vec2::new(
        pos.location_line() as u64,
        pos.get_column() as u64,
    );
    Ok((span, Galaxy { position: pos_vec }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_galaxy() {
        let input = b"...##..\n..#..#..\n";
        let (input, galaxy) =
            parse_galaxy(LocatedSpan::new(input))
                .expect("parse_galaxy failed");
        assert_eq!(galaxy.position, U64Vec2::new(1, 4));
        assert_eq!(input.fragment(), b"#..\n..#..#..\n");

        let (input, galaxy) = parse_galaxy(input)
            .expect("parse_galaxy failed");
        assert_eq!(galaxy.position, U64Vec2::new(1, 5));
        // assert_eq!(input.fragment(),
        // b"..\n..#..#..\n");

        let (input, galaxy) = parse_galaxy(input)
            .expect("parse_galaxy failed");
        assert_eq!(galaxy.position, U64Vec2::new(2, 3));
        assert_eq!(input.fragment(), b"..#..\n");

        let (input, galaxy) = parse_galaxy(input)
            .expect("parse_galaxy failed");
        assert_eq!(galaxy.position, U64Vec2::new(2, 6));
        assert_eq!(input.fragment(), b"..\n");
    }

    #[test]
    fn test_parse_input() {
        let input = b"...##..";
        let (input, galaxies) =
            parse_input(input).expect("parse_input failed");
        assert_eq!(galaxies.len(), 2);
        assert_eq!(
            galaxies[0].position,
            U64Vec2::new(1, 4)
        );
        assert_eq!(
            galaxies[1].position,
            U64Vec2::new(1, 5)
        );
        assert_eq!(input.fragment(), b"");
    }
    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_parse_test_input() {
        let (input, galaxies) =
            parse_input(TEST_INPUT.as_bytes())
                .expect("parse_input failed");
        assert_eq!(galaxies.len(), 9);
        assert_eq!(
            galaxies[0].position,
            U64Vec2::new(1, 4)
        );
        assert_eq!(
            galaxies[1].position,
            U64Vec2::new(2, 8)
        );
        assert_eq!(
            galaxies[6].position,
            U64Vec2::new(9, 8)
        );
        assert_eq!(
            galaxies[7].position,
            U64Vec2::new(10, 1)
        );
        assert_eq!(
            galaxies[8].position,
            U64Vec2::new(10, 5)
        );
        assert_eq!(input.fragment(), b"");
    }

    #[test]
    fn test_expand_galaxies() {
        let galaxies = vec![
            Galaxy {
                position: U64Vec2::new(1, 5),
            },
            Galaxy {
                position: U64Vec2::new(4, 3),
            },
            Galaxy {
                position: U64Vec2::new(6, 2),
            },
        ];
        let expanded_galaxies =
            expand_galaxies(&galaxies, 10);
        assert_eq!(
            expanded_galaxies[0].position,
            U64Vec2::new(1, 23)
        );
        assert_eq!(
            expanded_galaxies[1].position,
            U64Vec2::new(22, 12)
        );
        assert_eq!(
            expanded_galaxies[2].position,
            U64Vec2::new(33, 11)
        );
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(1030, _process(TEST_INPUT, 10)?);
        assert_eq!(8410, _process(TEST_INPUT, 100)?);
        Ok(())
    }
}
