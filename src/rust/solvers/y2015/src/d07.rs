#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[cfg(test)]
mod tests_y2015 {
    use super::*;

    const INPUT: &str = include_str!("d07.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(42));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
