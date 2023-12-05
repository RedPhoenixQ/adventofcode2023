use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    destination_start: u32,
    source_start: u32,
    range_len: u32,
}

impl Map<'_> {
    fn source_to_destination(&self, source: u32) -> u32 {
        if let Some(range) = self.ranges.iter().find(
            |MapRange {
                 source_start,
                 range_len,
                 ..
             }| { source > *source_start && source - source_start < *range_len },
        ) {
            range.destination_start + (source - range.source_start)
        } else {
            source
        }
    }
}

fn process(input: &str) -> String {
    let (_, (seeds, maps)) = parse(input).unwrap();
    dbg!(&seeds);
    dbg!(&maps);

    let final_dest = maps.into_iter().fold(seeds, |dest, map| {
        dest.into_iter()
            .map(|n| map.source_to_destination(n))
            .collect()
    });

    dbg!(&final_dest);

    final_dest.into_iter().min().unwrap().to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Map>)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(multispace1, u32),
        multispace1,
    )(input)?;

    let (input, maps) = separated_list1(multispace1, parse_map)(input)?;

    Ok((input, (seeds, maps)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (from, to)) = separated_pair(alpha1, tag("-to-"), alpha1)(input)?;

    let (input, ranges) = preceded(
        tuple((multispace1, tag("map:"), multispace1)),
        separated_list1(
            multispace1,
            map(
                separated_pair(separated_pair(u32, multispace1, u32), multispace1, u32),
                |((destination_start, source_start), range_len)| MapRange {
                    destination_start,
                    source_start,
                    range_len,
                },
            ),
        ),
    )(input)?;

    Ok((input, Map { from, to, ranges }))
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const ANSWER: &str = "35";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
