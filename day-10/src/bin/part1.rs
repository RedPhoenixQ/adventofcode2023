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

fn process(input: &str) -> String {
    let pipe_tiles: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.try_into().ok()).collect())
        .collect();

    let mut graph: DiGraph<Pipe, ()> = DiGraph::default();
    let mut grid: HashMap<(usize, usize), (NodeIndex, Pipe)> = HashMap::new();

    pipe_tiles.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, pipe)| {
            let Some(pipe) = pipe else {
            return;
        };
            let index = graph.add_node(*pipe);
            grid.insert((x, y), (index, *pipe));
        })
    });

    graph.extend_with_edges(grid.iter().flat_map(|(&(x, y), &(index, pipe))| {
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
                let coord = match direction {
                    Direction::North => (x, y.wrapping_sub(1)),
                    Direction::East => (x.wrapping_add(1), y),
                    Direction::South => (x, y.wrapping_add(1)),
                    Direction::West => (x.wrapping_sub(1), y),
                };
                if let Some((_, tile)) = grid.get(&coord) {
                    match (direction, tile) {
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
        .map(move |dir| (dir, index, (x, y)))
        .filter_map(|(direction, index, (x, y))| {
            let coord = match direction {
                Direction::North => (x, y.wrapping_sub(1)),
                Direction::East => (x.wrapping_add(1), y),
                Direction::South => (x, y.wrapping_add(1)),
                Direction::West => (x.wrapping_sub(1), y),
            };
            Some((index, grid.get(&coord)?.0))
        })
    }));

    // println!("{:?}", petgraph::dot::Dot::new(&graph));

    let (start, _) = grid
        .values()
        .find(|(_, pipe)| pipe == &Pipe::Start)
        .expect("start pipe to exist");

    dijkstra(&graph, *start, None, |_| 1)
        .values()
        .max()
        .unwrap()
        .to_string()
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
