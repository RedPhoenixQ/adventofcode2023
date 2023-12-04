use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn process(input: &str) -> String {
    let (_, cards) = parse(input).unwrap();
    dbg!(&cards);

    let all_wins: Vec<usize> = cards.into_iter().map(count_wins).collect();

    let map = extract_card_copies(all_wins);

    map.values().sum::<usize>().to_string()
}

fn count_wins(card: Card) -> usize {
    card.winning_numbers
        .iter()
        .filter(|num| card.numbers.contains(num))
        .count()
}

fn extract_card_copies(all_wins: Vec<usize>) -> BTreeMap<usize, usize> {
    let mut map: BTreeMap<usize, usize> = (0..all_wins.len()).map(|i| (i, 1)).collect();
    for (i, wins) in all_wins.into_iter().enumerate() {
        let repeats = *map.get(&i).unwrap_or(&1);
        for next_card in i + 1..=i + wins {
            if let Some(repeat_num) = map.get_mut(&next_card) {
                *repeat_num += repeats;
            } else {
                map.insert(next_card, repeats);
            };
        }
    }
    map
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(
        multispace1,
        map(
            tuple((
                delimited(tuple((tag("Card"), multispace1)), u32, tag(":")),
                preceded(multispace1, separated_list1(multispace1, u32)),
                preceded(
                    tuple((multispace1, tag("|"), multispace1)),
                    separated_list1(multispace1, u32),
                ),
            )),
            |(id, winning_numbers, numbers)| Card {
                id,
                winning_numbers,
                numbers,
            },
        ),
    )(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    const ANSWER: &str = "30";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    #[test]
    fn test_card_copy() {
        let all_wins: Vec<usize> = parse(EXAMPLE)
            .unwrap()
            .1
            .into_iter()
            .map(count_wins)
            .collect();

        assert_eq!(
            extract_card_copies(all_wins),
            BTreeMap::from([(0, 1), (1, 2), (2, 4), (3, 8), (4, 14), (5, 1)])
        )
    }
}
