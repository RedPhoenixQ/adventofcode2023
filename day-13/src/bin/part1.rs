use nom::{
    character::complete::{multispace1, one_of, line_ending},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use vecgrid::Vecgrid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[derive(Debug)]
enum RowOrColumn {
    Row(usize),
    Column(usize),
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => return Err(()),
        })
    }
}

fn process(input: &str) -> String {
    let (_, grids) = parse(input).unwrap();
    // dbg!(&grids);

    grids
        .into_iter()
        .map(|grid| {
            for row in grid.rows_iter() {
                println!("{}", row.map(|tile| match tile {
                    Tile::Ash => '.',
                    Tile::Rock => '#',
                }).collect::<String>())
            }

            let Some(row_index) = grid
                .rows_iter()
                .zip(grid.rows_iter().skip(1))
                .enumerate()
                .filter_map(|(i, (row1, row2))| {
                    row1.zip(row2)
                        .find(|(el1, el2)| el1 != el2)
                        .is_none()
                        .then_some(i)
                })
                .find(|&middle_top_i| {
                    let mut top = middle_top_i;
                    let mut bottom = middle_top_i + 1;
                    dbg!(top, bottom);
                    while let (Ok(left_row), Ok(right_row)) = (grid.row_iter(top), grid.row_iter(bottom)) {
                        if left_row
                            .zip(right_row)
                            .find(|(el1, el2)| el1 != el2)
                            .is_some()
                        {
                            return false;
                        }
                        top = top.wrapping_sub(1);
                        bottom += 1;
                    }
                    true 
                }) else {
                   return RowOrColumn::Column(grid
                .columns_iter()
                .zip(grid.columns_iter().skip(1))
                .enumerate()
                .filter_map(|(i, (column1, column2))| {
                    column1.zip(column2)
                        .find(|(el1, el2)| el1 != el2)
                        .is_none()
                        .then_some(i)
                })
                .find(|&middle_left_i| {
                let mut left = middle_left_i;
                    let mut right = middle_left_i + 1;
                    dbg!(left, right);
                    while let (Ok(left_column), Ok(right_column)) = (grid.column_iter(left), grid.column_iter(right)) {
                        if left_column
                            .zip(right_column)
                            .find(|(el1, el2)| el1 != el2)
                            .is_some()
                        {
                            return false;
                        }
                        left = left.wrapping_sub(1);
                        right += 1;
                    }
                    true
                }).expect("Horizontal mirror because there was no vertical mirror"))
                };
            RowOrColumn::Row(row_index)
        })
        .inspect(|n| println!("{n:?}"))
        .fold(0, |acc, row_or_column| match row_or_column {
            RowOrColumn::Row(row) => 100 * (row + 1),
            RowOrColumn::Column(column) => column + 1,
        } + acc)
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Vecgrid<Tile>>> {
    separated_list1(
        multispace1,
        separated_list1(
            line_ending,
            many1(one_of(".#").map(|c| -> Tile { c.try_into().unwrap() })),
        )
        .map(|rows| Vecgrid::from_rows(rows).unwrap()),
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    const ANSWER: &str = "405";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
