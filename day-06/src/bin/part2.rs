use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn process(input: &str) -> String {
    let (_, races) = parse(input).unwrap();
    dbg!(&races);

    races
        .into_iter()
        .map(|race| {
            (0..race.time)
                .filter_map(|hold_time| {
                    let boat_speed = hold_time;
                    let time_left_to_move = race.time - hold_time;
                    let distance_traveled = boat_speed * time_left_to_move;

                    if distance_traveled > race.distance {
                        Some(distance_traveled)
                    } else {
                        None
                    }
                })
                .count()
        })
        .reduce(|acc, next| acc * next)
        .unwrap()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Race>> {
    map(
        separated_pair(
            preceded(
                preceded(tag("Time:"), multispace1),
                separated_list1(multispace1, i32),
            ),
            multispace1,
            preceded(
                preceded(tag("Distance:"), multispace1),
                separated_list1(multispace1, i32),
            ),
        ),
        |(times, distances)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect()
        },
    )(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";
    const ANSWER: &str = "288";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
