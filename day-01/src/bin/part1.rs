fn process(input: &str) -> String {
    let numbers: Vec<i32> = input
        .lines()
        .map(|line| {
            let mut digits = line.matches(char::is_numeric);
            let first: i32 = digits
                .next()
                .expect("atleast one digit per line")
                .parse()
                .unwrap();
            let last = digits.last().and_then(|d| d.parse().ok()).unwrap_or(first);
            first * 10 + last
        })
        .collect();
    dbg!(&numbers);
    numbers
        .into_iter()
        .fold(0, |sum, n| sum + n as i32)
        .to_string()
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    const ANSWER: &str = "142";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
