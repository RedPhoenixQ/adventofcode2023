use std::collections::{BinaryHeap, HashMap, VecDeque};

use petgraph::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    const CARDINAL_DIRECTIONS: [Coord; 4] = [
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: -1 },
        Coord { x: 1, y: 0 },
        Coord { x: -1, y: 0 },
    ];
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn process(input: &str) -> String {
    let grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().map(move |(x, c)| {
                (
                    Coord {
                        x: x as isize,
                        y: y as isize,
                    },
                    c,
                )
            })
        })
        .collect();
    let end_coord = Coord {
        y: (input.lines().count() - 1) as isize,
        x: (input.lines().next().unwrap().len() - 2) as isize,
    };

    let mut graph: DiGraph<Coord, ()> = DiGraph::new();

    let start_coord = Coord { x: 1, y: 0 };
    let start_index = graph.add_node(start_coord);
    let mut coord_node: HashMap<Coord, NodeIndex> = HashMap::from([(start_coord, start_index)]);

    let mut steps = VecDeque::from([((start_coord, start_coord), start_index)]);

    dbg!(&grid, &graph, &steps);

    while let Some(((coord, prev), prev_i)) = steps.pop_front() {
        for diff in Coord::CARDINAL_DIRECTIONS {
            let next = coord + diff;
            if next == prev {
                continue;
            }
            let Some(c) = grid.get(&next) else {
                continue;
            };
            match (c, diff) {
                ('#', _)
                | ('^', Coord { x: 0, y: 1 })
                | ('>', Coord { x: -1, y: 0 })
                | ('v', Coord { x: 0, y: -1 })
                | ('<', Coord { x: 1, y: 0 }) => continue,
                _ => {}
            }
            let node_i = coord_node
                .get(&next)
                .and_then(|n| Some(*n))
                .unwrap_or_else(|| {
                    let n = graph.add_node(next);
                    coord_node.insert(next, n);
                    n
                });
            graph.update_edge(prev_i, node_i, ());
            steps.push_back(((next, coord), node_i));
        }
    }
    // println!("{:?}", petgraph::dot::Dot::new(&graph));
    let end_index = coord_node
        .get(&end_coord)
        .expect("end node to exist in graph");

    #[derive(Debug, PartialEq, Eq, PartialOrd)]
    struct QueueItem {
        steps: usize,
        node: NodeIndex,
    }
    impl Ord for QueueItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.steps.cmp(&other.steps)
        }
    }

    let mut queue: BinaryHeap<QueueItem> = BinaryHeap::from([QueueItem {
        steps: 0,
        node: start_index,
    }]);

    let mut max_steps = 0;

    while let Some(QueueItem { steps, node }) = queue.pop() {
        if &node == end_index {
            max_steps = max_steps.max(steps);
            continue;
        }
        for neighbor in graph.neighbors(node) {
            queue.push(QueueItem {
                steps: steps + 1,
                node: neighbor,
            });
        }
    }

    max_steps.to_string()
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    const ANSWER: &str = "94";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
