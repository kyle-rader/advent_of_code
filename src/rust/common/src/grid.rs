pub trait Grid {
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
}

impl<T> Grid for Vec<Vec<T>> {
    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
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
        let neighbors = subject.neighbors(row, col);
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
        let neighbors = subject.neighbors(row, col);
        assert_eq!(neighbors, expected.to_vec())
    }

    #[test]
    fn neighbors_empty() {
        let subject: Vec<Vec<char>> = vec![];
        let neighbors = subject.neighbors(5, 5);
        assert_eq!(neighbors, vec![])
    }
}
