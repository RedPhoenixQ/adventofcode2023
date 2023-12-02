use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, multispace1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    movesets: Vec<Vec<Move>>,
}

#[derive(Debug, PartialEq)]
enum Move {
    Red(i32),
    Green(i32),
    Blue(i32),
}

#[derive(Debug, Default)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn process(input: &str) -> String {
    let (_, games) = parse(input).unwrap();

    games
        .into_iter()
        .map(|game| {
            game.movesets
                .into_iter()
                .map(|moves| {
                    moves.into_iter().fold(Bag::default(), |mut sum, m| {
                        match m {
                            Move::Red(n) => sum.red += n,
                            Move::Green(n) => sum.green += n,
                            Move::Blue(n) => sum.blue += n,
                        };
                        sum
                    })
                })
                .reduce(|acc, Bag { red, green, blue }| Bag {
                    red: red.max(acc.red),
                    green: green.max(acc.green),
                    blue: blue.max(acc.blue),
                })
                .unwrap_or_default()
        })
        .map(|Bag { red, green, blue }| red * green * blue)
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(
        multispace1,
        map(
            tuple((delimited(tag("Game "), i32, tag(": ")), movesets)),
            |(id, movesets)| Game { id, movesets },
        ),
    )(input)
}

fn movesets(input: &str) -> IResult<&str, Vec<Vec<Move>>> {
    separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            map(
                separated_pair(i32, space1, alt((tag("red"), tag("green"), tag("blue")))),
                |(n, color)| match color {
                    "red" => Move::Red(n),
                    "green" => Move::Green(n),
                    "blue" => Move::Blue(n),
                    _ => unreachable!(),
                },
            ),
        ),
    )(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    const ANSWER: &str = "2286";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    #[test]
    fn parser() {
        assert_eq!(
            parse(EXAMPLE).unwrap().1[0..2],
            vec![
                Game {
                    id: 1,
                    movesets: vec![
                        vec![Move::Blue(3), Move::Red(4)],
                        vec![Move::Red(1), Move::Green(2), Move::Blue(6)],
                        vec![Move::Green(2)]
                    ]
                },
                Game {
                    id: 2,
                    movesets: vec![
                        vec![Move::Blue(1), Move::Green(2)],
                        vec![Move::Green(3), Move::Blue(4), Move::Red(1)],
                        vec![Move::Green(1), Move::Blue(1)]
                    ]
                }
            ]
        )
    }
}
