use std::error;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NeighborsError {
    #[error("The requested row ({0}) is not in the grid.")]
    MissingRow(usize),
    #[error(
        "The requested range ({start}..={end}) is out of bounds for row {row} of length {length}."
    )]
    OutOfBounds {
        row: usize,
        length: usize,
        start: usize,
        end: usize,
    },
    #[error("The requested range ({start}..={end}) is invalid. Start must be less than or equal to end.")]
    StartGreaterThanEnd { start: usize, end: usize },
}

type NeighborsResult<T> = Result<T, NeighborsError>;

pub trait Grid {
    fn neighbors_cell(&self, row: usize, col: usize) -> NeighborsResult<Vec<(usize, usize)>>;
    fn neighbors_row(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> NeighborsResult<Vec<(usize, usize)>>;
}

impl<T> Grid for Vec<Vec<T>> {
    fn neighbors_cell(&self, row: usize, col: usize) -> NeighborsResult<Vec<(usize, usize)>> {
        self.neighbors_row(row, col, col)
    }

    fn neighbors_row(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> NeighborsResult<Vec<(usize, usize)>> {
        let mut neighbors = vec![];

        // Check that the range given is in the grid.
        if start > end {
            return Err(NeighborsError::StartGreaterThanEnd { start, end });
        } else if row >= self.len() {
            return Err(NeighborsError::MissingRow(row));
        } else if end >= self[row].len() {
            return Err(NeighborsError::OutOfBounds {
                row,
                length: self[row].len(),
                start,
                end,
            });
        }

        let start_expanded = start.saturating_sub(1);
        let end_expanded = end + 1;

        // Add the neighbors above if we're not on the first row
        if let Some(above) = row.checked_sub(1) {
            let end = end_expanded.min(self[above].len().saturating_sub(1));
            for col in start_expanded..=end {
                neighbors.push((above, col));
            }
        }

        // Add the neighbors below if we're not on the last row
        let below = row + 1;
        if below < self.len() {
            let end = end_expanded.min(self[below].len().saturating_sub(1));
            for col in start_expanded..=end {
                neighbors.push((below, col));
            }
        }

        // Add the neighbors to the left if we're not on the first column
        if let Some(left) = start.checked_sub(1) {
            neighbors.push((row, left));
        }
        // Add the neighbors to the right if we're not on the last column
        if end_expanded < self[row].len() {
            neighbors.push((row, end_expanded));
        }

        Ok(neighbors)
    }
}

#[cfg(test)]
mod tests_grid {
    use super::*;
    use test_case::test_case;

    fn grid<T>(x: usize, y: usize, i: T) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut grid = Vec::with_capacity(y);
        for _ in 0..y {
            let mut row = Vec::with_capacity(x);
            for _ in 0..x {
                row.push(i);
            }
            grid.push(row);
        }
        grid
    }

    /*
      0 1 2 3 4
    0 . x ? x .
    1 . x x x .
    2 . . . . .
     */
    #[test_case(0, 2, &[(1, 1), (1, 2), (1, 3), (0, 1), (0, 3)])]
    /*
      0 1 2 3 4
    0 . x x x .
    1 . x ? x .
    2 . x x x .
     */
    #[test_case(1, 2, &[(0, 1), (0, 2), (0, 3), (2, 1), (2, 2), (2, 3), (1, 1), (1, 3)])]
    /*
      0 1 2 3 4
    0 . . . . .
    1 . . . x x
    2 . . . x ?
     */
    #[test_case(2, 4, &[(1, 3), (1, 4), (2, 3)])]
    /*
      0 1 2 3 4
    0 ? x . . .
    1 x x . . .
    2 . . . . .
     */
    #[test_case(0, 0, &[(1, 0), (1, 1), (0, 1)])]
    fn neighbors(row: usize, col: usize, expected: &[(usize, usize)]) {
        let subject = grid(5, 3, '.');
        let neighbors = subject.neighbors_cell(row, col);
        assert_eq!(neighbors, Ok(expected.to_vec()))
    }

    /*
      0 1 2 3
    0 x x x .
    1 x ?
    2 x x x
     */
    #[test_case(1, 1, &[(0, 0), (0, 1), (0, 2), (2, 0), (2, 1), (2, 2), (1, 0)])]
    /*
      0 1 2 3
    0 x x . .
    1 ? x
    2 x x .
     */
    #[test_case(1, 0, &[(0, 0), (0, 1), (2, 0), (2, 1), (1, 1)])]
    fn neighbors_jagged(row: usize, col: usize, expected: &[(usize, usize)]) {
        let subject = vec![
            vec!['x', 'x', 'x', '.'],
            vec!['x', '?'],
            vec!['x', 'x', 'x'],
        ];
        let neighbors = subject.neighbors_cell(row, col);
        assert_eq!(neighbors, Ok(expected.to_vec()))
    }

    #[test]
    fn neighbors_empty() {
        let subject: Vec<Vec<char>> = vec![];
        let neighbors = subject.neighbors_cell(5, 5);
        assert_eq!(neighbors, Err(NeighborsError::MissingRow(5)))
    }

    #[test]
    fn neighbors_start_greater_than_end() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_row(2, 4, 2);
        assert_eq!(
            neighbors,
            Err(NeighborsError::StartGreaterThanEnd { start: 4, end: 2 })
        )
    }

    #[test_case(0, 5)]
    #[test_case(5, 5)]
    #[test_case(2, 10_000)]
    fn neighbors_range_out_of_bounds(start: usize, end: usize) {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_row(2, start, end);
        assert_eq!(
            neighbors,
            Err(NeighborsError::OutOfBounds {
                row: 2,
                length: 5,
                start,
                end,
            })
        )
    }
}
