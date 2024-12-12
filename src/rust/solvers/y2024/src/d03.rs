use regex::Regex;
use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<i64, ParseError> {
    Ok(mul_matches(input)?.iter().sum())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(PartialEq, Eq, Debug, Error)]
enum ParseError {
    #[error(transparent)]
    IntParse(#[from] std::num::ParseIntError),
}

const DO_PATTERN: &str = r"do()";
const DONT_PATTERN: &str = "don't()";
const MUL_PATTERN: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
fn mul_matches(input: &str) -> Result<Vec<i64>, ParseError> {
    let re = Regex::new(MUL_PATTERN).unwrap();
    let mut matches = Vec::new();
    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<i64>()?;
        let b = cap[2].parse::<i64>()?;
        matches.push(a * b);
    }
    Ok(matches)
}

enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;

    const INPUT: &str = include_str!("d03.txt");
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn mul_matches_works() {
        assert_eq!(mul_matches(TEST_INPUT), Ok(vec![8, 25, 88, 40]));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(173419328));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
