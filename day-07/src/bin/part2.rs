use std::{cmp::Ordering, collections::BTreeMap};

use nom::{
    character::complete::{i32, multispace1, one_of},
    combinator::eof,
    multi::many1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::multi::collect_separated_terminated;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(cards: Vec<Card>, bid: i32) -> Hand {
        let mut card_counts = cards.iter().fold(BTreeMap::new(), |mut map, card| {
            if let Some(prev) = map.get(card) {
                map.insert(card, prev + 1);
            } else {
                map.insert(card, 1);
            }
            map
        });

        fn determine_hand_type(cards: BTreeMap<&Card, i32>) -> HandType {
            match (cards.iter().count(), cards.values().find(|&&num| num != 1)) {
                (1, _) => HandType::FiveOfAKind,
                (2, Some(&4)) => HandType::FourOfAKind,
                (2, _) => HandType::FullHouse,
                (3, Some(&3)) => HandType::ThreeOfAKind,
                (3, Some(&2)) => HandType::TwoPair,
                (4, _) => HandType::OnePair,
                (5, _) => HandType::HighCard,
                _ => unreachable!("invalid card count"),
            }
        }

        fn joker_permutation(
            cards: BTreeMap<&Card, i32>,
            options: &Vec<Card>,
            jokers_left: i32,
        ) -> HandType {
            dbg!(&cards, &options, &jokers_left);
            if jokers_left == 0 {
                determine_hand_type(cards)
            } else {
                options
                    .iter()
                    .map(move |card| {
                        let mut new_cards = cards.clone();
                        new_cards.insert(card, cards.get(card).unwrap() + 1);
                        joker_permutation(new_cards, options, jokers_left - 1)
                    })
                    .max()
                    .unwrap()
            }
        }

        let hand_type = if let Some(jokers) = card_counts.remove(&Card::Joker) {
            if jokers == 5 {
                HandType::FiveOfAKind
            } else {
                let options = card_counts.keys().map(|card| **card).collect();
                joker_permutation(card_counts, &options, jokers)
            }
        } else {
            determine_hand_type(card_counts)
        };

        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(&other.cards)
                .map(|(c1, c2)| c1.cmp(&c2))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            order => order,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryInto<Card> for char {
    type Error = String;
    fn try_into(self) -> Result<Card, Self::Error> {
        Ok(match self {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => return Err(format!("Invalid char '{}'", self)),
        })
    }
}

fn process(input: &str) -> String {
    let (_, mut hands) = parse(input).unwrap();
    hands.sort();
    dbg!(&hands);

    hands
        .into_iter()
        .enumerate()
        .inspect(|v| println!("{v:?}"))
        .map(|(i, hand)| hand.bid * (i + 1) as i32)
        .inspect(|v| println!("{v:?}"))
        .sum::<i32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    collect_separated_terminated(
        separated_pair(many1(one_of("AKQJT98765432")), multispace1, i32).map(|(cards, bid)| {
            let cards: Vec<Card> = cards.into_iter().map(|c| c.try_into().unwrap()).collect();
            Hand::new(cards, bid)
        }),
        multispace1,
        eof,
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const ANSWER: &str = "5905";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    #[test]
    fn card_order() {
        assert!(Card::Ace > Card::King);
        assert!(Card::Five < Card::Ten);
        assert_eq!(Card::Ace, Card::Ace);
    }

    #[test]
    fn hand_order() {
        assert!(
            Hand {
                cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair,
                bid: 100,
            } < Hand {
                cards: vec![Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five],
                hand_type: HandType::ThreeOfAKind,
                bid: 100,
            }
        );
        assert!(
            Hand {
                cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                hand_type: HandType::TwoPair,
                bid: 100,
            } > Hand {
                cards: vec![Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten],
                hand_type: HandType::TwoPair,
                bid: 100,
            }
        );
    }
}
