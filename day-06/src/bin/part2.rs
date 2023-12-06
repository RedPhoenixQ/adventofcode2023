use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn process(input: &str) -> String {
    let race = parse(input);
    dbg!(&race);

    todo!()
}

fn parse(input: &str) -> Race {
    let input = input.replace(" ", "");
    let res: IResult<&str, Race> = map(
        separated_pair(
            preceded(tag("Time:"), i32),
            multispace1,
            preceded(tag("Distance:"), i32),
        ),
        |(time, distance)| Race { time, distance },
    )(input.as_str());
    res.unwrap().1
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
    const ANSWER: &str = "71503";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
