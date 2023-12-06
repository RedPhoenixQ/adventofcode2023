use nom::{
    bytes::complete::tag,
    character::complete::{i64, multispace1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn process(input: &str) -> String {
    let race = parse(input);
    dbg!(&race);

    let mut left = 0..race.time / 2;
    let mut right = race.time / 2..race.time;
    while left.start != right.start {
        dbg!(&left, &right);

        let hold_time = right.start; // middle value
        let boat_speed = hold_time;
        let time_left_to_move = race.time - hold_time;
        let distance_traveled = boat_speed * time_left_to_move;

        let new_range = if distance_traveled > race.distance {
            // winning at this hold_time
            left
        } else {
            // not winning with this hold_time
            right
        };
        dbg!(
            distance_traveled,
            race.distance - distance_traveled,
            &new_range
        );
        let middle = new_range.start + (new_range.end - new_range.start) / 2;
        left = new_range.start..middle;
        right = middle..new_range.end;
    }
    dbg!(&left, &right);

    let min = right.end;

    let mut left = 0..race.time / 2;
    let mut right = race.time / 2..race.time;
    while left.start != right.start {
        dbg!(&left, &right);

        let hold_time = right.start; // middle value
        let boat_speed = hold_time;
        let time_left_to_move = race.time - hold_time;
        let distance_traveled = boat_speed * time_left_to_move;

        let new_range = if distance_traveled < race.distance {
            // winning at this hold_time
            left
        } else {
            // not winning with this hold_time
            right
        };
        dbg!(
            distance_traveled,
            race.distance - distance_traveled,
            &new_range
        );
        let middle = new_range.start + (new_range.end - new_range.start) / 2;
        left = new_range.start..middle;
        right = middle..new_range.end;
    }
    dbg!(&left, &right);

    let max = left.end;

    dbg!(min, max);

    format!("{}", (min..=max).count())
}

fn parse(input: &str) -> Race {
    let input = input.replace(" ", "");
    let res: IResult<&str, Race> = map(
        separated_pair(
            preceded(tag("Time:"), i64),
            multispace1,
            preceded(tag("Distance:"), i64),
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
