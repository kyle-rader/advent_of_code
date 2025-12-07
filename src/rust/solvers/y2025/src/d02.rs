#[allow(dead_code)]
fn part1(input: &str) -> Result<u64, String> {
    sum_invalid_ids(is_invalid_id, input)
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    sum_invalid_ids(is_invalid_id_2, input)
}

fn invalid_ids(
    ranges: Vec<std::ops::RangeInclusive<u64>>,
    is_invalid_fn: impl Fn(&str) -> bool,
) -> Result<Vec<u64>, String> {
    ranges
        .into_iter()
        .flat_map(|r| r.map(|i| i.to_string()))
        .filter(|i| is_invalid_fn(i))
        .map(|i| i.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn sum_invalid_ids(is_invalid_fn: impl Fn(&str) -> bool, input: &str) -> Result<u64, String> {
    let ranges = puzzle_data(input).map_err(|e| e.to_string())?;
    let invalid_ids = invalid_ids(ranges, is_invalid_fn)?;
    Ok(invalid_ids.into_iter().sum())
}

fn puzzle_data(input: &str) -> anyhow::Result<Vec<std::ops::RangeInclusive<u64>>> {
    let data = input
        .trim()
        .split(',')
        .filter(|r| !r.is_empty())
        .map(parse_range)
        .collect::<Result<Vec<std::ops::RangeInclusive<u64>>, _>>()?;
    Ok(data)
}

fn parse_range(range: &str) -> anyhow::Result<std::ops::RangeInclusive<u64>> {
    let mut parts = range.trim().split('-');
    let start = parts.next().ok_or(anyhow::anyhow!("Missing start"))?;
    let end = parts.next().ok_or(anyhow::anyhow!("Missing end"))?;
    let start = start
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("Invalid start '{start}'"))?;
    let end = end
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("Invalid end '{end}'"))?;
    Ok(start..=end)
}

fn is_invalid_id(id: &str) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }
    let mid = id.len() / 2;
    id[..mid] == id[mid..]
}

/// Check for repeats, by iterating over the different chunks of the input and checking if all chunks are the same.
/// The smallest chunk size is 1, the largest, is half the length of the input.
fn is_invalid_id_2(id: &str) -> bool {
    let chars: Vec<char> = id.chars().collect();
    let max_chunk_size = id.len() / 2;
    // Only test chunk sizes that fit evenly into the input.
    let chunk_sizes =
        (1..=max_chunk_size).filter(|chunk_size| id.len().is_multiple_of(*chunk_size));
    chunk_sizes.into_iter().any(|chunk_size| {
        let mut chunks = chars.chunks_exact(chunk_size);
        let mut all_same = true;
        let first = chunks.next().expect("Expected at least one chunk");
        for chunk in chunks {
            if chunk != first {
                all_same = false;
                break;
            }
        }
        all_same
    })
}

#[cfg(test)]
mod tests_y2025 {
    use super::*;
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case("11")]
    #[test_case("22")]
    #[test_case("1212")]
    #[test_case("123123")]
    #[test_case("202202")]
    fn can_identify_invalid_ids(subject: &str) {
        assert!(is_invalid_id(subject));
    }

    #[test_case("11")]
    #[test_case("111")]
    #[test_case("1111")]
    #[test_case("202202")]
    #[test_case("123123123")]
    #[test_case("2222222222")]
    fn can_identify_invalid_ids_2(subject: &str) {
        assert!(is_invalid_id_2(subject));
    }

    const INPUT: &str = include_str!("d02.txt");
    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test_case("11-22", 11..=22)]
    #[test_case("95-115", 95..=115)]
    #[test_case("998-1012", 998..=1012)]
    #[test_case("1188511880-1188511890", 1188511880..=1188511890)]
    #[test_case("222220-222224", 222220..=222224)]
    #[test_case("1698522-1698528", 1698522..=1698528)]
    #[test_case("446443-446449", 446443..=446449)]
    #[test_case("38593856-38593862", 38593856..=38593862)]
    #[test_case("565653-565659", 565653..=565659)]
    fn parse_range_works(subject: &str, expected: std::ops::RangeInclusive<u64>) {
        assert_eq!(parse_range(subject).unwrap(), expected);
    }

    #[test]
    fn puzzle_data_works() {
        let subject = puzzle_data(TEST_INPUT).expect("failed to parse puzzle data");
        assert_eq!(
            subject,
            vec![
                11..=22,
                95..=115,
                998..=1012,
                1188511880..=1188511890,
                222220..=222224,
                1698522..=1698528,
                446443..=446449,
                38593856..=38593862,
                565653..=565659,
                824824821..=824824827,
                2121212118..=2121212124
            ]
        );
    }

    #[test]
    fn invalid_ids_counts_match_example() {
        let ranges = puzzle_data(TEST_INPUT).expect("failed to parse puzzle data");
        let actual = invalid_ids(ranges, is_invalid_id_2).expect("failed to get invalid IDs");

        // Expected invalid IDs from the example:
        // 11-22: 11, 22
        // 95-115: 99, 111
        // 998-1012: 999, 1010
        // 1188511880-1188511890: 1188511885
        // 222220-222224: 222222
        // 1698522-1698528: (none)
        // 446443-446449: 446446
        // 38593856-38593862: 38593859
        // 565653-565659: 565656
        // 824824821-824824827: 824824824
        // 2121212118-2121212124: 2121212121
        let expected = vec![
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), Ok(4174379265));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(40398804950));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(65794984339));
    }
}
