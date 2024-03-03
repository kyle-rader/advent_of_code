use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(input.lines().filter(|l| is_nice(l)).count() as u64)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(input.lines().filter(|l| is_nice2(l)).count() as u64)
}

fn is_nice(s: &str) -> bool {
    let mut vowels = 0;
    let mut double = false;
    let mut last = '\0';
    let mut pair = ('\0', '\0');
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => {}
        }
        if last == c {
            double = true;
        }
        last = c;
        if pair.0 == '\0' {
            pair.0 = c;
        } else if pair.1 == '\0' {
            pair.1 = c;
        } else {
            pair.0 = pair.1;
            pair.1 = c;
        }
        if pair.0 == 'a' && pair.1 == 'b'
            || pair.0 == 'c' && pair.1 == 'd'
            || pair.0 == 'p' && pair.1 == 'q'
            || pair.0 == 'x' && pair.1 == 'y'
        {
            return false;
        }
    }
    vowels >= 3 && double
}

fn is_nice2(s: &str) -> bool {
    let mut pairs: HashSet<(char, char)> = HashSet::new();
    let mut last_pair = ('\0', '\0');
    let mut last = '\0';
    let mut last_1 = '\0';

    let mut double_pair = false;
    let mut sandwich = false;

    for c in s.chars() {
        let current = (last, c);
        if pairs.contains(&current) && current != last_pair {
            double_pair = true;
        }
        pairs.insert(current);

        if last_1 == c && last != c {
            sandwich = true;
        }

        last_1 = last;
        last = c;
        last_pair = current;
    }

    double_pair && sandwich
}

#[cfg(test)]
mod tests_y2015 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d05.txt");

    #[test_case("ugknbfddgicrmopn")]
    fn test_is_nice(subject: &str) {
        assert!(is_nice(subject));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(258));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(53));
    }
}
