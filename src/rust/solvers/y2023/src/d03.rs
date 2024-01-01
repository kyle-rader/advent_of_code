use std::collections::HashMap;

use common::grid::Grid;

use self::engine::{PartNumber, PartNumberError, Schematic};

mod engine;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, PartNumberError> {
    let schematic = Schematic::from(input);
    Ok(schematic.part_numbers()?.iter().map(|part| part.id).sum())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, PartNumberError> {
    let mut gears: HashMap<(usize, usize), Vec<PartNumber>> = HashMap::new();
    let schematic = Schematic::from(input);
    for part in schematic.part_numbers()? {
        let PartNumber {
            row, start, end, ..
        } = part;
        schematic
            .data
            .neighbors_row(row, start, end)?
            .iter()
            .filter(|p| schematic.data[p.0][p.1] == '*')
            .for_each(|p| gears.entry(*p).or_default().push(part));
    }

    let gears = gears;

    Ok(gears
        .iter()
        .filter_map(|(_gear, parts)| {
            if parts.len() == 2 {
                Some(parts[0].id * parts[1].id)
            } else {
                None
            }
        })
        .sum())
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;

    const INPUT: &str = include_str!("d03.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(530495));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
