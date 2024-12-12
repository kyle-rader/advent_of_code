use std::str::FromStr;

use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, RangeParseError> {
    let mut c = 0;
    for line in input.lines() {
        let mut parts = line.split(',');
        let Some(a) = parts.next() else {
            return Err(RangeParseError::Format);
        };
        let Some(b) = parts.next() else {
            return Err(RangeParseError::Format);
        };
        let a = a.parse::<Range>()?;
        let b = b.parse::<Range>()?;
        if a.contains(&b) || b.contains(&a) {
            c += 1;
        }
    }
    Ok(c)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, RangeParseError> {
    let mut c = 0;
    for line_result in input.lines().map(parse_line) {
        let Ok((a, b)) = line_result else {
            return line_result.map(|_| 0);
        };
        if a.overlaps(&b) {
            c += 1;
        }
    }
    Ok(c)
}

type RangePair = (Range, Range);
fn parse_line(line: &str) -> Result<RangePair, RangeParseError> {
    let mut parts = line.split(',');
    let Some(a) = parts.next() else {
        return Err(RangeParseError::Format);
    };
    let Some(b) = parts.next() else {
        return Err(RangeParseError::Format);
    };
    let a = a.parse::<Range>()?;
    let b = b.parse::<Range>()?;
    Ok((a, b))
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    /// Check if other is contained by self.
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Check if self and other overlap at all.
    pub fn overlaps(&self, other: &Self) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
            || (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
enum RangeParseError {
    #[error("Missing start id")]
    MissingStart,
    #[error("Missing end id")]
    MissingEnd,
    #[error("A Range has format n-m with only 2 values and a single dash")]
    Format,
    #[error(transparent)]
    Invalid(#[from] std::num::ParseIntError),
}

impl FromStr for Range {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-').filter(|s| !s.is_empty());
        let start: u32 = parts.next().ok_or(RangeParseError::MissingStart)?.parse()?;
        let end: u32 = parts.next().ok_or(RangeParseError::MissingEnd)?.parse()?;

        if parts.next().is_some() {
            return Err(RangeParseError::Format);
        }

        Ok(Range { start, end })
    }
}

#[allow(dead_code)]
const INPUT: &str = include_str!("d04.txt");

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    #[test]
    fn range_parse_empty() {
        assert_eq!("".parse::<Range>(), Err(RangeParseError::MissingStart));
    }

    #[test]
    fn range_parse_missing_end() {
        assert_eq!("1".parse::<Range>(), Err(RangeParseError::MissingEnd));
    }

    #[test]
    fn range_parse_invalid() {
        let subject = "1-a".parse::<Range>();
        assert!(matches!(subject, Err(RangeParseError::Invalid(_))));
    }

    #[test]
    fn range_parse_too_many_things() {
        let subject = "1-4-99".parse::<Range>();
        assert_eq!(subject, Err(RangeParseError::Format));
    }

    #[test]
    fn range_parse() {
        assert_eq!("1-4".parse(), Ok(Range { start: 1, end: 4 }));
    }

    #[test]
    fn range_contains() {
        let small = Range { start: 2, end: 4 };
        let med = Range { start: 1, end: 10 };
        let big = Range { start: 0, end: 100 };
        assert!(big.contains(&med));
        assert!(big.contains(&small));
        assert!(med.contains(&small));
        assert!(!small.contains(&med));
        assert!(!small.contains(&big));
        assert!(!med.contains(&big));
    }

    #[test]
    fn range_overlaps_right() {
        // a = ...456...
        // b = ....5678.
        let a = Range { start: 4, end: 6 };
        let b = Range { start: 5, end: 8 };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn range_overlaps_right_no_overlap() {
        // a = ..345....
        // b = ......789
        let a = Range { start: 3, end: 5 };
        let b = Range { start: 7, end: 9 };
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn range_overlaps_left() {
        // a = ...456...
        // b = .2345....
        let a = Range { start: 4, end: 6 };
        let b = Range { start: 1, end: 5 };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn range_overlaps_left_no_overlap() {
        // a = .....678.
        // b = .234.....
        let a = Range { start: 6, end: 8 };
        let b = Range { start: 2, end: 4 };
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(532));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(854));
    }
}
