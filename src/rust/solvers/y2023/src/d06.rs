#[allow(dead_code)]
fn part1(input: &str) -> Result<usize, String> {
    let races = parse_input(input)?;
    Ok(races
        .iter()
        .map(|r| possible_wins(*r))
        .reduce(|a, b| a * b)
        .unwrap())
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<usize, String> {
    let race = parse_input2(input)?;
    Ok(possible_wins(race))
}

type Race = (usize, usize);

fn parse_input(input: &str) -> Result<Vec<Race>, String> {
    let mut lines = input.lines();
    let times = numbers_from_line(lines.next().ok_or("missing times row")?)?;
    let distances = numbers_from_line(lines.next().ok_or("missing distances row")?)?;
    Ok(times.into_iter().zip(distances).collect())
}

fn parse_input2(input: &str) -> Result<Race, String> {
    let mut lines = input.lines();
    let time = number_from_line(lines.next().ok_or("missing times row")?)?;
    let dist = number_from_line(lines.next().ok_or("missing distances row")?)?;
    Ok((time, dist))
}

fn numbers_from_line(line: &str) -> Result<Vec<usize>, String> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

fn number_from_line(line: &str) -> Result<usize, String> {
    let full_number: Vec<&str> = line.split_whitespace().skip(1).collect();
    let full_number = full_number.join("");
    full_number.parse::<usize>().map_err(|e| e.to_string())
}

fn possible_wins(race: Race) -> usize {
    let (time, distance) = race;
    (1..=time)
        .filter(|speed| {
            let time_remaining = time - speed;
            let traveled = speed * time_remaining;
            traveled > distance
        })
        .count()
}

#[cfg(test)]
mod tests_y2023 {
    use super::*;
    use test_case::test_case;

    const INPUT: &str = include_str!("d06.txt");
    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_inputs() {
        let expected = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(parse_input(EXAMPLE), Ok(expected));
    }

    #[test]
    fn test_inputs_2() {
        let expected = (71530, 940200);
        assert_eq!(parse_input2(EXAMPLE), Ok(expected));
    }

    #[test_case((7, 9), 4)]
    #[test_case((15, 40), 8)]
    #[test_case((30, 200), 9)]
    fn test_possible_wins(race: Race, expected: usize) {
        assert_eq!(possible_wins(race), expected)
    }

    #[test]
    fn part1_example_works() {
        assert_eq!(part1(EXAMPLE), Ok(288));
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Ok(3317888));
    }

    #[test]
    fn part2_example_works() {
        assert_eq!(part2(EXAMPLE), Ok(71503));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(24655068));
    }
}
