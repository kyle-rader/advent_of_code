use common::banana;

fn part1(input: &str) -> i32 {
    input.chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("unknown input char"),
    })
}

fn part2(input: &str) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unknown input char"),
        }

        if floor < 0 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1("(()(()("), 3);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("()())"), Some(5));
    }
}
