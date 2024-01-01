use thiserror::Error;

use common::grid::Grid;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SchematicError {}

#[derive(Debug)]
pub struct Schematic {
    pub data: Vec<Vec<char>>,
    length_x: usize,
    length_y: usize,
}

fn is_symbol(c: &char) -> bool {
    *c != '.' && c.is_ascii_punctuation()
}

enum LookingFor {
    Start,
    End,
}

impl Schematic {
    fn valid_part_number_location(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> Result<bool, String> {
        // Check if part number location is adjacent to a symbol

        Ok(self
            .data
            .neighbors_row(row, start, end)
            .map_err(|e| e.to_string())?
            .iter()
            .any(|point| is_symbol(&self.data[point.0][point.1])))
    }

    pub fn part_numbers(&self) -> Result<Vec<u64>, String> {
        let mut part_numbers = vec![];

        for (row, line) in self.data.iter().enumerate() {
            let mut start = 0;
            let mut end;
            let mut looking_for = LookingFor::Start;

            for (col, c) in line.iter().enumerate() {
                match (&looking_for, c.is_ascii_digit()) {
                    (LookingFor::Start, true) => {
                        start = col;
                        looking_for = LookingFor::End;
                    }
                    (LookingFor::Start, false) => continue,
                    (LookingFor::End, true) => continue,
                    (LookingFor::End, false) => {
                        end = col - 1;
                        looking_for = LookingFor::Start;

                        if let Ok(true) = self.valid_part_number_location(row, start, end) {
                            if let Ok(part_number) = self.part_number(row, start, end) {
                                part_numbers.push(part_number);
                            }
                        }
                    }
                }
            }

            // Check if we ended on a valid part number
            if let LookingFor::End = &looking_for {
                end = self.length_x - 1;
                if let Ok(true) = self.valid_part_number_location(row, start, end) {
                    if let Ok(part_number) = self.part_number(row, start, end) {
                        part_numbers.push(part_number);
                    }
                }
            }
        }

        Ok(part_numbers)
    }

    fn part_number(&self, row: usize, start: usize, end: usize) -> Result<u64, String> {
        let part_number = self
            .data
            .get(row)
            .and_then(|line| line.get(start..=end))
            .and_then(|chars| chars.iter().collect::<String>().parse().ok())
            .ok_or_else(|| format!("Failed to parse part number at row {}, col {}", row, end))?;

        Ok(part_number)
    }

    fn gear_locations(&self) -> Vec<(usize, usize)> {
        let mut gear_locations = vec![];

        for (row, line) in self.data.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c == '*' {
                    gear_locations.push((row, col));
                }
            }
        }

        gear_locations
    }
}

impl From<&str> for Schematic {
    fn from(s: &str) -> Self {
        let data = s
            .lines()
            .skip_while(|l| l.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let length_y = data.len();
        let length_x = data.get(0).unwrap_or(&vec![]).len();

        Self {
            data,
            length_x,
            length_y,
        }
    }
}

#[cfg(test)]
mod tests_y2023_engine {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = "
467..114..
...*......
..35..6339
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn gear_locations() {
        let schematic = Schematic::from(INPUT);
        let subject = schematic.gear_locations();
        assert_eq!(subject, vec![(1, 3), (4, 3), (8, 5)]);
    }

    #[test]
    fn part_numbers() {
        let schematic = Schematic::from(INPUT);
        let subject = schematic.part_numbers();
        assert_eq!(subject, Ok(vec![467, 35, 6339, 617, 592, 755, 664, 598]));
    }

    #[test]
    fn engine_parse() {
        let schematic = Schematic::from(INPUT);
        assert_eq!(schematic.data[4][0], '6');
        assert_eq!(schematic.data[8][3], '$');
        assert_eq!(schematic.length_x, 10);
        assert_eq!(schematic.length_y, 10);
    }

    #[test_case(0, 5, 7, false)]
    #[test_case(5, 7, 8, false)]
    #[test_case(0, 0, 2, true)]
    #[test_case(2, 2, 3, true)]
    #[test_case(2, 6, 9, true)]
    #[test_case(4, 0, 2, true)]
    #[test_case(6, 2, 4, true)]
    #[test_case(7, 6, 8, true)]
    fn valid_part_number_location(row: usize, start: usize, end: usize, expected: bool) {
        let schematic = Schematic::from(INPUT);
        assert_eq!(
            schematic.valid_part_number_location(row, start, end),
            Ok(expected)
        );
    }
}
