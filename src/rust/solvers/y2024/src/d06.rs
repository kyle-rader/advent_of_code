use std::{collections::HashSet, str::FromStr};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let mut map: GuardMap = input.parse()?;
    Ok(map.follow_path() as u64)
}

#[allow(dead_code)]
fn part2(_input: &str) -> Result<u64, String> {
    Ok(0)
}

const GUARD_MARKERS: [char; 4] = ['>', '<', '^', 'v'];
type Pos = (usize, usize);
type NextPosFn = Box<dyn Fn(&GuardMap) -> Option<Pos>>;

struct GuardMap {
    map: Vec<Vec<char>>,
    pos: Pos,
    path: HashSet<Pos>,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl GuardMap {
    fn pos(&self) -> Pos {
        self.pos
    }

    fn get(&self, pos: Pos) -> char {
        self.map[pos.0][pos.1]
    }

    fn update(&mut self, pos: Pos, c: char) {
        self.map[pos.0][pos.1] = c;
    }

    fn follow_path(&mut self) -> usize {
        // the starting position counts
        self.path.insert(self.pos);

        // Get the guard at the starting position
        let mut dir = Dir::from_char(self.get(self.pos));

        // Update the start position with . as we dont have to track that in the map
        self.update(self.pos, '.');

        // Init the direction_fn which, based on the current direction, gives a function to move in that direction
        let mut get_next_pos = self.next_pos_fn(&dir);

        loop {
            // Step 1: Attempt to move forward to the coords from the current direction_fn
            let Some(next_pos) = get_next_pos(self) else {
                break;
            };

            // Check what's at the next position
            match self.get(next_pos) {
                '.' => {
                    // We can move here!
                    self.pos = next_pos;
                    self.path.insert(self.pos);
                }
                '#' => {
                    // We can't move here, so we need to change direction, in our current position
                    // Update the map with the new direction
                    dir = dir.next();
                    // Update the direction function
                    get_next_pos = self.next_pos_fn(&dir);
                }
                c => panic!("Invalid character ({c}) in map"),
            }
        }

        self.path.len()
    }
    fn next_pos_fn(&self, dir: &Dir) -> NextPosFn {
        Box::new(match dir {
            Dir::Up => Self::up,
            Dir::Down => Self::down,
            Dir::Left => Self::left,
            Dir::Right => Self::right,
        })
    }

    /// Returns the position after moving up
    fn up(&self) -> Option<Pos> {
        let (row, col) = self.pos;
        if row == 0 {
            None
        } else {
            Some((row - 1, col))
        }
    }

    /// Returns the position after moving down
    fn down(&self) -> Option<Pos> {
        let (row, col) = self.pos;
        if row == self.map.len() - 1 {
            None
        } else {
            Some((row + 1, col))
        }
    }

    /// Returns the position after moving left
    fn left(&self) -> Option<Pos> {
        let (row, col) = self.pos;
        if col == 0 {
            None
        } else {
            Some((row, col - 1))
        }
    }

    /// Returns the position after moving right
    fn right(&self) -> Option<Pos> {
        let (row, col) = self.pos;
        if col == self.map[0].len() - 1 {
            None
        } else {
            Some((row, col + 1))
        }
    }
}

impl FromStr for GuardMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<char>> = vec![];
        let mut pos = (0, 0);
        for (row, line) in s.lines().enumerate() {
            let mut row_vec = vec![];
            for (col, c) in line.chars().enumerate() {
                if GUARD_MARKERS.contains(&c) {
                    pos = (row, col);
                }
                row_vec.push(c);
            }
            map.push(row_vec);
        }

        Ok(GuardMap {
            map,
            pos,
            path: HashSet::new(),
        })
    }
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const INPUT: &str = include_str!("d06.txt");

    #[test]
    fn parse_input() {
        let map: GuardMap = TEST_INPUT.parse().expect("Failed to parse input");
        assert_eq!(map.pos(), (6, 4));
    }

    #[test]
    fn follow_path() {
        let mut map: GuardMap = TEST_INPUT.parse().expect("Failed to parse input");
        assert_eq!(map.follow_path(), 41);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(4454));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(0));
    }
}
