use itertools::Itertools;

fn process(input: &str) -> String {
    let histories: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().expect("input to be number"))
                .collect()
        })
        .collect();

    fn find_next_in_sequence(list: Vec<i32>) -> i32 {
        let mut only_zeros = true;
        let differences = list
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .inspect(|n| {
                if n != &0 {
                    only_zeros = false;
                }
            })
            .collect_vec();
        dbg!(&list, &differences, only_zeros);
        let first = *list.first().expect("to have elements in difference array");
        if !only_zeros {
            let next = find_next_in_sequence(differences);
            dbg!(first, next);
            first - next
        } else {
            first
        }
    }

    histories
        .into_iter()
        .map(find_next_in_sequence)
        .inspect(|next| println!("{next}"))
        .sum::<i32>()
        .to_string()
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    const ANSWER: &str = "2";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
