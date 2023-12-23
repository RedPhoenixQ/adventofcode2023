use std::{cmp::Ordering, collections::HashMap, ops::Range};

use nom::{
    branch::alt,
    character::complete::{self, alpha1, newline, one_of},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult, Parser,
};

const MIN_RATING: u32 = 1;
const MAX_RATING: u32 = 4000;

#[derive(Debug, Clone)]
struct Rating {
    x: Range<u32>,
    m: Range<u32>,
    a: Range<u32>,
    s: Range<u32>,
}

impl Default for Rating {
    fn default() -> Self {
        let range = MIN_RATING..MAX_RATING + 1;
        Self {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range.clone(),
        }
    }
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
    let (_, workflows) = parse(input).unwrap();
    dbg!(&workflows);

    let mut accepted_ranges: Vec<Rating> = Vec::new();

    find_max_ranges(Rating::default(), "in", &workflows, &mut accepted_ranges);

    accepted_ranges
        .into_iter()
        .inspect(|r| println!("{r:?}"))
        .fold(0, |acc, rating| {
            acc + rating.x.len() * rating.m.len() * rating.a.len() * rating.s.len()
        })
        .to_string()
}

fn find_max_ranges(
    mut rating: Rating,
    workflow_name: &str,
    workflows: &HashMap<&str, Vec<Rule>>,
    accepted_ranges: &mut Vec<Rating>,
) {
    println!("Staring new find: {rating:?}, {workflow_name}");
    for workflow in workflows.get(workflow_name).unwrap() {
        match workflow {
            Rule::Choose {
                category,
                order,
                value,
                goto,
            } if match category {
                Category::ExtremelyCool if rating.x.contains(value) => true,
                Category::Musical if rating.m.contains(value) => true,
                Category::Aerodynamic if rating.a.contains(value) => true,
                Category::Shiny if rating.s.contains(value) => true,
                _ => false,
            } =>
            {
                let mut next_rating = rating.clone();
                match order {
                    Ordering::Greater => match category {
                        Category::ExtremelyCool => {
                            rating.x.end = *value + 1;
                            next_rating.x.start = *value + 1;
                        }
                        Category::Musical => {
                            rating.m.end = *value + 1;
                            next_rating.m.start = *value + 1;
                        }
                        Category::Aerodynamic => {
                            rating.a.end = *value + 1;
                            next_rating.a.start = *value + 1;
                        }
                        Category::Shiny => {
                            rating.s.end = *value + 1;
                            next_rating.s.start = *value + 1;
                        }
                    },
                    Ordering::Less => match category {
                        Category::ExtremelyCool => {
                            rating.x.start = *value;
                            next_rating.x.end = *value;
                        }
                        Category::Musical => {
                            rating.m.start = *value;
                            next_rating.m.end = *value;
                        }
                        Category::Aerodynamic => {
                            rating.a.start = *value;
                            next_rating.a.end = *value;
                        }
                        Category::Shiny => {
                            rating.s.start = *value;
                            next_rating.s.end = *value;
                        }
                    },
                    Ordering::Equal => unreachable!("Rules should not have equal comparisons"),
                }

                match goto {
                    Goto::Rule(next_name) => {
                        find_max_ranges(next_rating, &next_name, workflows, accepted_ranges);
                    }
                    Goto::Accept => accepted_ranges.push(next_rating),
                    Goto::Reject => {}
                }
            }
            Rule::GoTo(goto) => match goto {
                Goto::Rule(next_name) => {
                    find_max_ranges(rating.clone(), &next_name, workflows, accepted_ranges)
                }
                Goto::Accept => {
                    println!("Accpeted {rating:?} default");
                    accepted_ranges.push(rating.clone())
                }
                Goto::Reject => {}
            },
            _ => {}
        }
    }
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<Rule>>> {
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
                        .map(|(category, compare, value, _, goto)| {
                            Rule::Choose {
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
                            }
                        }),
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
    .map(|workflows| workflows.into_iter().collect())
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
    const ANSWER: &str = "167409079868000";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
