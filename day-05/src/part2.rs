use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, digit1, line_ending, space1,
    },
    multi::{many0, many1, separated_list1},
    sequence::separated_pair,
    *,
};
use std::{collections::HashMap, ops::Range};
use tracing::debug;

#[derive(Debug)]
struct Seeds {
    seed_ranges: Vec<Range<u64>>,
}

struct AlmanacEntry<'a> {
    from: &'a str,
    to: &'a str,
    maps: Vec<RangeMap>,
}

impl AlmanacEntry<'_> {
    fn apply(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let mut source_ranges = vec![range];
        let mut result_ranges = vec![];
        for map in &self.maps {
            let mut new_source_ranges = vec![];
            while let Some(_range) = source_ranges.pop() {
                debug!(?_range, ?map, "applying map");
                let map_result = map.apply(_range);
                debug!(?map_result, "map result");
                if let Some(below_range) =
                    map_result.below_range
                {
                    new_source_ranges.push(below_range);
                }
                if let Some(above_range) =
                    map_result.above_range
                {
                    new_source_ranges.push(above_range);
                }
                if let Some(result) = map_result.result {
                    result_ranges.push(result);
                }
            }
            source_ranges = new_source_ranges;
        }

        // remaining ranges are mapped with the identity
        // map
        while let Some(_range) = source_ranges.pop() {
            result_ranges.push(_range);
        }
        result_ranges
    }
}

#[derive(Debug, PartialEq)]
struct RangeMap {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

#[derive(Debug, PartialEq)]
struct RangeMapResult {
    below_range: Option<Range<u64>>,
    above_range: Option<Range<u64>>,
    result: Option<Range<u64>>,
}

impl RangeMap {
    fn apply(&self, range: Range<u64>) -> RangeMapResult {
        let below_range = range.start
            ..self.source_range.start.min(range.end);
        let below_range = if below_range.is_empty() {
            None
        } else {
            Some(below_range)
        };

        let above_range =
            self.source_range.end.max(range.start)
                ..range.end;
        let above_range = if above_range.is_empty() {
            None
        } else {
            Some(above_range)
        };

        let result_source =
            self.source_range.start.max(range.start)
                ..self.source_range.end.min(range.end);
        let result = if result_source.is_empty() {
            None
        } else {
            let result_start = self.destination_range.start
                + result_source.start
                - self.source_range.start;
            let result_end = self.destination_range.end
                + result_source.end
                - self.source_range.end;
            Some(result_start..result_end)
        };

        RangeMapResult {
            below_range,
            above_range,
            result,
        }
    }
}

fn parse_seed_range(
    input: &str,
) -> IResult<&str, Range<u64>> {
    let (input, (start, length)) =
        separated_pair(digit1, space1, digit1)(input)?;
    let start = start.parse::<u64>().unwrap();
    Ok((
        input,
        start..start + length.parse::<u64>().unwrap(),
    ))
}

#[tracing::instrument(skip(input))]
fn parse_seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seed_ranges) =
        separated_list1(space1, parse_seed_range)(input)?;
    Ok((input, Seeds { seed_ranges }))
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
    let range_length = length.parse::<u64>().unwrap();
    let source_start = source.parse::<u64>().unwrap();
    let destination_start = destination.parse().unwrap();
    let map = RangeMap {
        source_range: source_start
            ..source_start + range_length,
        destination_range: destination_start
            ..destination_start + range_length,
    };
    Ok((input, map))
}

fn map_seed_range_to_location_ranges(
    seed_range: Range<u64>,
    almanac: &HashMap<&str, AlmanacEntry>,
) -> Vec<Range<u64>> {
    let mut from = "seed";
    let mut result_ranges = vec![seed_range];
    loop {
        let entry = almanac.get(from).unwrap();
        result_ranges = result_ranges
            .iter()
            .flat_map(|range| entry.apply(range.clone()))
            .collect();
        from = entry.to;
        if from == "location" {
            return result_ranges;
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
        .seed_ranges
        .into_iter()
        .flat_map(|seed| {
            let maps = map_seed_range_to_location_ranges(
                seed, &almanac,
            );
            debug!(?maps);
            maps
        })
        .map(|r| r.start)
        .min()
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
            source_range: 10..20,
            destination_range: 20..30,
        };
        let expected_result = RangeMapResult {
            below_range: None,
            above_range: None,
            result: Some(20..30),
        };
        assert_eq!(expected_result, map.apply(10..20));

        let expected_result = RangeMapResult {
            below_range: Some(0..10),
            above_range: Some(20..40),
            result: Some(20..30),
        };
        assert_eq!(expected_result, map.apply(0..40));

        let expected_result = RangeMapResult {
            below_range: None,
            above_range: None,
            result: Some(22..26),
        };
        assert_eq!(expected_result, map.apply(12..16));

        let expected_result = RangeMapResult {
            below_range: Some(2..8),
            above_range: None,
            result: None,
        };
        assert_eq!(expected_result, map.apply(2..8));

        let expected_result = RangeMapResult {
            below_range: None,
            above_range: Some(101..129),
            result: None,
        };
        assert_eq!(expected_result, map.apply(101..129));
    }

    #[test_log::test]
    fn test_almanac_entry_apply() {
        let entry = AlmanacEntry {
            from: "soil",
            to: "fertilizer",
            maps: vec![
                RangeMap {
                    source_range: 98..100,
                    destination_range: 50..52,
                },
                RangeMap {
                    source_range: 50..98,
                    destination_range: 52..100,
                },
            ],
        };
        assert_eq!(vec![81..95], entry.apply(79..93));
        assert_eq!(vec![57..70], entry.apply(55..68));
        assert_eq!(
            vec![50..52, 52..100, 0..50, 100..150],
            entry.apply(0..150)
        );
    }

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";
        let (input, seeds) = parse_seeds(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(seeds.seed_ranges, vec![79..93, 55..68]);
    }

    #[test]
    fn test_parse_map() {
        let input = "50 98 2";
        let (input, map) = parse_map(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            map,
            RangeMap {
                source_range: 98..100,
                destination_range: 50..52,
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
                source_range: 98..100,
                destination_range: 50..52,
            }
        );
        assert_eq!(
            entry.maps[1],
            RangeMap {
                source_range: 50..98,
                destination_range: 52..100,
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
        assert_eq!(46, process(TEST_INPUT)?);
        Ok(())
    }
}
