fn main() {
    println!("Hello, world!");
}

fn level(input: &str) -> i32 {
    input.chars().fold(0, |acc, x| match x {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("unknown input char"),
    })
}

fn first_step_into_basement(input: &str) -> Option<usize> {
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
    fn floor_level() {
        assert_eq!(level("(()(()("), 3);
    }

    #[test]
    fn find_basement() {
        assert_eq!(first_step_into_basement("()())"), Some(5));
    }

    #[test]
    fn never_find_basement() {
        assert_eq!(first_step_into_basement("()()"), None);
        assert_eq!(first_step_into_basement("()()("), None);
    }
}
