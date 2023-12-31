use self::engine::Schematic;

mod engine;

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let schematic = Schematic::from(input);
    Ok(schematic.part_numbers()?.iter().sum())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;

    const INPUT: &str = include_str!("d03.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(530495));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
