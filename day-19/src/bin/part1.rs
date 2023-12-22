use std::{cmp::Ordering, collections::HashMap};

use nom::{
    branch::alt,
    character::complete::{self, alpha1, multispace1, newline, one_of},
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};

#[derive(Debug)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(Debug)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny,
}

impl TryFrom<char> for Category {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'x' => Self::ExtremelyCool,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Rule<'a> {
    Choose {
        category: Category,
        order: Ordering,
        value: u32,
        goto: Goto<'a>,
    },
    GoTo(Goto<'a>),
}

#[derive(Debug)]
enum Goto<'a> {
    Rule(&'a str),
    Accept,
    Reject,
}

fn process(input: &str) -> String {
    let (_, (workflows, ratings)) = parse(input).unwrap();
    dbg!(&workflows, &ratings);

    let start_workflow = workflows.get("in").expect("'in' workflow to exist");

    ratings
        .into_iter()
        .filter_map(|rating| {
            let mut workflow = start_workflow;
            loop {
                let goto = workflow
                    .iter()
                    .find_map(|rule| match rule {
                        Rule::GoTo(goto) => Some(goto),
                        Rule::Choose {
                            category,
                            order,
                            value,
                            goto,
                        } => (match category {
                            Category::ExtremelyCool => rating.x,
                            Category::Musical => rating.m,
                            Category::Aerodynamic => rating.a,
                            Category::Shiny => rating.s,
                        }
                        .cmp(value)
                            == *order)
                            .then_some(goto),
                    })
                    .expect("atleast one rule to match");

                match goto {
                    Goto::Accept => return Some(rating),
                    Goto::Reject => return None,
                    Goto::Rule(name) => {
                        workflow = workflows.get(name).expect("linked workflows to exist");
                    }
                };
            }
        })
        .map(|r| r.x + r.m + r.a + r.s)
        .sum::<u32>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (HashMap<&str, Vec<Rule>>, Vec<Rating>)> {
    fn rating(input: &str) -> IResult<&str, u32> {
        delimited(
            tuple((one_of("xmas"), complete::char('='))),
            complete::u32,
            opt(complete::char(',')),
        )
        .parse(input)
    }

    separated_pair(
        separated_list1(
            newline,
            tuple((
                alpha1,
                delimited(
                    complete::char('{'),
                    separated_list1(
                        complete::char(','),
                        alt((
                            tuple((
                                one_of("xmas"),
                                one_of("<>"),
                                complete::u32,
                                complete::char(':'),
                                alpha1,
                            ))
                            .map(
                                |(category, compare, value, _, goto)| Rule::Choose {
                                    category: category.try_into().unwrap(),
                                    value,
                                    goto: match goto {
                                        "A" => Goto::Accept,
                                        "R" => Goto::Reject,
                                        name => Goto::Rule(name),
                                    },
                                    order: match compare {
                                        '<' => Ordering::Less,
                                        '>' => Ordering::Greater,
                                        _ => unreachable!("Invalid compare character parsed"),
                                    },
                                },
                            ),
                            complete::alpha1.map(|goto| {
                                Rule::GoTo(match goto {
                                    "A" => Goto::Accept,
                                    "R" => Goto::Reject,
                                    name => Goto::Rule(name),
                                })
                            }),
                        )),
                    ),
                    complete::char('}'),
                ),
            )),
        )
        .map(|workflows| workflows.into_iter().collect()),
        multispace1,
        separated_list1(
            newline,
            delimited(
                complete::char('{'),
                tuple((rating, rating, rating, rating)).map(|(x, m, a, s)| Rating { x, m, a, s }),
                complete::char('}'),
            ),
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

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    const ANSWER: &str = "19114";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
