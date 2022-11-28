use md5::{Digest, Md5};

#[allow(dead_code)]
fn part1(input: &str) -> Option<u64> {
    find_magic_pair(input, 5)
}

#[allow(dead_code)]
fn part2(input: &str) -> Option<u64> {
    find_magic_pair(input, 6)
}

fn find_magic_pair(input: &str, n: u8) -> Option<u64> {
    let mut i: u64 = 0;
    while i < u64::MAX {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(i.to_string());
        let result = hasher.finalize().to_vec();
        if starts_with_zeros(result, n) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn starts_with_zeros(digest: Vec<u8>, n: u8) -> bool {
    digest[0] == 0
        && digest[1] == 0
        && match n {
            5 => digest[2] <= 9,
            6 => digest[2] == 0,
            _ => panic!("Can't do that large of a suffix yet!"),
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT), Some(346386));
    }

    #[ignore]
    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Some(9958218));
    }
}

#[cfg(test)]
const INPUT: &str = "iwrupvqb";
