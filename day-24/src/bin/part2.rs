use std::ops::RangeInclusive;

use glam::{DVec2, I64Vec3};
use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
struct Hail {
    pos: I64Vec3,
    vel: I64Vec3,
}

impl Hail {
    fn xy_intersection(&self, other: &Self) -> Option<DVec2> {
        let nominator_t = ((other.pos.x - self.pos.x) * other.vel.y
            - (other.pos.y - self.pos.y) * other.vel.x) as f64;
        let nominator_u = ((other.pos.x - self.pos.x) * self.vel.y
            - (other.pos.y - self.pos.y) * self.vel.x) as f64;
        let denominator = (self.vel.x * other.vel.y - self.vel.y * other.vel.x) as f64;

        let t = nominator_t / denominator;
        let u = nominator_u / denominator;

        (denominator != 0. && t >= 0. && u >= 0.).then(|| DVec2 {
            x: self.pos.x as f64 + t * self.vel.x as f64,
            y: self.pos.y as f64 + t * self.vel.y as f64,
        })
    }
}

fn process(input: &str, range: RangeInclusive<u64>) -> String {
    let (_, hail) = parse(input).unwrap();
    //dbg!(&hail);

    hail.into_iter()
        .combinations(2)
        .filter(|combination| {
            let [h1, h2] = combination[..] else {
                unreachable!("Every combination should contain 2 elements: {combination:?}")
            };
            let intersection = h1.xy_intersection(&h2);
            //dbg!(h1, h2, intersection);

            intersection.is_some_and(|DVec2 { x, y }| {
                range.contains(&(x as u64)) && range.contains(&(y as u64))
            })
        })
        .count()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Hail>> {
    fn i_vec_3(input: &str) -> IResult<&str, I64Vec3> {
        tuple((
            complete::i64,
            complete::char(','),
            space1,
            complete::i64,
            complete::char(','),
            space1,
            complete::i64,
        ))
        .map(|(x, _, _, y, _, _, z)| I64Vec3 { x, y, z })
        .parse(input)
    }

    separated_list1(
        newline,
        separated_pair(
            i_vec_3,
            separated_pair(space1, complete::char('@'), space1),
            i_vec_3,
        )
        .map(|(pos, vel)| Hail { pos, vel }),
    )
    .parse(input)
}

fn main() {
    println!(
        "Output: {}",
        process(
            include_str!("./input.txt"),
            200000000000000..=400000000000000
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    const ANSWER: &str = "2";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE, 7..=27))
    }

    #[test]
    fn intersection() {
        let (_, hail) = parse(EXAMPLE).unwrap();
        let mut hail_iter = hail.into_iter().combinations(2);

        let [h1, h2] = hail_iter.next().unwrap()[..] else {
            unreachable!()
        };
        dbg!(h1, h2);
        assert_eq!(
            h1.xy_intersection(&h2).unwrap(),
            DVec2 {
                x: 14.333333333333332,
                y: 15.333333333333334,
            }
        );

        let [h3, h4] = hail_iter.next().unwrap()[..] else {
            unreachable!()
        };
        dbg!(h3, h4);
        assert_eq!(
            h3.xy_intersection(&h4).unwrap(),
            DVec2 {
                x: 11.666666666666668,
                y: 16.666666666666668,
            }
        );

        let [h5, h6] = hail_iter.next().unwrap()[..] else {
            unreachable!()
        };
        dbg!(h5, h6);
        assert_eq!(
            h5.xy_intersection(&h6).unwrap(),
            DVec2 {
                x: 6.199999999999999,
                y: 19.4
            }
        );

        let [h7, h8] = hail_iter.next().unwrap()[..] else {
            unreachable!()
        };
        dbg!(h7, h8);
        assert_eq!(h7.xy_intersection(&h8), None);
    }
}
