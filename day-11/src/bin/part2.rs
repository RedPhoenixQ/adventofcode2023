use std::collections::BTreeSet;

use glam::I64Vec2;
use itertools::Itertools;

const EXPANSION_RATIO: i64 = 1_000_000;

fn process(input: &str, ratio: i64) -> String {
    let galaxies: Vec<I64Vec2> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(move |(x, c)| (c == '#').then_some(I64Vec2::new(x as i64, y as i64)))
        })
        .collect();

    let min_x = galaxies.iter().map(|pos| pos.x).min().unwrap();
    let max_x = galaxies.iter().map(|pos| pos.x).max().unwrap();
    let min_y = galaxies.iter().map(|pos| pos.y).min().unwrap();
    let max_y = galaxies.iter().map(|pos| pos.y).max().unwrap();
    let rows_with_galaxies: BTreeSet<i64> = galaxies.iter().map(|pos| pos.x).collect();
    let columns_with_galaxies: BTreeSet<i64> = galaxies.iter().map(|pos| pos.y).collect();

    let rows_containing_galaxies = (min_x..max_x).into_iter().collect();
    let rows_to_expand = rows_with_galaxies
        .symmetric_difference(&rows_containing_galaxies)
        .collect_vec();
    let columns_containing_galaxies = (min_y..max_y).into_iter().collect();
    let columns_to_expand = columns_with_galaxies
        .symmetric_difference(&columns_containing_galaxies)
        .collect_vec();

    let expanded_galaxies = galaxies
        .into_iter()
        .map(|pos| {
            let dx = rows_to_expand.partition_point(|i| i < &&pos.x) as i64;
            let dy = columns_to_expand.partition_point(|i| i < &&pos.y) as i64;
            pos + I64Vec2::new(dx * (ratio - 1), dy * (ratio - 1))
        })
        .collect_vec();

    expanded_galaxies
        .iter()
        .enumerate()
        .flat_map(|(step, &galaxie)| {
            expanded_galaxies.iter().skip(step + 1).map(move |&other| {
                let diff = (galaxie - other).abs();
                diff.x + diff.y
            })
        })
        .sum::<i64>()
        .to_string()
}

fn main() {
    println!(
        "Output: {}",
        process(include_str!("./input.txt"), EXPANSION_RATIO)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    const ANSWER: &str = "1030";
    const EXPANSION_RATIO: i64 = 10;

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE, EXPANSION_RATIO))
    }

    const ANSWER1: &str = "8410";
    const EXPANSION_RATIO1: i64 = 100;

    #[test]
    fn example1() {
        assert_eq!(ANSWER1, process(EXAMPLE, EXPANSION_RATIO1))
    }
}
