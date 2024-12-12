use std::collections::HashMap;

use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, InputError> {
    let (a, b) = parse_input(input)?;
    let distance = a.iter().zip(b.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    Ok(distance)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let (left, right) = parse_input(input).map_err(|e| e.to_string())?;
    let mut counts: HashMap<u64, u64> = HashMap::new();
    right
        .iter()
        .for_each(|&n| *counts.entry(n).or_default() += 1);

    Ok(left
        .into_iter()
        .map(|n| n * counts.get(&n).unwrap_or(&0))
        .sum())
}

#[derive(Debug, PartialEq, Eq, Error)]
enum InputError {
    #[error("missing first number")]
    MissingFirstNumber,

    #[error("missing second number")]
    MissingSecondNumber,

    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

fn parse_line(line: &str) -> Result<(u64, u64), InputError> {
    let mut parts = line.split_whitespace();
    let a = parts
        .next()
        .ok_or(InputError::MissingFirstNumber)?
        .parse()?;
    let b = parts
        .next()
        .ok_or(InputError::MissingSecondNumber)?
        .parse()?;
    Ok((a, b))
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<u64>), InputError> {
    let lists: Result<Vec<(u64, u64)>, _> = input.lines().map(parse_line).collect();

    let (mut a, mut b): (Vec<u64>, Vec<u64>) = lists?.into_iter().unzip();
    a.sort();
    b.sort();
    Ok((a, b))
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;
    use test_case::test_case;

    #[test_case("", Err(InputError::MissingFirstNumber))]
    #[test_case("1", Err(InputError::MissingSecondNumber))]
    #[test_case("1 2", Ok((1, 2)))]
    #[test_case("12      24", Ok((12, 24)))]
    fn parse_line_works(input: &str, expected: Result<(u64, u64), InputError>) {
        assert_eq!(parse_line(input), expected);
    }

    #[test]
    fn parse_input_works() {
        let subject = parse_input(TEST_INPUT).expect("parse_input failed");
        assert_eq!(subject, (vec![1, 2, 3, 3, 3, 4], vec![3, 3, 3, 4, 5, 9]));
    }

    const INPUT: &str = include_str!("d01.txt");

    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(765748));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(27732508));
    }
}
