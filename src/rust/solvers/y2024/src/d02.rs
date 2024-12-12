#[allow(dead_code)]
fn part1(input: &str) -> Result<usize, String> {
    Ok(input
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .filter_map(|report| report_is_safe(report.as_slice()).then_some(true))
        .filter(|safe| *safe)
        .count())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    let mut reports: Vec<(bool, Report)> = input
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .map(|report| (report_is_safe(report.as_slice()), report))
        .collect();

    for (safe, report) in &mut reports {
        if *safe {
            continue;
        }

        for j in 0..report.len() {
            // This clone is not optimal, todo: remove by creating a compound iterator of some kind to skip each element.
            let mut new_report = report.clone();
            new_report.remove(j);
            if report_is_safe(new_report.as_slice()) {
                *safe = true;
                break;
            }
        }
    }

    Ok(reports.iter().filter(|(safe, _)| *safe).count() as u64)
}

type Report = Vec<i64>;
fn report_is_safe(report: &[i64]) -> bool {
    let diffs: Vec<i64> = report
        .windows(2)
        .filter_map(|w| w[0].checked_sub(w[1]))
        .collect();

    diffs.iter().all(|d| *d >= 1 && *d <= 3) || diffs.iter().all(|d| *d <= -1 && *d >= -3)
}

fn parse_line(line: &str) -> Result<Report, String> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d02.txt");

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn parse_line_works() {
        assert_eq!(parse_line("1 2 3 4 5"), Ok(vec![1, 2, 3, 4, 5]));
        assert_eq!(
            parse_line("1 2 3 4 a 6"),
            Err("invalid digit found in string".to_string())
        );
    }

    #[test_case("7 6 4 2 1", true)]
    #[test_case("1 2 7 8 9", false)]
    #[test_case("9 7 6 2 1", false)]
    #[test_case("1 3 2 4 5", false)]
    #[test_case("8 6 4 4 1", false)]
    #[test_case("1 3 6 7 9", true)]
    fn report_is_safe_works(input: &str, expected: bool) {
        let report = parse_line(input).unwrap();
        assert_eq!(report_is_safe(&report), expected);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(287));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(354));
    }
}
