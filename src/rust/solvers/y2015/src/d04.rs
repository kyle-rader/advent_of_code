use md5::Digest;

#[allow(dead_code)]
fn part1(input: &str) -> Option<u64> {
    let mut i: u64 = 0;
    while i < u64::MAX {
        let test = md5::Md5::digest(format!("{input}{i}").as_bytes()).to_vec();
        if starts_with_zeros(test, 5) {
            return Some(i);
        }
        i += 1;
        if i % 10_000 == 0 {
            println!("i = {i}");
        }
    }
    None
}

#[allow(dead_code)]
fn part2(input: &str) -> Option<u64> {
    None
}

fn starts_with_zeros(digest: Vec<u8>, n: u8) -> bool {
    for digit in digest.iter().take(n as usize) {
        if *digit != (0 as u8) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Some(42));
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Some(42));
    }
}

#[cfg(test)]
const INPUT: &str = "iwrupvqb";
