mod cubes;

use self::cubes::{Game, GameError};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let games = input
        .lines()
        .map(|line| line.parse::<Game>())
        .collect::<Result<Vec<Game>, GameError>>()
        .map_err(|e| e.to_string())?;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    Ok(games
        .iter()
        .filter(|g| possible_game(&g, max_red, max_green, max_blue))
        .map(|g| g.id as u64)
        .sum::<u64>())
}

fn possible_game(game: &Game, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    game.red <= max_red && game.green <= max_green && game.blue <= max_blue
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;

    const INPUT: &str = include_str!("d02.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(42));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
