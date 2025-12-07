use std::str::FromStr;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(Debug, PartialEq, Eq)]
struct BatteryBank(Vec<u8>);

impl FromStr for BatteryBank {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BatteryBank(
            s.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| format!("Invalid digit: {c}"))
                        .map(|d| d as u8)
                })
                .collect::<Result<Vec<u8>, String>>()?,
        ))
    }
}

#[cfg(test)]
mod tests_y2025 {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("1", BatteryBank(vec![1]))]
    #[test_case("12", BatteryBank(vec![1, 2]))]
    #[test_case("123", BatteryBank(vec![1, 2, 3]))]
    #[test_case("1234", BatteryBank(vec![1, 2, 3, 4]))]
    #[test_case("12345", BatteryBank(vec![1, 2, 3, 4, 5]))]
    #[test_case("123456", BatteryBank(vec![1, 2, 3, 4, 5, 6]))]
    #[test_case("1234567", BatteryBank(vec![1, 2, 3, 4, 5, 6, 7]))]
    #[test_case("12345678", BatteryBank(vec![1, 2, 3, 4, 5, 6, 7, 8]))]
    #[test_case("123456789", BatteryBank(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]))]
    fn parse_battery_bank_works(subject: &str, expected: BatteryBank) {
        assert_eq!(subject.parse::<BatteryBank>().unwrap(), expected);
    }

    const INPUT: &str = include_str!("d03.txt");
    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(0));
    }

    #[test]
    #[ignore = "not implemented"]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(0));
    }
}
