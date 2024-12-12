use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    ChangeDir { dir: String },
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum OperationParseError {
    #[error("Missing command after $")]
    MissingCommand,
    #[error("No dir given to cd")]
    CDMissingDir,
    #[error("Unknown command '{0}'")]
    Unknown(String),
}

impl TryFrom<&[&str]> for Operation {
    type Error = OperationParseError;

    fn try_from(tokens: &[&str]) -> Result<Self, Self::Error> {
        match tokens.first().ok_or(OperationParseError::MissingCommand)? {
            &"cd" => Ok(Operation::ChangeDir {
                dir: tokens
                    .get(1)
                    .ok_or(OperationParseError::CDMissingDir)
                    .copied()?
                    .into(),
            }),
            other => Err(OperationParseError::Unknown(other.to_string())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputLine {
    Operation { op: Operation },
    Unknown { line: String },
}

impl OutputLine {
    pub fn new_change_dir<T: Into<String>>(dir: T) -> Self {
        Self::Operation {
            op: Operation::ChangeDir { dir: dir.into() },
        }
    }

    pub fn new_unknown<T: Into<String>>(line: T) -> Self {
        Self::Unknown { line: line.into() }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum OutputLineParseError {
    #[error("Empty input line received")]
    EmptyLine,
    #[error(transparent)]
    Operation(#[from] OperationParseError),
}

impl FromStr for OutputLine {
    type Err = OutputLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split_whitespace().collect::<Vec<&str>>();
        match tokens.first().ok_or(OutputLineParseError::EmptyLine)? {
            &"$" => Ok(Self::Operation {
                op: Operation::try_from(&tokens[1..])?,
            }),
            _ => Ok(OutputLine::new_unknown(s)),
        }
    }
}

#[allow(dead_code)]
fn part1(_input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(_input: &str) -> Result<u64, String> {
    Ok(0)
}

#[cfg(test)]
const INPUT: &str = include_str!("d07.txt");

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    mod output_line {
        use super::*;

        #[test]
        fn change_dir() {
            assert_eq!("$ cd /".parse(), Ok(OutputLine::new_change_dir("/")));
        }

        #[test]
        fn change_dir_missing_dir() {
            assert_eq!(
                "$ cd ".parse::<OutputLine>(),
                Err(OutputLineParseError::Operation(
                    OperationParseError::CDMissingDir
                ))
            );
        }

        #[test]
        fn missing_command() {
            assert_eq!(
                "$".parse::<OutputLine>(),
                Err(OutputLineParseError::Operation(
                    OperationParseError::MissingCommand
                ))
            );
        }
    }

    #[test]
    #[ignore]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(42));
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
