mod cube_game;

use self::cube_game::Game;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let games = Game::games_from_str(input).map_err(|e| e.to_string())?;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    Ok(games
        .iter()
        .filter(|g| g.possible(max_red, max_green, max_blue))
        .map(|g| g.id as u64)
        .sum::<u64>())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let games = Game::games_from_str(input).map_err(|e| e.to_string())?;
    Ok(games.iter().map(|g| g.power()).sum())
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;

    const INPUT: &str = include_str!("d02.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(3059));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(65371));
    }
}
