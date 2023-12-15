use nom::{
    character::complete::{self, multispace1, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Status {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => return Err(()),
        })
    }
}

type Mask = u32;

fn process(input: &str) -> String {
    let (_, springs) = parse(input).unwrap();

    springs
        .into_iter()
        .map(|(records, span_lengths)| {
            let operational_mask: Mask = records
                .iter()
                .enumerate()
                .filter_map(|(i, status)| matches!(status, Status::Operational).then_some(i))
                .fold(0u32, |mask, i| mask | 1 << i);
            let damaged_mask: Mask = records
                .iter()
                .enumerate()
                .filter_map(|(i, status)| matches!(status, Status::Damaged).then_some(i))
                .fold(0u32, |mask, i| mask | 1 << i);
            println!("operational: {:032b}", operational_mask);
            println!("damaged: {:032b}", damaged_mask);

            let spaces = (records.len() as u32) - span_lengths.iter().sum::<u32>();
            dbg!(spaces);

            fn find_arrangements(
                mask: Mask,
                offset: u32,
                free_spaces: u32,
                spans: &[u32],
                results: &mut Vec<Mask>,
            ) {
                let Some(span) = spans.first() else {
                results.push(mask);
                return;
            };
                let spans = &spans[1..];
                let span_mask = u32::MAX >> (32 - span);

                if spans.is_empty() {
                    for used_spaces in 0..=free_spaces {
                        let new_offset = offset + used_spaces;
                        find_arrangements(
                            mask | span_mask << new_offset,
                            new_offset + span + 1,
                            free_spaces - used_spaces,
                            spans,
                            results,
                        );
                    }
                } else {
                    for used_spaces in 0..free_spaces {
                        let new_offset = offset + used_spaces;
                        find_arrangements(
                            mask | span_mask << new_offset,
                            new_offset + span + 1,
                            free_spaces - used_spaces - 1,
                            spans,
                            results,
                        );
                    }
                }
            }

            let mut results = Vec::new();
            let spans = span_lengths.as_slice();
            find_arrangements(0, 0, spaces, spans, &mut results);

            results
                .into_iter()
                .filter(|mask| {
                    mask & damaged_mask == damaged_mask
                        && !mask & operational_mask == operational_mask
                })
                .inspect(|mask| println!("PRODUCED: {:32b}", mask))
                .count()
        })
        .inspect(|n| println!("NUMBER {n}"))
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<(Vec<Status>, Vec<u32>)>> {
    separated_list1(
        multispace1,
        separated_pair(
            many1(one_of(".#?").map(|c| c.try_into().unwrap())),
            multispace1,
            separated_list1(complete::char(','), complete::u32),
        ),
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    const ANSWER: &str = "21";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
