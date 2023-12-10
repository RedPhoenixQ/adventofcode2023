use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1, one_of},
    combinator::eof,
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use nom_supreme::{multi::collect_separated_terminated, ParserExt};

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

type NodeId<'a> = &'a str;

#[derive(Debug)]
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

    let mut instructions = instructions.into_iter().cycle();
    let mut current_node = START_NODE;
    let mut current_step = 0;
    while current_node != END_NODE {
        current_step += 1;
        let node = nodes
            .iter()
            .find(|node| node.id == current_node)
            .expect("all referenced nodes to exist");
        match instructions.next().unwrap() {
            Instruction::Left => current_node = node.left,
            Instruction::Right => current_node = node.right,
        }
    }

    current_step.to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<Instruction>, Vec<Node>)> {
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
            alpha1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alpha1, tag(", "), alpha1),
                tag(")"),
            ),
        )
        .map(|(id, (left, right))| Node { id, left, right }),
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

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const ANSWER: &str = "2";

    const EXAMPLE2: &str = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    const ANSWER2: &str = "6";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    #[test]
    fn example2() {
        assert_eq!(ANSWER2, process(EXAMPLE2))
    }
}
