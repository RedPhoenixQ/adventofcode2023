fn process(input: &str) -> String {
    let re =
        regex::Regex::new(r"[1-9]|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let first = to_digit(re.find(line).unwrap().as_str());
            let mut last = first;
            for i in 0..input.len() {
                if let Some(n) = re.find_at(line, line.len() - i) {
                    last = to_digit(n.as_str());
                    break;
                }
            }
            first * 10 + last
        })
        .sum::<i32>()
        .to_string()
}

fn to_digit(digit: &str) -> i32 {
    match digit {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => unreachable!(),
    }
}

fn main() {
    println!("Output: {}", process(include_str!("./input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    const ANSWER: &str = "281";

    #[test]
    fn example() {
        assert_eq!(ANSWER, process(EXAMPLE))
    }
}
