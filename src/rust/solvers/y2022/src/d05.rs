use std::str::FromStr;

use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(Debug, Error, PartialEq, Eq)]
enum StackParseError {
    #[error("No crate rows found!")]
    NoCrates,
    #[error("Ran into a row with a different number of stacks than the first row")]
    Misaligned,
    #[error("An unexpected number of characters were found in a row")]
    UnexpectedNumberOfChars,
}

#[derive(Debug, PartialEq, Eq)]
struct Stacks(Vec<Vec<char>>);

impl FromStr for Stacks {
    type Err = StackParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Chop off the "id row" because we don't need it
        let rows: Vec<&str> = s.lines().collect();
        let last_row = rows.len().checked_sub(1).ok_or(StackParseError::NoCrates)?;
        let rows = &rows[0..last_row];

        // Init the stacks
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let row_length = rows.iter().nth(0).ok_or(StackParseError::NoCrates)?.len();
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

            for i in 0..stack_count {
                //  x :0123456789
                // row:[Z] [M] [P]
                // i  : 0   1   2
                let j = i * 4 + 1;
                match row[j] {
                    ' ' => {}
                    c => stacks[i].push(c),
                }
            }
        }

        Ok(Stacks(stacks))
    }
}

impl Stacks {
    fn get_mut(&mut self, i: usize) -> Option<&mut Vec<char>> {
        self.0.get_mut(i)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move(u32, usize, usize);

#[derive(Debug, Error, PartialEq, Eq)]
enum MoveParseError {
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
enum CrateTrackerParseError {
    #[error("Could not split starting state and moves from initial input")]
    Split,
    #[error(transparent)]
    Stack(#[from] StackParseError),
    #[error(transparent)]
    Move(#[from] MoveParseError),
}

#[derive(Debug, Error, PartialEq, Eq)]
enum MoveCratesError {
    #[error("{0}")]
    MissingStack(String),
    #[error("Out of crates in stack {0}")]
    OutOfCrates(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct CrateTracker {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl CrateTracker {
    pub fn move_crates(&mut self) -> Result<Vec<char>, MoveCratesError> {
        for Move(amount, from_idx, to_idx) in self.moves.iter() {
            for _ in 0..*amount {
                let from = self.stacks.get_mut(*from_idx - 1).ok_or_else(|| {
                    MoveCratesError::MissingStack(format!("from stack {from_idx} is not here"))
                })?;

                // un-borrow stacks, so we can borrow again to get the 'to' stack.
                let from = from
                    .pop()
                    .ok_or(MoveCratesError::OutOfCrates(*from_idx - 1))?;

                let to = self.stacks.get_mut(*to_idx - 1).ok_or_else(|| {
                    MoveCratesError::MissingStack(format!("to stack {to_idx} is not here"))
                })?;

                to.push(from);
            }
        }

        Ok(self
            .stacks
            .0
            .iter()
            .filter_map(|s| s.last())
            .map(|c| *c)
            .collect())
    }
}

impl FromStr for CrateTracker {
    type Err = CrateTrackerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split("\n\n").collect::<Vec<&str>>();
        let [stacks, moves] = &splits[..] else { return Err(CrateTrackerParseError::Split); };
        let stacks: Stacks = stacks.parse()?;
        let moves: Vec<Move> = moves
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Move>, _>>()?;
        Ok(CrateTracker { stacks, moves })
    }
}

#[cfg(test)]
mod tests {
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
        let mut subject = CrateTracker {
            stacks: Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]),
            moves: vec![Move(1, 2, 1), Move(3, 1, 3), Move(2, 2, 1), Move(1, 1, 2)],
        };

        assert_eq!(subject.move_crates(), Ok(vec!['C', 'M', 'Z']));
    }

    type CrateTrackerTestResult = Result<(), CrateTrackerParseError>;

    #[test]
    fn part1_works() {
        // arrange
        let mut crate_tracker: CrateTracker =
            INPUT.parse().expect("the laws of physics no longer apply");

        // act + assert
        let top_crates = crate_tracker.move_crates();
        assert_eq!(
            top_crates,
            Ok(vec!['R', 'N', 'Z', 'L', 'F', 'Z', 'S', 'J', 'H'])
        );
    }

    #[test]
    #[ignore]
    fn part2_works() -> CrateTrackerTestResult {
        assert_eq!(part2(INPUT), Ok(42));
        Ok(())
    }
}

#[cfg(test)]
const INPUT: &str = "[S]                 [T] [Q]        
[L]             [B] [M] [P]     [T]
[F]     [S]     [Z] [N] [S]     [R]
[Z] [R] [N]     [R] [D] [F]     [V]
[D] [Z] [H] [J] [W] [G] [W]     [G]
[B] [M] [C] [F] [H] [Z] [N] [R] [L]
[R] [B] [L] [C] [G] [J] [L] [Z] [C]
[H] [T] [Z] [S] [P] [V] [G] [M] [M]
 1   2   3   4   5   6   7   8   9 

move 6 from 1 to 7
move 2 from 2 to 4
move 2 from 7 to 4
move 6 from 4 to 3
move 1 from 5 to 1
move 3 from 8 to 3
move 15 from 3 to 4
move 6 from 5 to 9
move 14 from 4 to 2
move 3 from 2 to 7
move 1 from 2 to 7
move 9 from 9 to 1
move 3 from 2 to 1
move 7 from 6 to 7
move 1 from 6 to 8
move 2 from 9 to 1
move 9 from 2 to 3
move 8 from 3 to 9
move 1 from 1 to 4
move 1 from 8 to 6
move 1 from 6 to 2
move 5 from 9 to 8
move 2 from 9 to 1
move 1 from 4 to 2
move 17 from 1 to 9
move 1 from 3 to 1
move 3 from 2 to 3
move 2 from 4 to 5
move 12 from 7 to 3
move 16 from 9 to 2
move 5 from 7 to 5
move 2 from 1 to 2
move 1 from 3 to 6
move 1 from 4 to 6
move 1 from 7 to 3
move 1 from 6 to 3
move 7 from 3 to 4
move 5 from 8 to 3
move 1 from 6 to 7
move 7 from 3 to 4
move 6 from 3 to 1
move 2 from 4 to 8
move 1 from 5 to 2
move 10 from 4 to 5
move 3 from 5 to 2
move 2 from 8 to 9
move 5 from 2 to 8
move 1 from 3 to 5
move 2 from 5 to 8
move 12 from 5 to 7
move 1 from 4 to 2
move 5 from 9 to 4
move 1 from 2 to 5
move 6 from 1 to 3
move 6 from 3 to 5
move 10 from 7 to 4
move 2 from 7 to 3
move 4 from 7 to 6
move 1 from 9 to 5
move 12 from 2 to 1
move 1 from 8 to 7
move 3 from 7 to 4
move 4 from 4 to 8
move 7 from 5 to 3
move 1 from 2 to 4
move 10 from 1 to 5
move 2 from 1 to 2
move 4 from 6 to 7
move 8 from 8 to 3
move 5 from 4 to 9
move 12 from 3 to 8
move 4 from 3 to 8
move 2 from 9 to 2
move 3 from 5 to 4
move 1 from 3 to 5
move 1 from 7 to 6
move 14 from 4 to 6
move 6 from 5 to 9
move 8 from 2 to 8
move 3 from 5 to 7
move 21 from 8 to 4
move 16 from 4 to 9
move 8 from 6 to 2
move 4 from 6 to 1
move 1 from 4 to 6
move 2 from 4 to 8
move 3 from 1 to 8
move 2 from 4 to 6
move 1 from 6 to 2
move 3 from 8 to 4
move 2 from 2 to 5
move 2 from 5 to 7
move 1 from 8 to 9
move 1 from 4 to 9
move 1 from 1 to 6
move 3 from 6 to 3
move 3 from 2 to 3
move 1 from 4 to 6
move 3 from 6 to 7
move 10 from 9 to 7
move 1 from 4 to 7
move 6 from 8 to 3
move 1 from 6 to 8
move 2 from 2 to 5
move 1 from 2 to 1
move 1 from 8 to 9
move 1 from 2 to 8
move 1 from 1 to 9
move 7 from 9 to 1
move 1 from 8 to 5
move 7 from 1 to 7
move 3 from 5 to 8
move 3 from 7 to 2
move 1 from 8 to 4
move 1 from 2 to 4
move 2 from 4 to 6
move 5 from 3 to 1
move 9 from 7 to 2
move 6 from 3 to 8
move 8 from 2 to 7
move 2 from 6 to 4
move 2 from 1 to 7
move 2 from 1 to 4
move 24 from 7 to 4
move 4 from 8 to 9
move 2 from 7 to 5
move 1 from 5 to 2
move 1 from 3 to 8
move 4 from 2 to 8
move 13 from 9 to 2
move 2 from 8 to 6
move 3 from 9 to 6
move 26 from 4 to 2
move 1 from 5 to 7
move 2 from 6 to 2
move 2 from 4 to 1
move 7 from 2 to 1
move 15 from 2 to 6
move 8 from 2 to 8
move 4 from 6 to 8
move 9 from 2 to 9
move 13 from 6 to 7
move 6 from 1 to 9
move 2 from 2 to 4
move 4 from 1 to 6
move 3 from 8 to 3
move 1 from 4 to 9
move 2 from 6 to 7
move 1 from 4 to 3
move 3 from 3 to 2
move 14 from 7 to 4
move 5 from 9 to 5
move 9 from 8 to 5
move 7 from 9 to 6
move 2 from 5 to 6
move 2 from 9 to 2
move 10 from 5 to 1
move 1 from 3 to 1
move 2 from 8 to 1
move 1 from 9 to 2
move 1 from 7 to 5
move 4 from 2 to 1
move 1 from 9 to 8
move 3 from 4 to 1
move 1 from 8 to 6
move 12 from 1 to 5
move 1 from 1 to 6
move 1 from 7 to 5
move 4 from 6 to 9
move 2 from 2 to 4
move 1 from 9 to 6
move 1 from 1 to 5
move 2 from 9 to 7
move 10 from 6 to 5
move 1 from 6 to 7
move 20 from 5 to 1
move 1 from 7 to 9
move 2 from 9 to 1
move 3 from 5 to 1
move 2 from 8 to 4
move 2 from 8 to 7
move 1 from 5 to 9
move 1 from 8 to 4
move 22 from 1 to 7
move 5 from 4 to 8
move 1 from 5 to 9
move 19 from 7 to 4
move 2 from 9 to 1
move 1 from 5 to 9
move 10 from 1 to 8
move 1 from 9 to 1
move 1 from 8 to 3
move 8 from 4 to 7
move 1 from 5 to 6
move 3 from 4 to 5
move 1 from 5 to 9
move 11 from 7 to 4
move 4 from 4 to 9
move 1 from 6 to 2
move 1 from 3 to 9
move 5 from 9 to 4
move 5 from 7 to 9
move 23 from 4 to 2
move 17 from 2 to 7
move 2 from 2 to 8
move 4 from 4 to 7
move 1 from 4 to 5
move 2 from 5 to 2
move 5 from 8 to 9
move 5 from 2 to 7
move 9 from 7 to 5
move 11 from 9 to 2
move 1 from 4 to 3
move 5 from 8 to 7
move 3 from 8 to 5
move 2 from 1 to 3
move 2 from 3 to 9
move 1 from 5 to 8
move 5 from 7 to 5
move 15 from 5 to 4
move 2 from 8 to 1
move 2 from 5 to 1
move 4 from 4 to 1
move 1 from 8 to 7
move 8 from 2 to 1
move 4 from 2 to 8
move 2 from 7 to 4
move 5 from 8 to 6
move 5 from 7 to 9
move 4 from 6 to 5
move 7 from 4 to 8
move 1 from 6 to 1
move 1 from 3 to 1
move 2 from 5 to 1
move 7 from 1 to 5
move 5 from 1 to 3
move 4 from 7 to 9
move 4 from 3 to 9
move 2 from 9 to 7
move 6 from 9 to 2
move 1 from 4 to 1
move 1 from 3 to 5
move 1 from 2 to 5
move 5 from 9 to 4
move 4 from 4 to 6
move 1 from 8 to 9
move 8 from 4 to 3
move 7 from 7 to 3
move 5 from 1 to 3
move 11 from 5 to 9
move 1 from 7 to 6
move 2 from 3 to 5
move 1 from 3 to 1
move 3 from 6 to 2
move 2 from 5 to 1
move 2 from 1 to 2
move 3 from 1 to 5
move 5 from 9 to 2
move 2 from 6 to 8
move 2 from 3 to 8
move 4 from 9 to 7
move 3 from 5 to 2
move 2 from 1 to 8
move 1 from 9 to 8
move 1 from 9 to 2
move 4 from 7 to 9
move 11 from 8 to 7
move 1 from 8 to 2
move 6 from 9 to 7
move 3 from 7 to 1
move 13 from 2 to 7
move 24 from 7 to 1
move 2 from 2 to 6
move 1 from 8 to 3
move 1 from 9 to 3
move 5 from 2 to 4
move 1 from 2 to 5
move 1 from 6 to 2
move 1 from 6 to 3
move 1 from 2 to 4
move 3 from 7 to 3
move 2 from 1 to 7
move 2 from 3 to 8
move 2 from 7 to 8
move 9 from 3 to 2
move 3 from 4 to 8
move 1 from 5 to 1
move 9 from 2 to 1
move 3 from 4 to 9
move 1 from 7 to 8
move 6 from 3 to 9
move 2 from 1 to 5
move 15 from 1 to 3
move 13 from 3 to 9
move 11 from 1 to 4
move 5 from 4 to 1
move 6 from 3 to 6
move 4 from 4 to 8
move 6 from 1 to 4
move 1 from 5 to 2
move 1 from 2 to 1
move 3 from 4 to 2
move 2 from 8 to 5
move 2 from 4 to 2
move 9 from 9 to 3
move 9 from 3 to 5
move 2 from 9 to 4
move 5 from 2 to 6
move 1 from 1 to 8
move 1 from 4 to 1
move 10 from 9 to 2
move 9 from 2 to 4
move 10 from 4 to 1
move 3 from 1 to 3
move 4 from 1 to 2
move 5 from 2 to 4
move 2 from 5 to 2
move 4 from 1 to 7
move 10 from 5 to 4
move 2 from 2 to 4
move 1 from 9 to 2
move 2 from 3 to 5
move 1 from 3 to 5
move 3 from 6 to 7
move 8 from 4 to 9
move 6 from 6 to 1
move 4 from 9 to 5
move 2 from 9 to 1
move 1 from 2 to 6
move 6 from 5 to 2
move 3 from 7 to 9
move 4 from 8 to 2
move 1 from 7 to 9
move 1 from 5 to 3
move 2 from 7 to 4
move 1 from 7 to 1
move 14 from 1 to 9
move 1 from 1 to 9
move 1 from 3 to 8
move 3 from 2 to 5
move 2 from 4 to 2
move 6 from 8 to 1
move 1 from 2 to 1
move 5 from 1 to 9
move 1 from 1 to 7
move 2 from 8 to 5
move 1 from 5 to 4
move 1 from 6 to 1
move 8 from 2 to 7
move 2 from 6 to 1
move 9 from 9 to 5
move 11 from 4 to 8
move 4 from 7 to 4
move 6 from 4 to 6
move 1 from 7 to 4
move 6 from 6 to 7
move 1 from 5 to 9
move 6 from 8 to 9
move 8 from 9 to 5
move 1 from 4 to 5
move 15 from 9 to 3
move 3 from 1 to 4
move 6 from 7 to 2
move 3 from 4 to 9
move 2 from 7 to 3
move 1 from 7 to 3
move 1 from 7 to 2
move 2 from 8 to 1
move 3 from 8 to 5
move 2 from 1 to 7
move 8 from 3 to 6
move 3 from 6 to 5
move 1 from 6 to 1
move 10 from 5 to 7
move 6 from 5 to 4
move 4 from 2 to 4
move 6 from 5 to 1
move 6 from 1 to 8
move 2 from 9 to 2
move 2 from 9 to 7
move 6 from 3 to 7
move 1 from 3 to 5
move 1 from 1 to 9
move 2 from 8 to 1
move 2 from 5 to 4
move 3 from 3 to 7
move 10 from 4 to 6
move 1 from 9 to 7
move 12 from 7 to 3
move 12 from 3 to 8
move 2 from 1 to 5
move 1 from 1 to 3
move 13 from 8 to 1
move 7 from 7 to 1
move 13 from 6 to 9
move 1 from 7 to 4
move 6 from 5 to 3
move 3 from 4 to 3
move 6 from 3 to 1
move 10 from 9 to 4
move 2 from 7 to 6
move 8 from 1 to 9
move 3 from 2 to 9
move 1 from 3 to 5
move 1 from 3 to 5
move 1 from 1 to 4
move 6 from 9 to 3
move 2 from 6 to 7
move 4 from 9 to 5
move 4 from 1 to 6
move 1 from 2 to 4
move 6 from 1 to 4
move 3 from 9 to 3
move 3 from 6 to 8
move 3 from 8 to 7
move 5 from 5 to 1
move 1 from 3 to 9
move 1 from 9 to 5
move 1 from 3 to 2
move 2 from 5 to 1
move 1 from 6 to 9
move 1 from 6 to 3
move 2 from 9 to 7
move 2 from 8 to 1
move 1 from 3 to 2
move 1 from 2 to 5
move 1 from 7 to 1
move 7 from 7 to 9
move 12 from 1 to 9
move 1 from 5 to 2
move 1 from 7 to 1
move 13 from 4 to 7
move 1 from 9 to 4
move 5 from 7 to 3
move 4 from 9 to 1
move 8 from 7 to 9
move 3 from 2 to 3
move 4 from 3 to 7
move 5 from 4 to 6
move 3 from 9 to 4
move 10 from 1 to 5
move 3 from 4 to 7
move 16 from 9 to 2
move 3 from 9 to 2
move 6 from 5 to 3
move 4 from 6 to 2
move 1 from 4 to 6
move 2 from 6 to 8
move 1 from 5 to 2
move 1 from 5 to 8
move 7 from 7 to 2
move 16 from 2 to 1
move 1 from 5 to 1
move 10 from 2 to 8
move 14 from 8 to 5
move 2 from 2 to 6
move 1 from 2 to 5
move 2 from 2 to 1
move 8 from 1 to 7
move 4 from 1 to 7
move 2 from 1 to 7
move 5 from 3 to 2
move 1 from 1 to 6
move 2 from 2 to 5
move 4 from 1 to 7
move 1 from 2 to 8
move 1 from 2 to 8
move 3 from 6 to 7
move 10 from 7 to 5
move 1 from 2 to 8
move 27 from 5 to 9
move 1 from 5 to 6
move 1 from 6 to 4
move 1 from 4 to 3
move 3 from 3 to 7
move 4 from 3 to 6
move 2 from 6 to 4
move 3 from 8 to 1
move 2 from 6 to 1
move 12 from 7 to 8
move 2 from 3 to 9
move 1 from 9 to 2
move 1 from 2 to 8
move 2 from 1 to 2
move 6 from 3 to 8
move 1 from 7 to 4
move 15 from 9 to 5
move 7 from 9 to 4
move 1 from 2 to 1
move 16 from 8 to 2
move 8 from 5 to 2
move 24 from 2 to 9
move 3 from 1 to 2
move 24 from 9 to 1
move 5 from 5 to 9
move 3 from 4 to 1
move 1 from 7 to 6
move 1 from 6 to 3
move 1 from 3 to 2
move 3 from 2 to 3
move 1 from 5 to 6
move 1 from 2 to 7
";
