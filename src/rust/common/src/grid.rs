use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum NeighborsRowError {
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GridBoundsError {
    #[error("Row {0} is not in the grid which has {1} rows.")]
    MissingRow(usize, usize),
    #[error("Row {0} only has {1} columns, but column {2} is requested.")]
    MissingColumn(usize, usize, usize),
}

type NeighborsRowResult<T> = Result<T, NeighborsRowError>;
type NeighborsResult<T> = Result<T, GridBoundsError>;
type RowCol = (usize, usize);

pub trait Grid {
    fn in_grid(&self, point: &RowCol) -> Result<(), GridBoundsError>;
    fn neighbors_cell(&self, point: &RowCol) -> NeighborsRowResult<Vec<RowCol>>;
    fn neighbors_row(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> NeighborsRowResult<Vec<RowCol>>;
    fn neighbors_grid(&self, start: &RowCol, end: &RowCol) -> NeighborsResult<Vec<RowCol>>;
}

impl<T> Grid for Vec<Vec<T>> {
    fn in_grid(&self, point: &RowCol) -> Result<(), GridBoundsError> {
        if point.0 >= self.len() {
            Err(GridBoundsError::MissingRow(point.0, self.len()))
        } else if point.1 >= self[point.0].len() {
            Err(GridBoundsError::MissingColumn(
                point.0,
                self[point.0].len(),
                point.1,
            ))
        } else {
            Ok(())
        }
    }

    fn neighbors_cell(&self, point: &RowCol) -> NeighborsRowResult<Vec<RowCol>> {
        self.neighbors_row(point.0, point.1, point.1)
    }

    fn neighbors_row(
        &self,
        row: usize,
        start: usize,
        end: usize,
    ) -> NeighborsRowResult<Vec<RowCol>> {
        let mut neighbors = vec![];

        // Check that the range given is in the grid.
        if start > end {
            return Err(NeighborsRowError::StartGreaterThanEnd { start, end });
        } else if row >= self.len() {
            return Err(NeighborsRowError::MissingRow(row));
        } else if end >= self[row].len() {
            return Err(NeighborsRowError::OutOfBounds {
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

    fn neighbors_grid(&self, start: &RowCol, end: &RowCol) -> NeighborsResult<Vec<RowCol>> {
        let mut neighbors = vec![];

        // Check that the sub-grid given is in the grid.
        self.in_grid(start)?;
        self.in_grid(end)?;

        let start_row = start.0.min(end.0);
        let start_col = start.1.min(end.1);
        let start_col_expanded = start_col.saturating_sub(1);

        let end_row = start.0.max(end.0);
        let end_col = start.1.max(end.1);
        let end_col_expanded = end_col + 1;

        // Add the neighbors above if we're not on the first row
        if let Some(above) = start_row.checked_sub(1) {
            let end = end_col_expanded.min(self[above].len().saturating_sub(1));
            for col in start_col_expanded..=end {
                neighbors.push((above, col));
            }
        }

        // Add the neighbors below if we're not on the last row
        let below = end_row + 1;
        if below < self.len() {
            let end = end_col_expanded.min(self[below].len().saturating_sub(1));
            for col in start_col_expanded..=end {
                let point = (below, col);
                if self.in_grid(&point).is_ok() {
                    neighbors.push(point);
                }
            }
        }

        // Add the neighbors to the left if we're not on the first column
        if let Some(left) = start_col.checked_sub(1) {
            for row in start_row..=end_row {
                neighbors.push((row, left));
            }
        }

        // Add the neighbors to the right if we're not on the last column
        for row in start_row..=end_row {
            let point = (row, end_col_expanded);
            if self.in_grid(&point).is_ok() {
                neighbors.push(point);
            }
        }

        Ok(neighbors)
    }
}

#[cfg(test)]
mod tests_grid {
    use super::*;
    use test_case::test_case;

    fn grid<T>(rows: usize, cols: usize, i: T) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        let mut grid = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(i);
            }
            grid.push(row);
        }
        grid
    }

    #[test]
    fn in_grid_no_row() {
        let subject = grid(2, 2, 0_u8);
        let in_grid = subject.in_grid(&(2, 2));
        assert_eq!(in_grid, Err(GridBoundsError::MissingRow(2, 2)))
    }

    #[test]
    fn in_grid_no_col() {
        let subject = grid(2, 2, 0_u8);
        let in_grid = subject.in_grid(&(1, 2));
        assert_eq!(in_grid, Err(GridBoundsError::MissingColumn(1, 2, 2)))
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
    fn neighbors(row: usize, col: usize, expected: &[RowCol]) {
        let subject = grid(3, 5, '.');
        let neighbors = subject.neighbors_cell(&(row, col));
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
    fn neighbors_jagged(row: usize, col: usize, expected: &[RowCol]) {
        let subject = vec![
            vec!['x', 'x', 'x', '.'],
            vec!['x', '?'],
            vec!['x', 'x', 'x'],
        ];
        let neighbors = subject.neighbors_cell(&(row, col));
        assert_eq!(neighbors, Ok(expected.to_vec()))
    }

    #[test]
    fn neighbors_empty() {
        let subject: Vec<Vec<char>> = vec![];
        let neighbors = subject.neighbors_cell(&(5, 5));
        assert_eq!(neighbors, Err(NeighborsRowError::MissingRow(5)))
    }

    #[test]
    fn neighbors_start_greater_than_end() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_row(2, 4, 2);
        assert_eq!(
            neighbors,
            Err(NeighborsRowError::StartGreaterThanEnd { start: 4, end: 2 })
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
            Err(NeighborsRowError::OutOfBounds {
                row: 2,
                length: 5,
                start,
                end,
            })
        )
    }

    #[test]
    fn neighbors_grid_start_row_out_of_bounds() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_grid(&(5, 5), &(5, 5));
        assert_eq!(neighbors, Err(GridBoundsError::MissingRow(5, 5)))
    }

    #[test]
    fn neighbors_grid_start_col_out_of_bounds() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_grid(&(0, 5), &(5, 5));
        assert_eq!(neighbors, Err(GridBoundsError::MissingColumn(0, 5, 5)))
    }

    #[test]
    fn neighbors_grid_end_row_out_of_bounds() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_grid(&(0, 0), &(5, 5));
        assert_eq!(neighbors, Err(GridBoundsError::MissingRow(5, 5)))
    }

    #[test]
    fn neighbors_grid_end_col_out_of_bounds() {
        let subject = grid(5, 5, 0_u8);
        let neighbors = subject.neighbors_grid(&(0, 0), &(0, 5));
        assert_eq!(neighbors, Err(GridBoundsError::MissingColumn(0, 5, 5)))
    }

    /* 5x6 grid
        0 1 2 3 4 5
    0   ? . . x . .
    1   . . . x . .
    2   . . . x . .
    3   . . ? x . .
    4   x x x x . .
     */
    #[test_case((0, 0), (3, 2), &[(4, 0), (4, 1), (4, 2), (4, 3), (0, 3), (1, 3), (2, 3), (3, 3)])]
    /* 5x6 grid
        0 1 2 3 4 5
    0   x x x x x x
    1   x ? . . . .
    2   x . . . . .
    3   x . . . . ?
    4   x x x x x x
     */
    #[test_case((1, 1), (3, 5), &[(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
                                  (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5),
                                  (1, 0), (2, 0), (3, 0)])]
    /* 5x6 grid
        0 1 2 3 4 5
    0   . x x x x .
    1   . x ? . x .
    2   . x . ? x .
    3   . x x x x .
    4   . . . . . .
     */
    #[test_case((1, 2), (2, 3), &[(0, 1), (0, 2), (0, 3), (0, 4),
                                  (3, 1), (3, 2), (3, 3), (3, 4),
                                  (1, 1), (2, 1), (1, 4), (2, 4)])]
    fn neighbors_grid(start: RowCol, end: RowCol, expected: &[RowCol]) {
        let subject = grid(5, 6, 0_u8);
        let neighbors = subject.neighbors_grid(&start, &end);
        assert_eq!(neighbors, Ok(expected.to_vec()))
    }
}
