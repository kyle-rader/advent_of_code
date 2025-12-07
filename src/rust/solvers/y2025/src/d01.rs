use std::str::FromStr;

use anyhow::{anyhow, Context};
const DIAL_SIZE: i32 = 100;
const START_VALUE: i32 = 50;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let rotations = rotations(input).map_err(|e| e.to_string())?;

    let mut dial = Dial::default();
    dial.apply_rotations(&rotations);

    Ok(dial.zeros_landed_on as u64)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let rotations = rotations(input).map_err(|e| e.to_string())?;

    let mut dial = Dial::default();
    dial.apply_rotations(&rotations);

    Ok(dial.zeros_landed_on as u64 + dial.zeros_passed as u64)
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
    zeros_landed_on: usize,
    zeros_passed: usize,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            value: START_VALUE,
            zeros_landed_on: 0,
            zeros_passed: 0,
        }
    }
}

impl Dial {
    fn apply_rotation(&mut self, rotation: &Rotation) {
        // Add zeros passed from rotations beyond the dial size
        self.zeros_passed += rotation.0.unsigned_abs() as usize / DIAL_SIZE as usize;

        // Apply rotation
        let prev = self.value;

        self.value += rotation.0 % DIAL_SIZE;

        // If we crossed zero going negative, count it
        if self.value < 0 && prev != 0 {
            self.zeros_passed += 1;
        }

        // If we crossed zero going positive, count it
        if self.value > DIAL_SIZE && prev != 0 {
            self.zeros_passed += 1;
        }

        self.value %= DIAL_SIZE;

        // Adjust negative modulo result on dial
        if self.value < 0 {
            self.value += DIAL_SIZE;
        }

        // If we landed on zero, count it
        if self.value == 0 {
            self.zeros_landed_on += 1;
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
    fn how_does_mod_even_work() {
        assert_eq!(-101 % 100, -1);
    }

    #[test]
    fn r1000_passes_10_times() {
        let mut dial = Dial::default();
        dial.apply_rotation(&Rotation(1000));
        assert_eq!(dial.zeros_passed, 10);
    }

    #[test]
    fn example_solve_1() {
        let subject = part1(TEST_INPUT).unwrap();
        assert_eq!(subject, 3);
    }

    #[test]
    fn example_solve_2() {
        let subject = part2(TEST_INPUT).unwrap();
        assert_eq!(subject, 6);
    }

    #[test_case("L90", Rotation(-90))]
    #[test_case("R80", Rotation(80))]
    #[test_case("L0", Rotation(0))]
    #[test_case("R0", Rotation(0))]
    #[test_case("L1202", Rotation(-1202))]
    #[test_case("R270", Rotation(270))]
    fn parse_rotation(subject: &str, expected: Rotation) {
        assert_eq!(subject.parse::<Rotation>().unwrap(), expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(997));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(5978));
    }
}
