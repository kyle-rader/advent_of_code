use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GridBoundsError {
    #[error("Row {0} is not in the grid which has {1} rows.")]
    MissingRow(usize, usize),
    #[error("Row {0} only has {1} columns, but column {2} is requested.")]
    MissingColumn(usize, usize, usize),
}

type NeighborsResult<T> = Result<T, GridBoundsError>;
type RowCol = (usize, usize);

pub trait Grid<T> {
    fn in_grid(&self, point: &RowCol) -> Result<(), GridBoundsError>;
    fn neighbors_cell(&self, point: &RowCol) -> NeighborsResult<Vec<RowCol>>;
    fn neighbors_row(&self, row: usize, start: usize, end: usize) -> NeighborsResult<Vec<RowCol>>;
    fn neighbors_grid(&self, start: &RowCol, end: &RowCol) -> NeighborsResult<Vec<RowCol>>;
    fn words(&self, point: &RowCol, length: usize) -> Vec<Vec<T>>;
}

impl<T> Grid<T> for Vec<Vec<T>>
where
    T: Copy,
{
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

    fn neighbors_cell(&self, point: &RowCol) -> NeighborsResult<Vec<RowCol>> {
        self.neighbors_grid(point, point)
    }

    fn neighbors_row(&self, row: usize, start: usize, end: usize) -> NeighborsResult<Vec<RowCol>> {
        self.neighbors_grid(&(row, start), &(row, end))
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

    fn words(&self, point: &RowCol, length: usize) -> Vec<Vec<T>> {
        let mut words = vec![];
        let row = point.0;
        let col = point.1;
        let at_least = length - 1;

        if row >= self.len() || col >= self[row].len() {
            return words;
        }

        // Down
        if row + length <= self.len() {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row + i][col]);
            }
            words.push(word);
        }

        // Left
        if col >= at_least {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row][col - i]);
            }
            words.push(word);
        }

        // Right
        if col + length <= self[row].len() {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row][col + i]);
            }
            words.push(word);
        }

        // Up
        if row >= at_least {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row - i][col]);
            }
            words.push(word);
        }

        // Down-right
        if row + length <= self.len() && col + length <= self[row].len() {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row + i][col + i]);
            }
            words.push(word);
        }

        // Down-left
        if row + length <= self.len() && col >= at_least {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row + i][col - i]);
            }
            words.push(word);
        }

        // Up-right
        if row >= at_least && col + length <= self[row].len() {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row - i][col + i]);
            }
            words.push(word);
        }

        // Up-left
        if row >= at_least && col >= at_least {
            let mut word = vec![];
            for i in 0..length {
                word.push(self[row - i][col - i]);
            }
            words.push(word);
        }

        words
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
        let neighbors = subject.neighbors_cell(&(1, 1));
        assert_eq!(neighbors, Err(GridBoundsError::MissingRow(1, 0)))
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

    fn word_grid() -> Vec<Vec<char>> {
        vec![
            vec!['a', 'b', 'c', 'd', '1'],
            vec!['e', 'f', 'g', 'h', '2'],
            vec!['i', 'j', 'k', 'l', '3'],
            vec!['m', 'n', 'o', 'p', '4'],
            vec!['q', 'r', 's', 't', '5'],
        ]
    }

    #[test]
    fn grid_words_from_0_0() {
        let subject = word_grid();

        let words = subject.words(&(0, 0), 3);

        assert_eq!(
            words,
            vec![
                vec!['a', 'e', 'i'],
                vec!['a', 'b', 'c'],
                vec!['a', 'f', 'k'],
            ]
        );
    }

    #[test]
    fn grid_words_from_0_4() {
        let subject = word_grid();

        let words = subject.words(&(0, 4), 3);

        assert_eq!(
            words,
            vec![
                vec!['1', '2', '3'],
                vec!['1', 'd', 'c'],
                vec!['1', 'h', 'k'],
            ]
        );
    }

    #[test]
    fn grid_words_from_4_4() {
        let subject = word_grid();

        let words = subject.words(&(4, 4), 3);

        assert_eq!(
            words,
            vec![
                vec!['5', 't', 's'],
                vec!['5', '4', '3'],
                vec!['5', 'p', 'k'],
            ]
        );
    }

    #[test]
    fn grid_words_from_center() {
        let subject = word_grid();

        let words = subject.words(&(2, 2), 3);

        assert_eq!(
            words,
            vec![
                vec!['k', 'o', 's'], // down ✅
                vec!['k', 'j', 'i'], // left
                vec!['k', 'l', '3'], // right ✅
                vec!['k', 'g', 'c'], // up
                vec!['k', 'p', '5'], // down-right ✅
                vec!['k', 'n', 'q'], // down-left
                vec!['k', 'h', '1'], // up-right
                vec!['k', 'f', 'a'], // up-left
            ]
        );
    }
}
