use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

struct MapRange {
    from: u64,
    to: u64,
    length: u64,
}

impl MapRange {
    pub fn contains(&self, value: u64) -> bool {
        value >= self.from && value <= self.from + self.length
    }
}

struct Map {
    source: String,
    target: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum MapParseError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("invalid map format")]
    InvalidFormat,
}

impl FromStr for Map {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut id = lines
            .next()
            .ok_or(MapParseError::InvalidFormat)?
            .trim_end_matches(" map:")
            .split('-');
        let source = id.next().ok_or(MapParseError::InvalidFormat)?;
        let target = id.nth(1).ok_or(MapParseError::InvalidFormat)?;

        let mut ranges = Vec::new();
        for line in lines {
            let mut parts = line.split_whitespace();
            let to = parts.next().ok_or(MapParseError::InvalidFormat)?.parse()?;
            let from = parts.next().ok_or(MapParseError::InvalidFormat)?.parse()?;
            let length = parts.next().ok_or(MapParseError::InvalidFormat)?.parse()?;
            ranges.push(MapRange { to, from, length });
        }

        Ok(Map {
            source: source.into(),
            target: target.into(),
            ranges,
        })
    }
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;

    const INPUT: &str = include_str!("d05.txt");
    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse_seeds() {
        assert_eq!(parse_seeds("seeds: 79 14 55 13"), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_map() {
        let map = "fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4";

        let subject: Map = map.parse().unwrap();
        let expected = Map {
            source: "fertilizer".to_string(),
            target: "water".to_string(),
            ranges: vec![
                MapRange {
                    from: 49,
                    to: 53,
                    length: 8,
                },
                MapRange {
                    from: 0,
                    to: 11,
                    length: 42,
                },
                MapRange {
                    from: 42,
                    to: 0,
                    length: 7,
                },
                MapRange {
                    from: 57,
                    to: 7,
                    length: 4,
                },
            ],
        };
    }

    #[test]
    fn test_map_rang_contains() {
        let range = MapRange {
            from: 0,
            to: 10,
            length: 10,
        };
        for i in 0..10 {
            assert!(range.contains(i))
        }
        assert!(!range.contains(10));
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
