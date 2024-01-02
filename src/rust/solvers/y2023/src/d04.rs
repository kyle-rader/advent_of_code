use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(input.lines().map(score_card).sum::<u64>())
}

fn parse_card(line: &str) -> (HashSet<u64>, HashSet<u64>) {
    let mut parts = line.split('|');
    let winning = parts
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let winning = HashSet::from_iter(winning);
    let have = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());
    let have = HashSet::from_iter(have);
    (winning, have)
}

fn score_card(line: &str) -> u64 {
    let (winning, have) = parse_card(line);
    match winning.intersection(&have).count() {
        0 => 0,
        n => 2_u64.pow(n as u32 - 1),
    }
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let card_matches: Vec<usize> = input
        .lines()
        .map(parse_card)
        .map(|(winning, have)| winning.intersection(&have).count())
        .collect();
    let queue: Vec<usize> = (0..card_matches.len()).collect();
    let mut total = card_matches.len() as u64;
    let mut cache: HashMap<usize, u64> = HashMap::new();

    // Seed the cache with cards that have 0 matches, as those will not recurse.
    for (i, matches) in card_matches.iter().enumerate() {
        if *matches == 0 {
            cache.insert(i, 0);
        }
    }

    for card in queue {
        total += part2_score_card(card, &card_matches, &mut cache);
    }

    Ok(total)
}

fn part2_score_card(
    card: usize,
    card_matches: &Vec<usize>,
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    if let Some(score) = cache.get(&card) {
        return *score;
    }

    let matches = card_matches[card];
    let cards_won: Vec<usize> = (card..=(card + matches)).skip(1).collect();

    let mut total = matches as u64;
    for card in cards_won {
        total += part2_score_card(card, card_matches, cache);
    }

    cache.insert(card, total);
    total
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d04.txt");

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), Ok(13));
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE), Ok(30));
    }

    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[test_case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[test_case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[test_case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[test_case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[test_case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_score_card(line: &str, expected: u64) {
        assert_eq!(score_card(line), expected);
    }

    #[test]
    fn parse_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (winning, have) = parse_card(line);
        assert_eq!(winning, HashSet::from_iter(vec![41, 48, 83, 86, 17]));
        assert_eq!(have, HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(25183));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(5667240));
    }
}
