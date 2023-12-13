use glam::{ivec2, IVec2};
use std::collections::{HashMap, HashSet};

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

#[derive(Debug, PartialEq, Clone, Copy)]
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

    fn get_offset(&self) -> IVec2 {
        match self {
            Direction::North => ivec2(0, -1),
            Direction::East => ivec2(1, 0),
            Direction::South => ivec2(0, 1),
            Direction::West => ivec2(-1, 0),
        }
    }
}

fn process(input: &str) -> String {
    let grid: HashMap<IVec2, Option<Pipe>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .map(move |(x, c)| (ivec2(x as i32, y as i32), c.try_into().ok()))
        })
        .collect();

    let (&start_pos, _) = grid
        .iter()
        .find(|(_, pipe)| pipe.is_some_and(|p| p == Pipe::Start))
        .expect("start pipe to exist");

    let mut start_connections = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .filter_map(|direction| {
        grid.get(&(start_pos + direction.get_offset()))?
            .and_then(|pipe| pipe.connects_to(&direction.opposite()).then_some(direction))
    });

    let first_direction = start_connections
        .next()
        .expect("outgoing connection from start");
    let last_direction = start_connections
        .next()
        .expect("incomming connection to start");

    let start_pipe = match (first_direction, last_direction) {
        (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
            Pipe::Horizontal
        }
        (Direction::East, Direction::West) | (Direction::West, Direction::East) => Pipe::Vertical,
        (Direction::North, Direction::East) | (Direction::East, Direction::North) => Pipe::NE90,
        (Direction::North, Direction::West) | (Direction::West, Direction::North) => Pipe::NW90,
        (Direction::South, Direction::East) | (Direction::East, Direction::South) => Pipe::SE90,
        (Direction::South, Direction::West) | (Direction::West, Direction::South) => Pipe::SW90,
        _ => unreachable!("invalid start directions"),
    };

    let mut pipe_tiles: HashMap<IVec2, (Pipe, Direction)> = HashMap::new();

    let mut right_turns = 0i32;
    let mut current_pos = start_pos;
    let mut current_direction = first_direction;
    loop {
        let next_pos = current_pos + current_direction.get_offset();
        let next_pipe = grid
            .get(&next_pos)
            .expect("tile to exist")
            .expect("pipe to connect to another pipe");

        if next_pipe == Pipe::Start {
            break;
        }

        let next_direction = next_pipe
            .next_direction(&current_direction)
            .expect("to find the next");

        pipe_tiles.insert(next_pos, (next_pipe, next_direction));

        right_turns += if current_direction == next_direction {
            0
        } else if current_direction.right() == next_direction {
            1
        } else {
            -1
        };
        current_direction = next_direction;
        current_pos = next_pos;
    }

    // dbg!(&pipe_tiles, right_turns);

    let allowed_to_turn_right = right_turns.is_positive();

    pipe_tiles.insert(start_pos, (start_pipe, first_direction));

    let inner_tiles: HashSet<IVec2> = pipe_tiles
        .iter()
        .flat_map(|(pos, (pipe, direction))| {
            let next_turn = if allowed_to_turn_right {
                direction.right()
            } else {
                direction.left()
            };
            match pipe {
                Pipe::NE90 | Pipe::NW90 | Pipe::SE90 | Pipe::SW90 => {
                    if pipe.connects_to(&next_turn) {
                        return vec![];
                    }
                }
                _ => {}
            }

            [next_turn, direction.opposite()]
                .into_iter()
                .flat_map(|direction| {
                    let next_offset = direction.get_offset();
                    (1..)
                        .into_iter()
                        .map_while(|offset_multiplier| {
                            let next_pos = *pos + next_offset * offset_multiplier;
                            if pipe_tiles.get(&next_pos).is_none() {
                                Some(next_pos)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // dbg!(&inner_tiles);

    format!("{}", inner_tiles.len())
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
