use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(Debug)]
struct PageUpdater {
    rules: HashMap<u64, HashSet<u64>>,
}

impl PartialEq for PageUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.rules.iter().all(|(k, v)| {
            if let Some(other_v) = other.rules.get(k) {
                v.iter().all(|x| other_v.contains(x)) && v.len() == other_v.len()
            } else {
                false
            }
        })
    }
}

impl Eq for PageUpdater {}

impl FromStr for PageUpdater {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules: HashMap<u64, HashSet<u64>> = HashMap::new();
        for line in s.lines() {
            let mut parts = line.split("|");
            let before = parts.next().ok_or("missing first page")?;
            let before = before
                .parse::<u64>()
                .map_err(|e| format!("invalid page: {}", e))?;

            let after = parts.next().ok_or("missing after page")?;
            let after = after
                .parse::<u64>()
                .map_err(|e| format!("invalid next page: {}", e))?;

            rules.entry(after).or_default().insert(before);
        }

        Ok(Self { rules })
    }
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;

    const INPUT: &str = include_str!("d05.txt");

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn page_rules_parsing_works() {
        let rules = TEST_INPUT
            .split("\n\n")
            .next()
            .expect("missing input!")
            .parse::<PageUpdater>()
            .expect("failed to parse");

        let expected = PageUpdater {
            rules: vec![
                (13, vec![97, 61, 29, 47, 75, 53].into_iter().collect()),
                (29, vec![97, 75, 61, 47, 53].into_iter().collect()),
                (47, vec![97, 75].into_iter().collect()),
                (53, vec![97, 61, 75, 47].into_iter().collect()),
                (61, vec![97, 75, 47].into_iter().collect()),
                (75, vec![97].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(&rules, &expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(42));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
