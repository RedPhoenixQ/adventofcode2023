fn process(input: &str) -> String {
    let symbols: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' && !c.is_numeric() {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let numbers: Vec<_> = symbols
        .into_iter()
        .flat_map(|(x, y)| {
            let mut out: Vec<i32> = vec![];
            for line_i in [y - 1, y, y + 1] {
                let Some(line) = lines.get(line_i) else {
                    continue;
                };
                if line.get(x).is_some_and(|c| c.is_numeric()) {
                    let mut max_left = x;
                    let mut max_right = x;
                    while line
                        .get(max_left.saturating_sub(1))
                        .is_some_and(|c| c.is_numeric())
                    {
                        max_left -= 1;
                    }
                    while line.get(max_right + 0).is_some_and(|c| c.is_numeric()) {
                        max_right += 1;
                    }
                    out.push(
                        line.get(max_left..max_right)
                            .unwrap()
                            .iter()
                            .collect::<String>()
                            .parse()
                            .unwrap(),
                    );
                } else {
                    let mut max = x.saturating_sub(1);
                    if line.get(max).is_some_and(|c| c.is_numeric()) {
                        while line
                            .get(max.wrapping_sub(1))
                            .is_some_and(|c| c.is_numeric())
                        {
                            max -= 1;
                        }
                        out.push(
                            line.get(max..x)
                                .unwrap()
                                .iter()
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                        );
                    }
                    let mut max = x + 1;
                    if line.get(max).is_some_and(|c| c.is_numeric()) {
                        while line.get(max + 1).is_some_and(|c| c.is_numeric()) {
                            max += 1;
                        }
                        out.push(
                            line.get(x + 1..=max)
                                .unwrap()
                                .iter()
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                        );
                    }
                }
            }
            out
        })
        .collect();
    dbg!(&numbers);

    numbers.into_iter().sum::<i32>().to_string()
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    const ANSWER: &str = "4361";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
