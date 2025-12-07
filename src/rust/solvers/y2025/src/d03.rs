use std::str::FromStr;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let banks = input
        .lines()
        .map(|line| line.parse::<BatteryBank>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<BatteryBank>, String>>()?;
    let maxes = banks
        .iter()
        .map(|bank| bank.max_joltage())
        .collect::<Vec<u64>>();
    Ok(maxes.iter().sum())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let banks = input
        .lines()
        .map(|line| line.parse::<BatteryBank>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<BatteryBank>, String>>()?;
    let maxes = banks
        .iter()
        .map(|bank| bank.max_joltage_with_size(12))
        .collect::<Vec<u64>>();
    Ok(maxes.iter().sum())
}

#[derive(Debug, PartialEq, Eq)]
struct BatteryBank(Vec<u8>);

impl BatteryBank {
    /// Find the highest joltage by finding `size` digits, where each digit is the earliest (leftmost)
    /// maximum value within the allowed range for that position.
    fn max_joltage(&self) -> u64 {
        self.max_joltage_with_size(2)
    }

    /// Find the highest joltage with a specified number of digits.
    /// For each digit position i (0-indexed), finds the earliest max value in the range
    /// from (last_max_index + 1) to (n - size + i).
    fn max_joltage_with_size(&self, size: usize) -> u64 {
        if self.0.len() == 1 {
            // Single battery bank, return the single value.
            return self.0[0] as u64;
        }

        if size == 0 {
            return 0;
        }

        let n = self.0.len();
        let mut result = 0u64;
        let mut last_index = 0usize;

        for i in 0..size {
            let start = if i == 0 { 0 } else { last_index + 1 };
            let end = n - size + i + 1; // +1 because end is exclusive

            if start >= end {
                // Not enough elements left, pad with zeros
                break;
            }

            // Find the first (leftmost) occurrence of the maximum value in the range start..end
            let (max_index, max_value) = self
                .0
                .iter()
                .enumerate()
                .skip(start)
                .take(end - start)
                .max_by(|(i1, &x1), (i2, &x2)| {
                    match x1.cmp(&x2) {
                        std::cmp::Ordering::Equal => i2.cmp(i1), // prefer smaller index (first occurrence)
                        other => other,
                    }
                })
                .expect("Expected at least one element finding the max value");

            result = result * 10 + (*max_value as u64);
            last_index = max_index;
        }

        result
    }
}

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

    #[test_case("1919", 99)]
    #[test_case("991", 99)]
    #[test_case("99", 99)]
    #[test_case("191", 91)]
    #[test_case("119", 19)]
    #[test_case("199", 99)]
    #[test_case("987654321111111", 98)]
    #[test_case("811111111111119", 89)]
    #[test_case("234234234234278", 78)]
    #[test_case("818181911112111", 92)]
    #[test_case("2331221221361221232332583266422231222315311212133222227552392222213223325332632323212227323432113121", 97)]
    #[test_case("2233226222432274235222128222151122221191321631122222252323328313312322222318322121122332221222212221", 98)]
    #[test_case("2232233312322212322222222322212213212122222212233223222252321623322212221221232111232224332322231121", 64)]
    fn max_joltage_works(subject: &str, expected: u64) {
        assert_eq!(
            subject.parse::<BatteryBank>().unwrap().max_joltage(),
            expected
        );
    }

    const INPUT: &str = include_str!("d03.txt");
    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), Ok(357));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), Ok(3121910778619));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(17405));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(171990312704598));
    }
}
