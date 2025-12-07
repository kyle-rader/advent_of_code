use std::str::FromStr;

use anyhow::{anyhow, Context};
const DIAL_SIZE: i32 = 100;
const START_VALUE: i32 = 50;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let rotations = rotations(input).map_err(|e| e.to_string())?;

    let mut dial = Dial::default();
    dial.apply_rotations(&rotations);

    Ok(dial.zeros as u64)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

fn rotations(input: impl AsRef<str>) -> anyhow::Result<Vec<Rotation>> {
    input
        .as_ref()
        .lines()
        .map(|line| line.parse::<Rotation>())
        .collect()
}

#[derive(Debug, PartialEq)]
struct Dial {
    value: i32,
    zeros: usize,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            value: START_VALUE,
            zeros: 0,
        }
    }
}

impl Dial {
    fn apply_rotation(&mut self, rotation: &Rotation) {
        self.value = (self.value + rotation.0) % DIAL_SIZE;
        if self.value == 0 {
            self.zeros += 1;
        }
    }

    fn apply_rotations(&mut self, rotations: &[Rotation]) {
        for rotation in rotations {
            self.apply_rotation(rotation);
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rotation(i32);

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, value) = s.split_at(1);
        let value: i32 = value
            .parse()
            .context(format!("Not a valid rotation '{s}'"))?;

        let value = value % DIAL_SIZE;

        match dir.to_uppercase().as_str() {
            "L" => Ok(Rotation(-value)),
            "R" => Ok(Rotation(value)),
            _ => Err(anyhow!("Invalid direction: {}", dir)),
        }
    }
}

#[cfg(test)]
mod tests_y2025 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d01.txt");

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn example_solve() {
        let subject = part1(TEST_INPUT).unwrap();
        assert_eq!(subject, 3);
    }

    #[test_case("L90", Rotation(-90))]
    #[test_case("R80", Rotation(80))]
    #[test_case("L0", Rotation(0))]
    #[test_case("R0", Rotation(0))]
    #[test_case("L1202", Rotation(-2))]
    #[test_case("R270", Rotation(70))]
    fn parse_rotation(subject: &str, expected: Rotation) {
        assert_eq!(subject.parse::<Rotation>().unwrap(), expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(42));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
