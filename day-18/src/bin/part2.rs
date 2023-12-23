use itertools::Itertools;
use nom::{
    character::complete::{self, alpha1, hex_digit1, multispace1, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
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
            'U' | 'u' | '3' => Direction::Up,
            'D' | 'd' | '1' => Direction::Down,
            'L' | 'l' | '2' => Direction::Left,
            'R' | 'r' | '0' => Direction::Right,
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
    steps: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

fn process(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();
    dbg!(&instructions, instructions.len());

    let mut area = 0;

    let mut inst_iter = instructions.chunks(2);

    while let Some([inst1, inst2]) = inst_iter.next() {
        let w = match inst1.direction {
            Direction::Down | Direction::Right => inst1.steps,
            Direction::Up | Direction::Left => -inst1.steps,
        };
        let h = match inst2.direction {
            Direction::Down | Direction::Right => inst2.steps,
            Direction::Up | Direction::Left => -inst2.steps,
        };
        dbg!(inst1, inst2, w, h, w * h);
        area += w * h;
    }

    // dbg!(&trench, trench.len());

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         print!(
    //             "{}",
    //             trench
    //                 .contains(&Coord { x, y })
    //                 .then_some('#')
    //                 .unwrap_or('.')
    //         );
    //     }
    //     println!("");
    // }

    (area - 1).to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        multispace1,
        delimited(
            tuple((alpha1, space1)),
            tuple((
                complete::u8,
                tuple((space1, complete::char('('), complete::char('#'))),
                hex_digit1.map(|colorcode: &str| {
                    let steps =
                        isize::from_str_radix(&colorcode[0..colorcode.len() - 1], 16).unwrap();
                    let direction = colorcode.chars().last().unwrap().try_into().unwrap();
                    Instruction { direction, steps }
                }),
            )),
            complete::char(')'),
        )
        .map(|(d, _, inst)| Instruction {
            direction: inst.direction,
            steps: d as isize,
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
    const ANSWER: &str = "952408144115";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
