use glam::{ivec2, IVec2};
use itertools::Itertools;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NE90,
    NW90,
    SE90,
    SW90,
    #[default]
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NE90,
            'J' => Pipe::NW90,
            '7' => Pipe::SW90,
            'F' => Pipe::SE90,
            'S' => Pipe::Start,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_offset(&self) -> IVec2 {
        match self {
            Direction::North => ivec2(0, -1),
            Direction::East => ivec2(1, 0),
            Direction::South => ivec2(0, 1),
            Direction::West => ivec2(-1, 0),
        }
    }
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn process(input: &str) -> String {
    let list_of_tiles: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.try_into().ok()).collect())
        .collect();

    let mut start_pos = ivec2(-1, -1);
    let all_tiles: HashMap<IVec2, Option<Pipe>> = list_of_tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &pipe)| (ivec2(x as i32, y as i32), pipe))
        })
        .inspect(|(pos, pipe)| {
            if pipe == &Some(Pipe::Start) {
                start_pos = *pos
            }
        })
        .collect();

    let (opposite_start_direction, start_pipe) = match [
        (Direction::North, [Pipe::Vertical, Pipe::SE90, Pipe::SW90]),
        (Direction::South, [Pipe::Vertical, Pipe::NE90, Pipe::NW90]),
        (Direction::East, [Pipe::Horizontal, Pipe::NW90, Pipe::SW90]),
        (Direction::West, [Pipe::Horizontal, Pipe::NE90, Pipe::SE90]),
    ]
    .into_iter()
    .filter_map(|(direction, valid_pipes)| {
        let pipe = all_tiles
            .get(&(start_pos + direction.get_offset()))?
            .clone()?;
        valid_pipes.contains(&pipe).then_some(direction)
    })
    .collect_tuple()
    .expect("to find two connection pipes")
    {
        // Order is guaranteed from the array above
        (dir @ Direction::North, Direction::South) => (dir, Pipe::Vertical),
        (dir @ Direction::East, Direction::West) => (dir, Pipe::Horizontal),
        (dir @ Direction::North, Direction::East) => (dir, Pipe::NE90),
        (dir @ Direction::North, Direction::West) => (dir, Pipe::NW90),
        (dir @ Direction::South, Direction::East) => (dir, Pipe::SE90),
        (dir @ Direction::South, Direction::West) => (dir, Pipe::SW90),
        dir => unreachable!("invalied start connections, {dir:?}"),
    };

    let mut current_pos = start_pos;
    let mut current_pipe = start_pipe;
    let mut last_direction = !opposite_start_direction;
    let mut pipe_tiles: HashMap<IVec2, Pipe> = HashMap::new();
    while current_pipe != Pipe::Start {
        let next_direction = match (&last_direction, current_pipe) {
            (Direction::East, Pipe::Horizontal) => Direction::East,
            (Direction::West, Pipe::Horizontal) => Direction::West,
            (Direction::North, Pipe::Vertical) => Direction::North,
            (Direction::South, Pipe::Vertical) => Direction::South,
            (Direction::South, Pipe::NE90) => Direction::East,
            (Direction::West, Pipe::NE90) => Direction::North,
            (Direction::South, Pipe::NW90) => Direction::West,
            (Direction::East, Pipe::NW90) => Direction::North,
            (Direction::North, Pipe::SE90) => Direction::East,
            (Direction::West, Pipe::SE90) => Direction::South,
            (Direction::North, Pipe::SW90) => Direction::West,
            (Direction::East, Pipe::SW90) => Direction::South,
            _ => unreachable!(
                "Invalid path was taken: {last_direction:?}, {current_pipe:?} after {} pipes",
                pipe_tiles.len()
            ),
        };

        let next_pos = current_pos + next_direction.get_offset();
        let next_pipe = all_tiles
            .get(&next_pos)
            .expect("to find next pipe in main loop")
            .expect("pipes in main loop to connect to pipes");

        pipe_tiles.insert(next_pos, next_pipe);

        current_pos = next_pos;
        current_pipe = next_pipe;
        last_direction = next_direction;
    }

    dbg!(pipe_tiles.len());

    list_of_tiles
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            // println!("");
            row.into_iter()
                .enumerate()
                .fold(
                    (false, 0, None),
                    |(old_is_inside, tiles_inside, horizontal_enter_tile), (x, tile)| {
                        let tile = tile
                            .is_some_and(|pipe| pipe == Pipe::Start)
                            .then_some(Some(start_pipe))
                            .unwrap_or(tile);
                        let is_main_tile = pipe_tiles.contains_key(&ivec2(x as i32, y as i32));
                        let (is_inside, horizontal_enter_tile) =
                            match (is_main_tile, tile, horizontal_enter_tile) {
                                // Remember when entering a pipe turn
                                (true, Some(Pipe::NE90), _) | (true, Some(Pipe::SE90), _) => {
                                    (old_is_inside, tile)
                                }
                                // Switch when crossing vertical line or exiting pipe during in
                                // other direction than the entering pipe
                                (true, Some(Pipe::Vertical), _)
                                | (true, Some(Pipe::NW90), Some(Pipe::SE90))
                                | (true, Some(Pipe::SW90), Some(Pipe::NE90)) => {
                                    (!old_is_inside, None)
                                }

                                _ => (old_is_inside, horizontal_enter_tile),
                            };

                        // if old_is_inside != is_inside {
                        //     if is_inside {
                        //         print!("\x1b[42m");
                        //     } else {
                        //         print!("\x1b[0m");
                        //     }
                        // }
                        // print!(
                        //     "{}{}{}\x1b[0m",
                        //     if is_inside {
                        //         "\x1b[42m"
                        //     } else if is_main_tile {
                        //         "\x1b[43m"
                        //     } else {
                        //         ""
                        //     },
                        //     if is_main_tile { "\x1b[1m" } else { "" },
                        //     match tile {
                        //         Some(Pipe::Vertical) => '|',
                        //         Some(Pipe::Horizontal) => '-',
                        //         Some(Pipe::NE90) => 'L',
                        //         Some(Pipe::NW90) => 'J',
                        //         Some(Pipe::SW90) => '7',
                        //         Some(Pipe::SE90) => 'F',
                        //         Some(Pipe::Start) => 'S',
                        //         None => '.',
                        //     }
                        // );

                        let is_inside_tile = is_inside && !is_main_tile;
                        let tiles_inside = tiles_inside + is_inside_tile as i32;

                        (is_inside, tiles_inside, horizontal_enter_tile)
                    },
                )
                .1
        })
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    const ANSWER: &str = "4";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    const EXAMPLE1: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    const ANSWER1: &str = "8";

    #[test]
    fn example1() {
        assert_eq!(ANSWER1, process(EXAMPLE1))
    }

    const EXAMPLE2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    const ANSWER2: &str = "10";

    #[test]
    fn example2() {
        assert_eq!(ANSWER2, process(EXAMPLE2))
    }
}
