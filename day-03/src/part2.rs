use std::slice::Iter;

use crate::custom_error::AocError;
use glam::u32::UVec2;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct PartNumber {
    number: u32,
    start_position: UVec2,
}

impl PartNumber {
    fn new(number: u32, x: u32, y: u32) -> Self {
        Self {
            number,
            start_position: UVec2::new(x, y),
        }
    }
    fn length(&self) -> u32 {
        self.number.to_string().len() as u32
    }

    fn is_y_adjacent(&self, position: UVec2) -> bool {
        let y = self.start_position.y;
        let y2 = position.y;
        (y + self.length() >= y2) && (y2 + 1 >= y)
    }
}

#[derive(Debug)]
struct Gear {
    position: UVec2,
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let mut line1_parts: Vec<PartNumber> = vec![];
    let mut line2_parts: Vec<PartNumber> = vec![];
    let mut line2_gears: Vec<Gear> = vec![];
    let mut part_number_ratio_sum = 0u32;
    for (line, x) in input.lines().zip(0u32..) {
        let line0_parts = line1_parts;
        line1_parts = line2_parts;
        line2_parts = vec![];
        let line1_gears = line2_gears;
        line2_gears = vec![];

        line.chars()
            .zip(0u32..)
            .group_by(|(c, _)| match c {
                '.' => '.',
                '0'..='9' => '#',
                _ => '*',
            })
            .into_iter()
            .for_each(|(key, mut group)| {
                match key {
                    '#' => {
                        let mut group = group.peekable();
                        let (_, y) = *group.peek().unwrap();
                        let number = group
                            .map(|(c, _)| c)
                            .collect::<String>()
                            .parse::<u32>();
                        line2_parts.push(PartNumber::new(
                            number.unwrap(),
                            x,
                            y,
                        ));
                    }
                    '*' => {
                        let (_, y) = group.next().unwrap();
                        line2_gears.push(Gear {
                            position: UVec2::new(x, y),
                        });
                    }
                    '.' => {}
                    _ => unreachable!(),
                };
            });
        part_number_ratio_sum += find_part_numbers(
            &line1_gears,
            &line0_parts,
            &line1_parts,
            &line2_parts,
        );
    }
    part_number_ratio_sum += find_part_numbers(
        &line2_gears,
        &line1_parts,
        &line2_parts,
        &[],
    );
    Ok(part_number_ratio_sum)
}

fn find_part_numbers(
    gears: &[Gear],
    parts0: &[PartNumber],
    parts1: &[PartNumber],
    parts2: &[PartNumber],
) -> u32 {
    let mut part_number_ratio_sum = 0u32;
    for gear in gears {
        let mut adjacent_parts = vec![];
        for part in parts0
            .iter()
            .chain(parts1.iter())
            .chain(parts2.iter())
        {
            if part.is_y_adjacent(gear.position) {
                adjacent_parts.push(part.number);
            }
        }
        if adjacent_parts.len() == 2 {
            part_number_ratio_sum +=
                adjacent_parts.into_iter().product::<u32>();
        }
    }
    part_number_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, process(input)?);
        Ok(())
    }
}
