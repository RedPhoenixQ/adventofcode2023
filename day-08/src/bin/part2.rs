use std::collections::BTreeMap;

use indicatif::ProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1, one_of},
    combinator::eof,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use nom_supreme::{multi::collect_separated_terminated, ParserExt};

type NodeId<'a> = &'a str;

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    id: NodeId<'a>,
    left: NodeId<'a>,
    right: NodeId<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

fn process(input: &str) -> String {
    let (_, (instructions, nodes)) = parse(input).unwrap();
    dbg!(&instructions);
    dbg!(&nodes);

    let mut instructions = instructions
        .into_iter()
        .cycle()
        .progress_count(1_000_000_000);
    let mut active_nodes: Vec<_> = nodes
        .keys()
        .filter_map(|&node_id| node_id.ends_with("A").then_some(node_id))
        .collect();

    let mut current_step = 0;
    let mut finished = false;
    while !finished {
        current_step += 1;
        finished = true;
        let next_instruciton = instructions.next().unwrap();

        for node_id in active_nodes.iter_mut() {
            let node = nodes.get(node_id).expect("all referenced nodes to exist");
            let next_node = match next_instruciton {
                Instruction::Left => node.0,
                Instruction::Right => node.1,
            };
            if finished && !next_node.ends_with("Z") {
                finished = false
            }
            *node_id = next_node;
        }
    }

    current_step.to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<Instruction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(one_of("RL"))
        .map(|chars| {
            chars
                .into_iter()
                .map(|c| match c {
                    'R' => Instruction::Right,
                    'L' => Instruction::Left,
                    _ => unreachable!("invalid char parsed"),
                })
                .collect::<Vec<_>>()
        })
        .terminated(multispace1)
        .parse(input)?;

    let (input, nodes) = collect_separated_terminated(
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
        multispace1,
        eof,
    )
    .parse(input)?;

    Ok((input, (instructions, nodes)))
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    const ANSWER: &str = "6";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
