use std::collections::HashSet;

#[allow(dead_code)]
fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for line in input.lines() {
        sum += prrority_of_bad_item(line);
    }

    sum
}

fn prrority_of_bad_item(line: &str) -> u64 {
    let mid = line.len() / 2;
    let first: HashSet<char> = line.chars().take(mid).collect();
    let second: HashSet<char> = line.chars().rev().take(mid).collect();

    let suspect = first.intersection(&second).next();
    if let Some(suspect) = suspect {
        priority(*suspect)
    } else {
        panic!("No duplicate item found!")
    }
}

fn priority(suspect: char) -> u64 {
    (match suspect as u8 {
        n if (97..=122).contains(&n) => n - 96,
        n if (65..=90).contains(&n) => n - 38,
        _ => panic!("Bad input!"),
    }) as u64
}

#[allow(dead_code)]
fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for i in (0..lines.len()).step_by(3) {
        let a: HashSet<char> = lines[i].chars().collect();
        let b: HashSet<char> = lines[i + 1].chars().collect();
        let c: HashSet<char> = lines[i + 2].chars().collect();

        let inter1: HashSet<&char> = a.intersection(&b).collect();
        let inter2: HashSet<&char> = a.intersection(&c).collect();
        sum += priority(**inter1.intersection(&inter2).next().unwrap());
    }
    sum
}

#[allow(dead_code)]
const INPUT: &str = include_str!("d03.txt");

#[cfg(test)]
mod tests_y2022 {
    use super::*;

    #[test]
    fn priority_works() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn parse_line() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(prrority_of_bad_item(line), 16);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), 7597);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), 2607);
    }
}
