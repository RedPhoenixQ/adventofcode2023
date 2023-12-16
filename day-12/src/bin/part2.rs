use indicatif::ParallelProgressIterator;
use nom::{
    character::complete::{self, multispace1, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

type Mask = u128;

fn process(input: &str) -> String {
    let (_, springs) = parse(input).unwrap();

    springs
        .into_par_iter()
        .map(|(mut records, span_lengths)| {
            let base_records = records.clone();
            // Make records repeate five times with Status::Unknown inbetween
            records.push(Status::Unknown);
            records.extend_from_slice(&base_records);
            records.push(Status::Unknown);
            records.extend_from_slice(&base_records);
            records.push(Status::Unknown);
            records.extend_from_slice(&base_records);
            records.push(Status::Unknown);
            records.extend_from_slice(&base_records);

            let span_lengths: Vec<_> = span_lengths
                .iter()
                .cloned()
                .cycle()
                .take(span_lengths.len() * 5)
                .collect();

            // dbg!(records.len(), span_lengths.len());

            let operational_mask: Mask = records
                .iter()
                .enumerate()
                .filter_map(|(i, status)| matches!(status, Status::Operational).then_some(i))
                .fold(0, |mask, i| mask | 1 << i);
            let damaged_mask: Mask = records
                .iter()
                .enumerate()
                .filter_map(|(i, status)| matches!(status, Status::Damaged).then_some(i))
                .fold(0, |mask, i| mask | 1 << i);
            // println!("operational: {:032b}", operational_mask);
            // println!("damaged: {:032b}", damaged_mask);

            let spaces = (records.len() as u32) - span_lengths.iter().sum::<u32>();
            // dbg!(spaces);

            fn count_arrangements(
                mask: Mask,
                offset: u32,
                free_spaces: u32,
                spans: &[u32],
                operational_mask: Mask,
                damaged_mask: Mask,
            ) -> u32 {
                let Some(span) = spans.first() else {
                    return 0;
                };

                let spans = &spans[1..];
                let span_mask = Mask::MAX >> (Mask::BITS - span);

                if spans.is_empty() {
                    (0..=free_spaces)
                        .filter(|used_spaces| {
                            let new_offset = offset + used_spaces;
                            let new_mask = mask | span_mask << new_offset;
                            new_mask & damaged_mask == damaged_mask
                                && !new_mask & operational_mask == operational_mask
                        })
                        .count() as u32
                } else {
                    (0..free_spaces)
                        .map(|used_spaces| {
                            let new_offset = offset + used_spaces;
                            let new_mask = mask | span_mask << new_offset;
                            if new_mask & operational_mask != 0
                                || (new_offset > 0
                                    && !new_mask
                                        & (damaged_mask & (Mask::MAX >> (Mask::BITS - new_offset)))
                                        != 0)
                            {
                                return 0;
                            }

                            count_arrangements(
                                new_mask,
                                new_offset + span + 1,
                                free_spaces - used_spaces - 1,
                                spans,
                                operational_mask,
                                damaged_mask,
                            )
                        })
                        .sum()
                }
            }

            let spans = span_lengths.as_slice();
            count_arrangements(0, 0, spaces, spans, operational_mask, damaged_mask)
        })
        .progress()
        // .inspect(|n| println!("NUMBER {n}"))
        .sum::<u32>()
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
    const ANSWER: &str = "525152";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
