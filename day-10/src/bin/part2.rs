use glam::{uvec2, UVec2};
use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NE90,
    NW90,
    SE90,
    SW90,
    Start,
}

impl Pipe {
    fn connects_to(&self, direction: &Direction) -> bool {
        match self {
            Pipe::Vertical => [Direction::North, Direction::South].contains(&direction),
            Pipe::Horizontal => [Direction::West, Direction::East].contains(&direction),
            Pipe::NE90 => [Direction::North, Direction::East].contains(&direction),
            Pipe::NW90 => [Direction::North, Direction::West].contains(&direction),
            Pipe::SE90 => [Direction::South, Direction::East].contains(&direction),
            Pipe::SW90 => [Direction::South, Direction::West].contains(&direction),
            Pipe::Start => [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .contains(&direction),
        }
    }

    fn next_direction(&self, from: &Direction) -> Option<Direction> {
        let in_direction = from.opposite();
        if !self.connects_to(&in_direction) {
            return None;
        }
        match self {
            Pipe::Vertical => [Direction::North, Direction::South],
            Pipe::Horizontal => [Direction::West, Direction::East],
            Pipe::NE90 => [Direction::North, Direction::East],
            Pipe::NW90 => [Direction::North, Direction::West],
            Pipe::SE90 => [Direction::South, Direction::East],
            Pipe::SW90 => [Direction::South, Direction::West],
            Pipe::Start => return None,
        }
        .into_iter()
        .find(|connection| connection != &in_direction)
    }
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

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Self {
        self.right().opposite()
    }

    fn translate_UVec2(&self, &UVec2 { x, y }: &UVec2) -> UVec2 {
        match self {
            Direction::North => uvec2(x, y.wrapping_sub(1)),
            Direction::East => uvec2(x.wrapping_add(1), y),
            Direction::South => uvec2(x, y.wrapping_add(1)),
            Direction::West => uvec2(x.wrapping_sub(1), y),
        }
    }
}

fn process(input: &str) -> String {
    let grid: HashMap<UVec2, Option<Pipe>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| (uvec2(x as u32, y as u32), c.try_into().ok()))
        })
        .collect();

    fn walk_pipe(
        current_pos: &UVec2,
        current_direction: Direction,
        grid: &HashMap<UVec2, Option<Pipe>>,
        pipe_tiles: &mut HashMap<UVec2, Pipe>,
        right_turns: i32,
    ) -> i32 {
        let next_pos = &current_direction.translate_UVec2(current_pos);
        let next_pipe = grid
            .get(&next_pos)
            .expect("tile to exist")
            .expect("pipe to connect to another pipe");
        pipe_tiles.insert(*next_pos, next_pipe);

        if next_pipe == Pipe::Start {
            return right_turns;
        }

        let next_direction = next_pipe
            .next_direction(&current_direction)
            .expect("to find the next");

        let turn = if current_direction == next_direction {
            0
        } else if current_direction.right() == next_direction {
            1
        } else {
            -1
        };

        walk_pipe(
            next_pos,
            next_direction,
            grid,
            pipe_tiles,
            right_turns + turn,
        )
    }

    let (start_pos, _) = grid
        .iter()
        .find(|(_, pipe)| pipe.is_some_and(|p| p == Pipe::Start))
        .expect("start pipe to exist");

    let first_direction = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .find_map(|direction| {
        grid.get(&direction.translate_UVec2(start_pos))?
            .and_then(|pipe| pipe.connects_to(&direction.opposite()).then_some(direction))
    })
    .expect("Start to have a connecting pipe");

    let mut pipe_tiles: HashMap<UVec2, Pipe> = HashMap::new();
    let right_turns = walk_pipe(start_pos, first_direction, &grid, &mut pipe_tiles, 0);
    dbg!(&pipe_tiles, right_turns);

    format!("{}", pipe_tiles.len() / 2)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    const ANSWER: &str = "8";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
