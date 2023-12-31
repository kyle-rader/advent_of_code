pub trait Grid {
    fn neighbors_single(&self, row: usize, col: usize) -> Vec<(usize, usize)>;
    fn neighbors_range(&self, row: usize, start: usize, end: usize) -> Vec<(usize, usize)>;
}

impl<T> Grid for Vec<Vec<T>> {
    fn neighbors_single(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let neighbors = vec![
            (row.checked_sub(1), col.checked_sub(1)),
            (row.checked_sub(1), Some(col)),
            (row.checked_sub(1), Some(col + 1)),
            (Some(row), col.checked_sub(1)),
            // Skip the center
            (Some(row), Some(col + 1)),
            (Some(row + 1), col.checked_sub(1)),
            (Some(row + 1), Some(col)),
            (Some(row + 1), Some(col + 1)),
        ];

        // Filter out any neighbors that are out of bounds from checked subtractions
        let neighbors = neighbors
            .into_iter()
            .filter_map(|(row, col)| match (row, col) {
                (Some(row), Some(col)) => Some((row, col)),
                _ => None,
            });

        // Filter out any neighbors that are out of bounds by going to far in the positive direction
        neighbors
            .filter_map(|(row, col)| {
                if row >= self.len() || col >= self[row].len() {
                    None
                } else {
                    Some((row, col))
                }
            })
            .collect()
    }

    fn neighbors_range(&self, row: usize, start: usize, end: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
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
        let right = end + 1;
        if right < self[row].len() {
            neighbors.push((row, right));
        }

        neighbors
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
    #[test_case(0, 2, &[(0, 1), (0, 3), (1, 1), (1, 2), (1, 3)])]
    /*
      0 1 2 3 4
    0 . x x x .
    1 . x ? x .
    2 . x x x .
     */
    #[test_case(1, 2, &[(0, 1), (0, 2), (0, 3), (1, 1), (1, 3), (2, 1), (2, 2), (2, 3)])]
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
    #[test_case(0, 0, &[(0, 1), (1, 0), (1, 1)])]
    fn neighbors(row: usize, col: usize, expected: &[(usize, usize)]) {
        let subject = grid(5, 3, '.');
        let neighbors = subject.neighbors_single(row, col);
        assert_eq!(neighbors, expected.to_vec())
    }

    /*
      0 1 2 3
    0 x x x .
    1 x ?
    2 x x x
     */
    #[test_case(1, 1, &[(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (2, 1), (2, 2)])]
    /*
      0 1 2 3
    0 x x . .
    1 ? x
    2 x x .
     */
    #[test_case(1, 0, &[(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)])]
    fn neighbors_jagged(row: usize, col: usize, expected: &[(usize, usize)]) {
        let subject = vec![
            vec!['x', 'x', 'x', '.'],
            vec!['x', '?'],
            vec!['x', 'x', 'x'],
        ];
        let neighbors = subject.neighbors_single(row, col);
        assert_eq!(neighbors, expected.to_vec())
    }

    #[test]
    fn neighbors_empty() {
        let subject: Vec<Vec<char>> = vec![];
        let neighbors = subject.neighbors_single(5, 5);
        assert_eq!(neighbors, vec![])
    }
}
