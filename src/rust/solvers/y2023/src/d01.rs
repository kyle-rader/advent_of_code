use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(sum_lines(input, |l| decode_line1(l)))
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(sum_lines(input, |l| decode_line2(l)))
}

fn sum_lines(input: &str, decoder: impl Fn(&str) -> Result<u64, DecodeError>) -> u64 {
    input.lines().filter_map(|l| decoder(l).ok()).sum::<u64>()
}

fn combine_first_and_last(digits: &[u32]) -> Result<u64, DecodeError> {
    let first = *digits.first().ok_or(DecodeError)? as u64;
    let last = *digits.last().ok_or(DecodeError)? as u64;
    Ok(first * 10 + last)
}

#[derive(Debug, Error)]
#[error("No Digits Found")]
struct DecodeError;

fn decode_line1(line: impl AsRef<str>) -> Result<u64, DecodeError> {
    let digits: Vec<u32> = line
        .as_ref()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    combine_first_and_last(digits.as_slice())
}

const SPELLINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn decode_line2(line: impl AsRef<str>) -> Result<u64, DecodeError> {
    let mut digits = Vec::new();
    let mut i = 0;
    let line = line.as_ref();
    let chars = line.chars().collect::<Vec<char>>();

    while i < line.len() {
        // First check if current char is a digit
        if let Some(d) = chars[i].to_digit(10) {
            digits.push(d);
            i += 1;
            continue;
        }

        // Then check if current position is the start of a spelled out digit
        for (j, spelling) in SPELLINGS.iter().enumerate() {
            if line[i..].starts_with(spelling) {
                digits.push(j as u32 + 1);
                break;
            }
        }

        // Advance i by 1 to next digit or start of spelled out digit
        i += 1;
    }

    combine_first_and_last(&digits)
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d01.txt");

    #[test_case("1abc2", 12)]
    #[test_case("pqr3stu8vwx", 38)]
    #[test_case("a1b2c3d4e5f", 15)]
    #[test_case("treb7uchet", 77)]
    fn decode1(line: &str, expected: u64) {
        assert_eq!(decode_line1(line).unwrap(), expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(55002));
    }

    #[test_case("one2three", 13)]
    #[test_case("2four5six", 26)]
    #[test_case("pqr7foobareight9ten", 79)]
    #[test_case("a1b2c3d4e5ffooseven", 17)]
    #[test_case("two123four", 24)]
    #[test_case("twone", 21)]
    #[test_case("9six9qbgcvljfvccdjslspprgonenine", 99)]
    fn decode2(line: &str, expected: u64) {
        assert_eq!(decode_line2(line).unwrap(), expected);
    }

    const PART2_EXAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part2_example() {
        assert_eq!(part2(PART2_EXAMPLE), Ok(281));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(55093));
    }
}
