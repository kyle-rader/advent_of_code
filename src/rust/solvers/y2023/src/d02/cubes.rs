use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum CubeError {
    #[error("missing count")]
    MissingCount,

    #[error("missing color")]
    MissingColor,

    #[error("unknown color: {0}")]
    UnknownColor(String),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

impl FromStr for Cube {
    type Err = CubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let count = parts
            .next()
            .ok_or(CubeError::MissingCount)?
            .parse::<u32>()?;

        let color = parts.next().ok_or(CubeError::MissingColor)?.to_lowercase();

        match color.as_str() {
            "red" => Ok(Cube::Red(count)),
            "green" => Ok(Cube::Green(count)),
            "blue" => Ok(Cube::Blue(count)),
            _ => Err(CubeError::UnknownColor(color)),
        }
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum GameError {
    #[error("missing game id")]
    MissingId,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    Cube(#[from] CubeError),
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn total(&self) -> u32 {
        self.red + self.green + self.blue
    }
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let id = parts
            .next()
            .ok_or(GameError::MissingId)?
            .trim_start_matches("Game ")
            .trim_end_matches(':')
            .parse::<u32>()?;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in parts.next().unwrap_or("").split(';') {
            for cube in set.split(',') {
                let cube = cube.trim();
                let cube = cube.parse::<Cube>()?;
                match cube {
                    Cube::Red(count) => red = red.max(count),
                    Cube::Green(count) => green = green.max(count),
                    Cube::Blue(count) => blue = blue.max(count),
                }
            }
        }

        Ok(Game {
            id,
            red,
            green,
            blue,
        })
    }
}

#[cfg(test)]
mod tests_y2023_cubes {
    use super::*;
    use test_case::test_case;

    #[test_case("5 Red", Ok(Cube::Red(5)))]
    #[test_case("2 red", Ok(Cube::Red(2)))]
    fn can_make_cubes(input: &str, expected: Result<Cube, CubeError>) {
        let subject = input.parse::<Cube>();
        assert_eq!(subject, expected);
    }

    #[test]
    fn can_make_games() {
        let input = "Game 21: 2 blue, 3 red; 3 green, 3 blue, 6 red; 4 blue, 6 red; 2 green, 2 blue, 9 red; 2 red, 4 blue";
        let subject = input.parse::<Game>();
        let expected = Game {
            id: 21,
            red: 9,
            green: 3,
            blue: 4,
        };
        assert_eq!(subject, Ok(expected));
    }
}
