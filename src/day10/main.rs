use anyhow::Result;
use core::panic;
use hashbrown::HashMap;
use num::Integer;
use std::{hash::Hash, io::BufRead};

const INPUT: &[u8] = include_bytes!("input.txt");

fn main() {
    if let Err(e) = part1(INPUT) {
        eprintln!("Part 1 Error: {}", e);
    }
    if let Err(e) = part2(INPUT) {
        eprintln!("Part 2 Error: {}", e);
    }
}

#[derive(Clone, Copy, Debug)]
enum PipeShape {
    Vertical,
    Horizontal,
    Bend { goes_up: bool, goes_right: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Pipe {
    position: Position,
    shape: PipeShape,
}

#[derive(Clone, Debug)]
enum Tile {
    Pipe {
        position: Position,
        shape: PipeShape,
    },
    Ground {
        position: Position,
    },
    Start {
        position: Position,
    },
}

impl Tile {
    fn position(&self) -> Position {
        match self {
            Tile::Pipe { position, .. } => *position,
            Tile::Ground { position } => *position,
            Tile::Start { position } => *position,
        }
    }
}

impl Tile {
    fn from_char_and_position(position: Position, c: u8) -> Self {
        match c {
            b'.' => Tile::Ground { position },
            b'S' => Tile::Start { position },
            b'-' => Tile::Pipe {
                position,
                shape: PipeShape::Horizontal,
            },
            b'|' => Tile::Pipe {
                position,
                shape: PipeShape::Vertical,
            },
            b'7' => Tile::Pipe {
                position,
                shape: PipeShape::Bend {
                    goes_up: false,
                    goes_right: false,
                },
            },
            b'F' => Tile::Pipe {
                position,
                shape: PipeShape::Bend {
                    goes_up: false,
                    goes_right: true,
                },
            },
            b'J' => Tile::Pipe {
                position,
                shape: PipeShape::Bend {
                    goes_up: true,
                    goes_right: false,
                },
            },
            b'L' => Tile::Pipe {
                position,
                shape: PipeShape::Bend {
                    goes_up: true,
                    goes_right: true,
                },
            },
            _ => panic!("Invalid tile character: {}", c),
        }
    }
}

impl Pipe {
    fn neighbours(&self) -> (Position, Position) {
        let Pipe {
            position: Position { x, y },
            shape,
        } = self;
        let (x, y) = (*x, *y);
        match shape {
            PipeShape::Horizontal => (Position { x, y: y - 1 }, Position { x, y: y + 1 }),
            PipeShape::Vertical => (Position { x: x - 1, y }, Position { x: x + 1, y }),
            PipeShape::Bend {
                goes_up,
                goes_right,
            } => {
                let horizontal_neighbour = if *goes_right {
                    Position { x, y: y + 1 }
                } else {
                    assert!(y > 0);
                    Position { x, y: y - 1 }
                };
                let vertical_neighbour = if *goes_up {
                    assert!(x > 0);
                    Position { x: x - 1, y }
                } else {
                    Position { x: x + 1, y }
                };
                (vertical_neighbour, horizontal_neighbour)
            }
        }
    }
}

fn part1(input: &[u8]) -> Result<i64> {
    let (position_to_tile, start) = parse_input_to_hash_map(input);
    if start.is_none() {
        return Err(anyhow::anyhow!("No start position found"));
    }
    let start = start.unwrap();
    let mut prev = position_to_tile.get(&start).unwrap().clone();
    let start_neighbours = find_a_pipe_neighbours_for_start(&position_to_tile, start).clone();
    let mut current = start_neighbours.0;
    let mut visited_count: i64 = 1;
    loop {
        match current {
            Tile::Pipe { position, shape } => {
                visited_count += 1;
                let pipe = Pipe { position, shape };
                let (position1, position2) = pipe.neighbours();
                let next = if position1 == prev.position() {
                    position2
                } else {
                    position1
                };
                prev = current;
                current = position_to_tile.get(&next).unwrap().clone();
            }
            Tile::Start { .. } => break,
            Tile::Ground { .. } => unreachable!("Pipe should not be connected to ground"),
        }
    }
    let (div, rem) = visited_count.div_rem(&2);
    if rem == 0 {
        println!("Visited count: {}, furthest point: {}", visited_count, div);
        Ok(div)
    } else {
        Err(anyhow::anyhow!(
            "Visited count was odd, something has gone wrong!"
        ))
    }
}

fn find_a_pipe_neighbours_for_start(
    position_to_tile: &HashMap<Position, Tile>,
    start: Position,
) -> (Tile, Tile) {
    let mut neighbours = Vec::new();
    for (x, y) in [(0i64, 1i64), (1, 0), (0, -1), (-1, 0)].iter() {
        if let Some(tile) = position_to_tile.get(&Position {
            x: ((start.x as i64) + x) as usize,
            y: ((start.y as i64) + y) as usize,
        }) {
            if let Tile::Pipe { .. } = tile {
                neighbours.push(tile);
            }
        }
    }
    if neighbours.len() == 2 {
        (neighbours[0].clone(), neighbours[1].clone())
    } else {
        panic!(
            "Start position did not have exactly two pipe neighbours, found: {:?}",
            neighbours
        );
    }
}

fn count_inner_tiles(row_nr: usize, line: &str, nest_tile_map: &HashMap<Position, Tile>) -> i64 {
    let mut edges_passed_through = 0i64;
    let mut nest_area_counter = 0;
    let mut current_nest_pipe_segment: Vec<PipeShape> = Vec::new();
    for (y, c) in line.bytes().enumerate() {
        let position = Position { x: row_nr, y };
        let is_nest_pipe = nest_tile_map.contains_key(&position);
        let tile = Tile::from_char_and_position(position, c);
        match (tile, is_nest_pipe) {
            (Tile::Pipe { position: _, shape }, true) => current_nest_pipe_segment.push(shape),
            (Tile::Start { .. }, _) => {
                unreachable!(
                    "Start tile should have been replaced with appropriate pipe tile in part2"
                );
            }
            (tile, _) => {
                if !current_nest_pipe_segment.is_empty() {
                    if current_nest_pipe_segment.len() > 1 {
                        match (
                            current_nest_pipe_segment.first(),
                            current_nest_pipe_segment.last(),
                        ) {
                            (
                                Some(PipeShape::Bend { goes_up: true, .. }),
                                Some(PipeShape::Bend { goes_up: false, .. }),
                            )
                            | (
                                Some(PipeShape::Bend { goes_up: false, .. }),
                                Some(PipeShape::Bend { goes_up: true, .. }),
                            ) => edges_passed_through += 1,
                            _ => {}
                        }
                    }
                    current_nest_pipe_segment.clear();
                }
                if let (Tile::Ground { .. }, true) = (tile, edges_passed_through.is_odd()) {
                    nest_area_counter += 1;
                }
            }
        }
    }
    nest_area_counter
}

fn part2(input: &[u8]) -> Result<i64> {
    let (mut position_to_tile, start) = parse_input_to_hash_map(input);
    if start.is_none() {
        return Err(anyhow::anyhow!("No start position found"));
    }
    let start = start.unwrap();
    todo!("Get a hash map of only the nest pipes, and pass that to find_a_pipe_neighours_for_start, and later usages of position_to_tile are also wrong");
    let (neighbour1, neighbour2) = find_a_pipe_neighbours_for_start(&position_to_tile, start);
    // insert pipe tile in place of start tile. need to check the neighbours being above or below
    let Position { x, y } = start;
    let (Position { x: x1, y: y1 }, Position { x: x2, y: y2 }) =
        (neighbour1.position(), neighbour2.position());
    if x1.abs_diff(x2) == 2 {
        position_to_tile.insert(
            start,
            Tile::Pipe {
                position: start,
                shape: PipeShape::Vertical,
            },
        );
    } else if y1.abs_diff(y2) == 2 {
        position_to_tile.insert(
            start,
            Tile::Pipe {
                position: start,
                shape: PipeShape::Horizontal,
            },
        );
    } else {
        let (goes_up, goes_right) = ((x1 < x) | (x2 < x), (y1 > y) | (y2 > y));
        position_to_tile.insert(
            start,
            Tile::Pipe {
                position: start,
                shape: PipeShape::Bend {
                    goes_up,
                    goes_right,
                },
            },
        );
    }

    let nest_tiles = input
        .lines()
        .enumerate()
        .map(|(row_nr, line)| count_inner_tiles(row_nr, line.unwrap().as_str(), &position_to_tile))
        .sum::<i64>();

    println!("Nest tiles: {}", nest_tiles);

    Ok(nest_tiles)
}

fn parse_input_to_hash_map(input: &[u8]) -> (HashMap<Position, Tile>, Option<Position>) {
    let mut map = HashMap::new();
    let mut start = None;
    input.lines().enumerate().for_each(|(x, line)| {
        line.unwrap()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(y, tile)| {
                let position = Position { x, y };
                map.insert(position, Tile::from_char_and_position(position, *tile));
                if *tile == b'S' {
                    start = Some(position);
                }
            })
    });
    (map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(EXAMPLE_INPUT1.as_bytes()).unwrap(), 4);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(part1(EXAMPLE_INPUT2.as_bytes()).unwrap(), 8);
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(part2(EXAMPLE_INPUT1.as_bytes()).unwrap(), 1);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(part2(EXAMPLE_INPUT2.as_bytes()).unwrap(), 1);
    }

    fn test_pipe_neighbours(
        x: usize,
        y: usize,
        shape: PipeShape,
        expected1: Position,
        expected2: Position,
    ) {
        let pipe = Pipe {
            position: Position { x, y },
            shape,
        };
        let neighbours = pipe.neighbours();
        assert_eq!(neighbours.0, expected1);
        assert_eq!(neighbours.1, expected2);
    }
    #[test]
    fn test_neighbours() {
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Vertical,
            Position { x: 1, y: 2 },
            Position { x: 3, y: 2 },
        );
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Horizontal,
            Position { x: 2, y: 1 },
            Position { x: 2, y: 3 },
        );
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Bend {
                goes_up: true,
                goes_right: true,
            },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 3 },
        );
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Bend {
                goes_up: false,
                goes_right: true,
            },
            Position { x: 3, y: 2 },
            Position { x: 2, y: 3 },
        );
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Bend {
                goes_up: true,
                goes_right: false,
            },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 1 },
        );
        test_pipe_neighbours(
            2,
            2,
            PipeShape::Bend {
                goes_up: false,
                goes_right: false,
            },
            Position { x: 3, y: 2 },
            Position { x: 2, y: 1 },
        );
    }
}
