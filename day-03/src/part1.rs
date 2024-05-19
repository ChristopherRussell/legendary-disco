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
    let mut actual_part_numbers: Vec<PartNumber> = vec![];
    for (line, x) in input.lines().zip(0u32..) {
        let line0_parts = line1_parts;
        line1_parts = line2_parts;
        line2_parts = vec![];
        let line1_gears = line2_gears;
        line2_gears = vec![];

        line.chars()
            .zip(0u32..)
            .chunk_by(|(c, _)| match c {
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
        for gear in line1_gears {
            for part in line0_parts
                .iter()
                .chain(line1_parts.iter())
                .chain(line2_parts.iter())
            {
                if part.is_y_adjacent(gear.position) {
                    actual_part_numbers.push(*part);
                }
            }
        }
    }
    for gear in line2_gears {
        for part in
            line1_parts.iter().chain(line2_parts.iter())
        {
            if part.is_y_adjacent(gear.position) {
                actual_part_numbers.push(*part);
            }
        }
    }
    Ok(actual_part_numbers
        .into_iter()
        .map(|part| part.number)
        .sum::<u32>())
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
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
