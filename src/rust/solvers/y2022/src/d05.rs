use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StackParseError {
    #[error("No crate rows found!")]
    NoCrates,
    #[error("Ran into a row with a different number of stacks than the first row")]
    Misaligned,
    #[error("An unexpected number of characters were found in a row")]
    UnexpectedNumberOfChars,
}

#[cfg(test)]
const INPUT: &str = include_str!("d05.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = StackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Chop off the "id row" because we don't need it
        let rows: Vec<&str> = s.lines().collect();
        let last_row = rows.len().checked_sub(1).ok_or(StackParseError::NoCrates)?;
        let rows = &rows[0..last_row];

        // Init the stacks
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let row_length = rows.first().ok_or(StackParseError::NoCrates)?.len();
        let stack_count = (row_length + 1) / 4;

        (0..stack_count).for_each(|_| stacks.push(Vec::new()));

        // Iterate over rows in reverse, so we can use 'push' to build the stacks (instead of stacks.insert(0, c))
        for row in rows.iter().rev() {
            let row: Vec<char> = row.chars().collect();
            if (row_length + 1) % 4 != 0 {
                return Err(StackParseError::UnexpectedNumberOfChars);
            }
            if row.len() != row_length {
                return Err(StackParseError::Misaligned);
            }

            for (i, stack) in stacks.iter_mut().enumerate().take(stack_count) {
                //  x :0123456789
                // row:[Z] [M] [P]
                // i  : 0   1   2
                let j = i * 4 + 1;
                match row[j] {
                    ' ' => {}
                    c => stack.push(c),
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

pub enum MoveType {
    Part1,
    Part2,
}

impl Stacks {
    fn get_mut(&mut self, i: usize) -> Option<&mut Vec<char>> {
        self.0.get_mut(i)
    }

    pub fn execute_moves(
        &mut self,
        moves: &[Move],
        move_type: MoveType,
    ) -> Result<String, MoveCratesError> {
        for Move(amount, from_idx, to_idx) in moves.iter() {
            match move_type {
                MoveType::Part1 => self.move_one_by_one(*amount, *from_idx, *to_idx)?,
                MoveType::Part2 => self.move_group(*amount, *from_idx, *to_idx)?,
            }
        }

        Ok(self
            .0
            .iter()
            .filter_map(|s| s.last())
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(""))
    }

    fn move_one_by_one(
        &mut self,
        amount: u32,
        from_idx: usize,
        to_idx: usize,
    ) -> Result<(), MoveCratesError> {
        for _ in 0..amount {
            let from = self.get_mut(from_idx - 1).ok_or_else(|| {
                MoveCratesError::MissingStack(format!("from stack {from_idx} is not here"))
            })?;

            // un-borrow stacks, so we can borrow again to get the 'to' stack.
            let from = from
                .pop()
                .ok_or(MoveCratesError::OutOfCrates(from_idx - 1))?;

            let to = self.get_mut(to_idx - 1).ok_or_else(|| {
                MoveCratesError::MissingStack(format!("to stack {to_idx} is not here"))
            })?;

            to.push(from);
        }
        Ok(())
    }

    fn move_group(
        &mut self,
        amount: u32,
        from_idx: usize,
        to_idx: usize,
    ) -> Result<(), MoveCratesError> {
        let mut temp = Vec::new();
        for _ in 0..amount {
            let from = self.get_mut(from_idx - 1).ok_or_else(|| {
                MoveCratesError::MissingStack(format!("from stack {from_idx} is not here"))
            })?;

            let from = from
                .pop()
                .ok_or(MoveCratesError::OutOfCrates(from_idx - 1))?;

            temp.push(from);
        }

        let to = self.get_mut(to_idx - 1).ok_or_else(|| {
            MoveCratesError::MissingStack(format!("to stack {to_idx} is not here"))
        })?;

        for _ in 0..temp.len() {
            to.push(temp.pop().unwrap());
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move(u32, usize, usize);

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MoveParseError {
    #[error("No Amount found in line")]
    NoAmount,
    #[error("No From found in line")]
    NoFrom,
    #[error("No To found in line")]
    NoTo,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let amount: u32 = parts.get(1).ok_or(MoveParseError::NoAmount)?.parse()?;
        let from: usize = parts.get(3).ok_or(MoveParseError::NoFrom)?.parse()?;
        let to: usize = parts.get(5).ok_or(MoveParseError::NoTo)?.parse()?;
        Ok(Move(amount, from, to))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CrateTrackerParseError {
    #[error("Could not split starting state and moves from initial input")]
    Split,
    #[error(transparent)]
    Stack(#[from] StackParseError),
    #[error(transparent)]
    Move(#[from] MoveParseError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MoveCratesError {
    #[error("{0}")]
    MissingStack(String),
    #[error("Out of crates in stack {0}")]
    OutOfCrates(usize),
}

pub fn parse_problem(s: &str) -> Result<(Stacks, Vec<Move>), CrateTrackerParseError> {
    let splits = s.split("\n\n").collect::<Vec<&str>>();
    let [stacks, moves] = &splits[..] else {
        return Err(CrateTrackerParseError::Split);
    };
    let stacks: Stacks = stacks.parse()?;
    let moves: Vec<Move> = moves
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Move>, _>>()?;
    Ok((stacks, moves))
}

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    mod stacks {
        use super::*;

        #[test]
        fn from_str() {
            let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";
            let expected = Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
            let subject: Result<Stacks, StackParseError> = input.parse();
            assert_eq!(subject, Ok(expected));
        }

        #[test]
        fn no_input() {
            let subject: Result<Stacks, StackParseError> = "".parse();
            assert_eq!(subject, Err(StackParseError::NoCrates));
        }

        #[test]
        fn can_actually_have_any_chars() {
            let input = "$$$ [D]    
[?] [🤔]    
[Z] [M] [P]
 1   2   3 
";
            let subject: Result<Stacks, StackParseError> = input.parse();
            let expected = Stacks(vec![vec!['Z', '?', '$'], vec!['M', '🤔', 'D'], vec!['P']]);
            assert_eq!(subject, Ok(expected));
        }

        #[test]
        fn mis_aligned_second_row() {
            let input = "[D]
[?] [🤔]
[Z] [M] [P]
 1   2   3 
";
            let subject: Result<Stacks, StackParseError> = input.parse();
            assert_eq!(subject, Err(StackParseError::Misaligned));
        }

        #[test]
        fn invalid_number_of_chars() {
            let input = "[D  M]
[D  M]
 1   2   3 
";
            let subject: Result<Stacks, StackParseError> = input.parse();
            assert_eq!(subject, Err(StackParseError::UnexpectedNumberOfChars));
        }
    }

    mod moves {
        use super::*;

        #[test]
        fn move_parse() -> Result<(), MoveParseError> {
            let input = "move 666 from 1 to 7";
            let subject = input.parse();
            let expected = Move(666, 1, 7);
            assert_eq!(subject, Ok(expected));
            Ok(())
        }

        #[test]
        fn nothing() {
            let subject: Result<Move, _> = "".parse();
            let expected = Err(MoveParseError::NoAmount);
            assert_eq!(subject, expected);
        }

        #[test]
        fn no_from() {
            let subject: Result<Move, _> = "move 42".parse();
            let expected = Err(MoveParseError::NoFrom);
            assert_eq!(subject, expected);
        }

        #[test]
        fn no_to() {
            let subject: Result<Move, _> = "move 42 from 1337".parse();
            let expected = Err(MoveParseError::NoTo);
            assert_eq!(subject, expected);
        }

        #[test]
        fn not_int() {
            let subject: Result<Move, _> = "move x from 1337 to 9".parse();
            assert!(matches!(subject, Err(MoveParseError::ParseInt(_))));
        }
    }

    #[test]
    fn move_crates() {
        // [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let mut stacks = Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let moves = vec![Move(1, 2, 1), Move(3, 1, 3), Move(2, 2, 1), Move(1, 1, 2)];

        assert_eq!(
            stacks.execute_moves(&moves, MoveType::Part1),
            Ok(String::from("CMZ"))
        );
    }

    #[test]
    fn part1_works() {
        // arrange
        let (mut stacks, moves) = parse_problem(INPUT).expect("Failed to parse the problem");

        // act + assert
        let top_crates = stacks.execute_moves(&moves, MoveType::Part1);
        assert_eq!(top_crates, Ok(String::from("RNZLFZSJH")));
    }

    #[test]
    fn part2_works() {
        // arrange
        let (mut stacks, moves) = parse_problem(INPUT).expect("Failed to parse the problem");

        // act + assert
        let top_crates = stacks.execute_moves(&moves, MoveType::Part2);
        assert_eq!(top_crates, Ok(String::from("CNSFCGJSM")));
    }
}
