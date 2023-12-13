use glam::{ivec2, IVec2};
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*};

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

fn process(input: &str) -> String {
    let list_of_tiles: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.try_into().ok()).collect())
        .collect();

    let all_tiles: HashMap<IVec2, Option<Pipe>> = list_of_tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &pipe)| (ivec2(x as i32, y as i32), pipe))
        })
        .collect();

    let mut graph: DiGraph<(IVec2, Pipe), ()> = DiGraph::default();
    let mut pos_to_index: HashMap<IVec2, (NodeIndex, Pipe)> = HashMap::new();

    all_tiles.iter().for_each(|(&pos, &pipe)| {
        let Some(pipe) = pipe else {
            return;
        };
        let index = graph.add_node((pos, pipe));
        pos_to_index.insert(pos, (index, pipe));
    });

    graph.extend_with_edges(pos_to_index.iter().flat_map(|(&pos, &(index, pipe))| {
        match pipe {
            Pipe::Vertical => vec![Direction::North, Direction::South],
            Pipe::Horizontal => vec![Direction::West, Direction::East],
            Pipe::NE90 => vec![Direction::North, Direction::East],
            Pipe::NW90 => vec![Direction::North, Direction::West],
            Pipe::SE90 => vec![Direction::South, Direction::East],
            Pipe::SW90 => vec![Direction::South, Direction::West],
            Pipe::Start => [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .filter(|direction| {
                let coord = pos + direction.get_offset();
                if let Some(Some(pipe)) = all_tiles.get(&coord) {
                    match (direction, pipe) {
                        (Direction::North, Pipe::Vertical)
                        | (Direction::North, Pipe::SE90)
                        | (Direction::North, Pipe::SW90)
                        | (Direction::East, Pipe::Horizontal)
                        | (Direction::East, Pipe::NW90)
                        | (Direction::East, Pipe::SW90)
                        | (Direction::South, Pipe::Vertical)
                        | (Direction::South, Pipe::NE90)
                        | (Direction::South, Pipe::NW90)
                        | (Direction::West, Pipe::Horizontal)
                        | (Direction::West, Pipe::NE90)
                        | (Direction::West, Pipe::SE90) => true,
                        _ => false,
                    }
                } else {
                    false
                }
            })
            .collect(),
        }
        .into_iter()
        .map(move |dir| (dir, index, pos))
        .filter_map(|(direction, index, pos)| {
            Some((index, pos_to_index.get(&(pos + direction.get_offset()))?.0))
        })
    }));

    // println!("{:?}", petgraph::dot::Dot::new(&graph));

    let (start, start_pos) = pos_to_index
        .iter()
        .find_map(|(&pos, &(index, pipe))| (pipe == Pipe::Start).then_some((index, pos)))
        .expect("start pipe to exist");

    let pipe_tiles: HashMap<IVec2, Pipe> = dijkstra(&graph, start, None, |_| 1)
        .keys()
        .filter_map(|index| graph.node_weight(*index))
        .cloned()
        .collect();

    let start_pipe = match [
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
        (Direction::North, Direction::South) => Pipe::Vertical,
        (Direction::East, Direction::West) => Pipe::Horizontal,
        (Direction::North, Direction::East) => Pipe::NE90,
        (Direction::North, Direction::West) => Pipe::NW90,
        (Direction::South, Direction::East) => Pipe::SE90,
        (Direction::South, Direction::West) => Pipe::SW90,
        dir => unreachable!("invalied start connections, {dir:?}"),
    };

    list_of_tiles
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            println!("");
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
