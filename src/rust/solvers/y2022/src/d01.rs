#[allow(dead_code)]
fn part1(input: &str) -> u64 {
    *parse_elves(input).iter().max().unwrap()
}

#[allow(dead_code)]
fn part2(input: &str) -> u64 {
    let mut calories: Vec<u64> = parse_elves(input);
    calories.sort();
    calories.iter().rev().take(3).sum()
}

fn parse_elves(input: &str) -> Vec<u64> {
    input
        .split_terminator("\n\n")
        .map(|cals| {
            cals.split_whitespace()
                .filter_map(|c| c.parse::<u64>().ok())
                .sum()
        })
        .collect()
}

#[allow(dead_code)]
const INPUT: &str = include_str!("d01.txt");

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 71502);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 208191);
    }
}
