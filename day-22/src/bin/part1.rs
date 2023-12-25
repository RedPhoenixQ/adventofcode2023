use std::{collections::HashSet, ops::RangeInclusive};

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult, Parser,
};
use range_ext::intersect::Intersect;

#[derive(Debug)]
struct Block {
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

fn process(input: &str) -> String {
    let (_, mut blocks) = parse(input).unwrap();

    blocks.sort_by_key(|block| *block.z.start());
    //dbg!(&blocks, blocks.len());

    let mut tower: Vec<(usize, Block)> = Vec::new();
    let mut solo_supporting_blocks: HashSet<usize> = HashSet::new();

    for (block_i, block) in blocks.into_iter().enumerate() {
        let mut z_offset = 0;
        enum SupportingBlocks {
            None,
            One(usize),
            Many,
        }
        let mut supporting_blocks: SupportingBlocks = SupportingBlocks::None;

        for (tower_i, top_block) in tower.iter().rev() {
            if block.x.intersect(&top_block.x).is_any() && block.y.intersect(&top_block.y).is_any()
            {
                match supporting_blocks {
                    SupportingBlocks::None => {
                        z_offset = *block.z.start() - *top_block.z.end() - 1;
                        supporting_blocks = SupportingBlocks::One(*tower_i);
                    }
                    _ if *block.z.start() - *top_block.z.end() - 1 > z_offset => break,
                    _ => {
                        supporting_blocks = SupportingBlocks::Many;
                        break;
                    }
                };
            }
        }

        tower.push((
            block_i,
            Block {
                x: block.x,
                y: block.y,
                z: *block.z.start() - z_offset..=*block.z.end() - z_offset,
            },
        ));

        tower.sort_by_key(|(_, block)| *block.z.end());

        //println!("\nNEXT ROUND");
        //dbg!(&tower, z_offset, &supporting_blocks);

        if let SupportingBlocks::One(solo_support) = supporting_blocks {
            solo_supporting_blocks.insert(solo_support);
        }
    }

    //dbg!(&solo_supporting_blocks, &tower);

    dbg!(tower.len(), solo_supporting_blocks.len());

    (tower.len() - solo_supporting_blocks.len()).to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<Block>> {
    fn coords(input: &str) -> IResult<&str, (u32, u32, u32)> {
        tuple((
            complete::u32,
            delimited(complete::char(','), complete::u32, complete::char(',')),
            complete::u32,
        ))
        .parse(input)
    }

    separated_list1(
        newline,
        separated_pair(coords, complete::char('~'), coords).map(|(start, end)| Block {
            x: start.0..=end.0,
            y: start.1..=end.1,
            z: start.2..=end.2,
        }),
    )
    .parse(input)
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    const ANSWER: &str = "5";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }

    #[test]
    #[ignore = "only to test/verify logic"]
    fn range_overlap() {
        assert!((3..=3).intersect(&(1..=5)).is_any() && (1..=5).intersect(&(3..=3)).is_any())
    }
}
