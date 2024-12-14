use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    let mut parts = input.split("\n\n");
    let rules = parts.next().ok_or("missing rules")?;
    let updates = parts.next().ok_or("missing updates")?;

    let rules = rules.parse::<PageUpdater>()?;

    let updates: Vec<Vec<u64>> = updates
        .split("\n")
        .filter(|u| !u.is_empty())
        .map(|u| u.split(',').map(u64::from_str).collect())
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    Ok(updates
        .into_iter()
        .filter(|u| rules.valid(u))
        .map(|u| *u.get(u.len() / 2).expect("missing middle"))
        .sum())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

#[derive(Debug)]
struct PageUpdater {
    after: HashMap<u64, HashSet<u64>>,
    before: HashMap<u64, HashSet<u64>>,
}

impl PageUpdater {
    fn valid(&self, pages: &[u64]) -> bool {
        for i in 0..pages.len() {
            let page = pages[i];
            // Check if there is a rule that says the current page can only come
            // before any of the pages we've already seen.
            if let Some(pages_that_must_come_before) = self.before.get(&page) {
                if !pages
                    .iter()
                    .take(i)
                    .all(|x| !pages_that_must_come_before.contains(x))
                {
                    return false;
                }
            }
            // Check that all pages that must come after this page are after it
            if let Some(pages_that_must_come_after) = self.after.get(&page) {
                if !pages
                    .iter()
                    .skip(i + 1)
                    .all(|x| !pages_that_must_come_after.contains(x))
                {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for PageUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.before.len() == other.before.len()
            && self.after.len() == other.after.len()
            && self.before.iter().all(|(k, v)| {
                if let Some(other_v) = other.before.get(k) {
                    v.iter().all(|x| other_v.contains(x)) && v.len() == other_v.len()
                } else {
                    false
                }
            })
            && self.after.iter().all(|(k, v)| {
                if let Some(other_v) = other.after.get(k) {
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
        let mut before: HashMap<u64, HashSet<u64>> = HashMap::new();
        let mut after: HashMap<u64, HashSet<u64>> = HashMap::new();

        for line in s.lines() {
            let mut parts = line.split("|");
            let first = parts.next().ok_or("missing first page")?;
            let first = first
                .parse::<u64>()
                .map_err(|e| format!("invalid page: {}", e))?;

            let second = parts.next().ok_or("missing after page")?;
            let second = second
                .parse::<u64>()
                .map_err(|e| format!("invalid next page: {}", e))?;

            before.entry(first).or_default().insert(second);
            after.entry(second).or_default().insert(first);
        }

        Ok(Self { after, before })
    }
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;
    use test_case::test_case;

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
            after: vec![
                (13, vec![97, 61, 29, 47, 75, 53].into_iter().collect()),
                (29, vec![97, 75, 61, 47, 53].into_iter().collect()),
                (47, vec![97, 75].into_iter().collect()),
                (53, vec![97, 61, 75, 47].into_iter().collect()),
                (61, vec![97, 75, 47].into_iter().collect()),
                (75, vec![97].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
            before: vec![
                (97, vec![13, 61, 47, 29, 53, 75].into_iter().collect()),
                (75, vec![29, 53, 47, 61, 13].into_iter().collect()),
                (61, vec![13, 53, 29].into_iter().collect()),
                (29, vec![13].into_iter().collect()),
                (53, vec![29, 13].into_iter().collect()),
                (47, vec![53, 13, 61, 29].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(&rules, &expected);
    }

    #[test_case("75,47,61,53,29")]
    #[test_case("97,61,53,29,13")]
    #[test_case("75,29,13")]
    fn update_is_valid(input: &str) {
        let rules = TEST_INPUT
            .split("\n\n")
            .next()
            .expect("missing input!")
            .parse::<PageUpdater>()
            .expect("failed to parse");

        let input = input
            .split(",")
            .map(|x| x.parse::<u64>().expect("invalid input"))
            .collect::<Vec<_>>();

        assert!(rules.valid(&input));
    }

    #[test_case("75,97,47,61,53")]
    #[test_case("61,13,29")]
    #[test_case("97,13,75,29,47")]
    fn update_is_invalid(input: &str) {
        let rules = TEST_INPUT
            .split("\n\n")
            .next()
            .expect("missing input!")
            .parse::<PageUpdater>()
            .expect("failed to parse");

        let input = input
            .split(",")
            .map(|x| x.parse::<u64>().expect("invalid input"))
            .collect::<Vec<_>>();

        assert!(!rules.valid(&input));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(4766));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
