use anyhow::anyhow;
use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(input
        .lines()
        .filter_map(|l| decode_line(l).ok())
        .map(|d| d as u64)
        .sum::<u64>())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(Debug, Error)]
#[error("No Digits Found")]
struct DecodeError;

fn decode_line(line: impl AsRef<str>) -> Result<u32, DecodeError> {
    let digits: Vec<u32> = line
        .as_ref()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    let first = digits.first().ok_or(DecodeError)?;
    let last = digits.last().ok_or(DecodeError)?;
    Ok(first * 10 + last)
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d01.txt");

    const LINE1: &str = "1abc2";
    const LINE2: &str = "pqr3stu8vwx";
    const LINE3: &str = "a1b2c3d4e5f";
    const LINE4: &str = "treb7uchet";

    #[test_case(LINE1, 12)]
    #[test_case(LINE2, 38)]
    #[test_case(LINE3, 15)]
    #[test_case(LINE4, 77)]
    fn decode_single_line(line: &str, expected: u32) {
        assert_eq!(decode_line(line).unwrap(), expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(55002));
    }

    // #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
