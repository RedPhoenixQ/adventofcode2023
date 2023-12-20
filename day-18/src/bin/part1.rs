use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    character::complete::{self, hex_digit1, multispace1, one_of, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'U' | 'u' => Direction::Up,
            'D' | 'd' => Direction::Down,
            'L' | 'l' => Direction::Left,
            'R' | 'r' => Direction::Right,
            _ => return Err(()),
        })
    }
}

impl std::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: u8,
    color: (u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn process(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();
    dbg!(&instructions);

    let mut trench: HashSet<Coord> = HashSet::new();
    let mut pos = Coord { x: 0, y: 0 };
    let mut left_turns = 0;

    for instruction in instructions {
        for _ in 0..instruction.steps {
            trench.insert(pos);
            match instruction.direction {
                Direction::Down => pos.y += 1,
                Direction::Up => pos.y -= 1,
                Direction::Left => pos.x -= 1,
                Direction::Right => pos.x += 1,
            }
        }
    }

    dbg!(&trench, trench.len());

    let min_x = trench.iter().map(|coord| coord.x).min().unwrap();
    let max_x = trench.iter().map(|coord| coord.x).max().unwrap();
    let min_y = trench.iter().map(|coord| coord.y).min().unwrap();
    let max_y = trench.iter().map(|coord| coord.y).max().unwrap();

    dbg!(min_x, max_x, min_y, max_y);

    let mut todo: Vec<(Coord, Direction)> = Vec::from(&[(Coord { x: 1, y: 1 }, Direction::Right)]);

    while let Some((coord, from)) = todo.pop() {
        dbg!(todo.len());

        trench.insert(coord);

        for direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            if !from == direction {
                continue;
            }
            let next = match direction {
                Direction::Down => Coord {
                    x: coord.x,
                    y: coord.y + 1,
                },
                Direction::Up => Coord {
                    x: coord.x,
                    y: coord.y - 1,
                },
                Direction::Left => Coord {
                    x: coord.x - 1,
                    y: coord.y,
                },
                Direction::Right => Coord {
                    x: coord.x + 1,
                    y: coord.y,
                },
            };
            if trench.contains(&next) {
                continue;
            }
            todo.push((next, direction));
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                trench
                    .contains(&Coord { x, y })
                    .then_some('#')
                    .unwrap_or('.')
            );
        }
        println!("");
    }

    trench.len().to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        multispace1,
        tuple((
            one_of("UDLR").map(|c| c.try_into().unwrap()),
            space1,
            complete::u8,
            tuple((space1, complete::char('('), complete::char('#'))),
            hex_digit1.map(|colorcode: &str| {
                (0..6)
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&colorcode[i..i + 2], 16).unwrap())
                    .collect_tuple()
                    .unwrap()
            }),
            complete::char(')'),
        ))
        .map(|(direction, _, steps, _, color, _)| Instruction {
            direction,
            steps,
            color,
        }),
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)";
    const ANSWER: &str = "62";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
