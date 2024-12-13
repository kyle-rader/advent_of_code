use std::collections::HashSet;

use common::grid::Grid;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let target = vec!['X', 'M', 'A', 'S'];
    let mut count = 0;

    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            for word in puzzle.words(&(i, j), 4) {
                if word == target {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let puzzle = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    let possible_matches: HashSet<Vec<char>> = HashSet::from_iter([
        vec!['M', 'S', 'M', 'S'],
        vec!['M', 'S', 'S', 'M'],
        vec!['S', 'M', 'M', 'S'],
        vec!['S', 'M', 'S', 'M'],
    ]);

    for i in 1..puzzle.len() - 1 {
        for j in 1..puzzle[i].len() - 1 {
            // If current cell is 'A', check if it's got an MAS cross
            if puzzle[i][j] == 'A' {
                /*
                1.4
                .A.
                3.2
                */
                let corners = vec![
                    puzzle[i - 1][j - 1], // 1
                    puzzle[i + 1][j + 1], // 2
                    puzzle[i + 1][j - 1], // 3
                    puzzle[i - 1][j + 1], // 4
                ];

                if possible_matches.contains(&corners) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;

    const INPUT: &str = include_str!("d04.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(2534));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(1866));
    }
}
