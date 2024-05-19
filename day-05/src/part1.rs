use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, digit1, line_ending, space1,
    },
    multi::{many0, many1, separated_list1},
    *,
};
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
struct Seeds {
    seeds: Vec<u64>,
}

struct AlmanacEntry<'a> {
    from: &'a str,
    to: &'a str,
    maps: Vec<RangeMap>,
}

impl AlmanacEntry<'_> {
    fn apply(&self, value: u64) -> u64 {
        for map in &self.maps {
            if let Some(result) = map.apply(value) {
                debug!(self.from, ?value, self.to, ?result);
                return result;
            }
        }
        debug!(self.from, ?value, self.to, ?value);
        value
    }
}

#[derive(Debug, PartialEq)]
struct RangeMap {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl RangeMap {
    fn apply(&self, value: u64) -> Option<u64> {
        if (self.source_start <= value)
            & (value < self.source_start + self.length)
        {
            return Some(
                self.destination_start + value
                    - self.source_start,
            );
        }
        None
    }
}

#[tracing::instrument(skip(input))]
fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) =
        separated_list1(space1, digit1)(input)?;
    let seeds = Seeds {
        seeds: seeds
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect(),
    };

    Ok((input, seeds))
}

fn parse_all_entries(
    input: &str,
) -> IResult<&str, HashMap<&str, AlmanacEntry>> {
    let mut almanac_entires =
        HashMap::<&str, AlmanacEntry>::new();
    let (input, entries) =
        many1(parse_almanac_entry)(input)?;
    for entry in entries {
        almanac_entires.insert(entry.from, entry);
    }
    Ok((input, almanac_entires))
}

fn parse_almanac_entry(
    input: &str,
) -> IResult<&str, AlmanacEntry> {
    let (input, _) = many0(line_ending)(input)?;
    let (input, (from, to)) = parse_almanac_name(input)?;
    let (input, _) = tag(" map:")(input)?;
    let (input, maps) = many1(parse_map)(input)?;
    Ok((input, AlmanacEntry { from, to, maps }))
}

fn parse_almanac_name(
    input: &str,
) -> IResult<&str, (&str, &str)> {
    let (input, from) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = alpha1(input)?;
    Ok((input, (from, to)))
}

fn parse_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, _) = many0(line_ending)(input)?;
    let (input, destination) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, source) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, length) = digit1(input)?;
    let map = RangeMap {
        source_start: source.parse().unwrap(),
        destination_start: destination.parse().unwrap(),
        length: length.parse().unwrap(),
    };
    Ok((input, map))
}

fn map_seed_to_location(
    seed: u64,
    almanac: &HashMap<&str, AlmanacEntry>,
) -> u64 {
    let mut from = "seed";
    let mut value = seed;
    loop {
        let entry = almanac.get(from).unwrap();
        value = entry.apply(value);
        from = entry.to;
        if from == "location" {
            return value;
        }
    }
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<u64, AocError> {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, almanac) =
        parse_all_entries(input).unwrap();
    assert_eq!(input, "");
    let min_location = seeds
        .seeds
        .into_iter()
        .map(|seed| map_seed_to_location(seed, &almanac))
        .reduce(
            |acc, location| {
                if location < acc {
                    location
                } else {
                    acc
                }
            },
        )
        .unwrap();
    Ok(min_location)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_map_apply() {
        let map = RangeMap {
            source_start: 10,
            destination_start: 20,
            length: 10,
        };
        assert_eq!(Some(20), map.apply(10));
        assert_eq!(Some(21), map.apply(11));
        assert_eq!(Some(29), map.apply(19));
        assert_eq!(None, map.apply(20));
        assert_eq!(None, map.apply(9));
    }

    #[test]
    fn test_almanac_entry_apply() {
        let entry = AlmanacEntry {
            from: "soil",
            to: "fertilizer",
            maps: vec![
                RangeMap {
                    source_start: 10,
                    destination_start: 20,
                    length: 10,
                },
                RangeMap {
                    source_start: 30,
                    destination_start: 0,
                    length: 10,
                },
            ],
        };
        assert_eq!(20, entry.apply(10));
        assert_eq!(21, entry.apply(11));
        assert_eq!(29, entry.apply(19));
        assert_eq!(0, entry.apply(30));
        assert_eq!(1, entry.apply(31));
        assert_eq!(9, entry.apply(39));
        assert_eq!(40, entry.apply(40));
        assert_eq!(99999, entry.apply(99999));
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let (input, seeds) = parse_seeds(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(seeds.seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_map() {
        let input = "50 98 2";
        let (input, map) = parse_map(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            map,
            RangeMap {
                source_start: 98,
                destination_start: 50,
                length: 2
            }
        );
    }

    #[test]
    fn test_parse_almanac_name() {
        let input = "seed-to-soil map:";
        let (input, (from, to)) =
            parse_almanac_name(input).unwrap();
        assert_eq!(input, " map:");
        assert_eq!(from, "seed");
        assert_eq!(to, "soil");
    }

    #[test]
    fn test_parse_almanac_entry() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let (input, entry) =
            parse_almanac_entry(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(entry.from, "seed");
        assert_eq!(entry.to, "soil");
        assert_eq!(entry.maps.len(), 2);
        assert_eq!(
            entry.maps[0],
            RangeMap {
                source_start: 98,
                destination_start: 50,
                length: 2
            }
        );
        assert_eq!(
            entry.maps[1],
            RangeMap {
                source_start: 50,
                destination_start: 52,
                length: 48
            }
        );
    }

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse_all_entries() {
        let (input, _) = parse_seeds(TEST_INPUT).unwrap();
        let (input, entries) =
            parse_all_entries(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(entries.len(), 7);
        assert_eq!(
            entries
                .keys()
                .copied()
                .sorted()
                .collect::<Vec<&str>>(),
            vec![
                "fertilizer",
                "humidity",
                "light",
                "seed",
                "soil",
                "temperature",
                "water",
            ]
        )
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        assert_eq!(35, process(TEST_INPUT)?);
        Ok(())
    }
}
