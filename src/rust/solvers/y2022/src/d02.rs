use std::str::FromStr;

use anyhow::Result;

type Round = (Hand, Hand);

#[derive(Debug, PartialEq)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl RoundResult {
    fn score(&self) -> u64 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }

    fn infer(&self, them: &Hand) -> Hand {
        match (them, self) {
            // If they have Rock, and we want a Win, use Paper.
            (Hand::Rock, RoundResult::Win) => Hand::Paper,
            (Hand::Rock, RoundResult::Draw) => Hand::Rock,
            (Hand::Rock, RoundResult::Loss) => Hand::Scissors,

            (Hand::Paper, RoundResult::Win) => Hand::Scissors,
            (Hand::Paper, RoundResult::Draw) => Hand::Paper,
            (Hand::Paper, RoundResult::Loss) => Hand::Rock,

            (Hand::Scissors, RoundResult::Win) => Hand::Rock,
            (Hand::Scissors, RoundResult::Draw) => Hand::Scissors,
            (Hand::Scissors, RoundResult::Loss) => Hand::Paper,
        }
    }
}

impl FromStr for RoundResult {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(String::from("unknown target outcome!")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn against(&self, other: &Self) -> u64 {
        let round = match (self, other) {
            // I have rock, they play scissors,  I win.
            (Hand::Rock, Hand::Scissors) => RoundResult::Win,
            (Hand::Scissors, Hand::Paper) => RoundResult::Win,
            (Hand::Paper, Hand::Rock) => RoundResult::Win,

            (Hand::Rock, Hand::Rock) => RoundResult::Draw,
            (Hand::Scissors, Hand::Scissors) => RoundResult::Draw,
            (Hand::Paper, Hand::Paper) => RoundResult::Draw,

            (Hand::Paper, Hand::Scissors) => RoundResult::Loss,
            (Hand::Rock, Hand::Paper) => RoundResult::Loss,
            (Hand::Scissors, Hand::Rock) => RoundResult::Loss,
        };
        self.score() + round.score()
    }
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(format!("unknown hand value {value}")),
        }
    }
}

fn parse_round<T: FromStr<Err = String>>(line: &str) -> Result<(Hand, T), String> {
    let mut split = line.split_whitespace();
    match (split.next(), split.next()) {
        (Some(them), Some(us)) => Ok((them.parse()?, us.parse()?)),
        (_, _) => Err(String::from("not enough hands on this line!")),
    }
}

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(input
        .lines()
        .map(parse_round)
        .collect::<Result<Vec<Round>, String>>()?
        .iter()
        .fold(0, |acc, (them, us)| acc + us.against(them)))
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(input
        .lines()
        .map(parse_round)
        .collect::<Result<Vec<(Hand, RoundResult)>, String>>()?
        .iter()
        .fold(0, |acc, (them, target)| {
            acc + target.infer(them).against(them)
        }))
}

#[allow(dead_code)]
const INPUT: &str = include_str!("d02.txt");

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    #[test]
    fn hand_try_from() {
        assert_eq!("A".parse(), Ok(Hand::Rock));
        assert_eq!("B".parse(), Ok(Hand::Paper));
        assert_eq!("C".parse(), Ok(Hand::Scissors));
    }

    #[test]
    fn round_result_try_from() {
        assert_eq!("X".parse(), Ok(RoundResult::Loss));
        assert_eq!("Y".parse(), Ok(RoundResult::Draw));
        assert_eq!("Z".parse(), Ok(RoundResult::Win));
    }

    #[test]
    fn rock_behavior() {
        assert_eq!(Hand::Rock.against(&Hand::Scissors), 7);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(9177));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(12111));
    }
}
