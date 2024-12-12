use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char, digit1, space0},
    combinator::{map_res, opt},
    multi::{many0, many_till},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

#[allow(dead_code)]
fn part1(input: &str) -> IResult<&str, i64> {
    Ok(("", mul_matches(input)?.1.iter().sum()))
}

#[allow(dead_code)]
fn part2(input: &str) -> Result<u64, String> {
    Ok(0)
}

fn mul_matches(input: &str) -> IResult<&str, Vec<i64>> {
    let (_, instructions) = parse_instructions(input)?;

    Ok((
        "",
        instructions
            .iter()
            .filter_map(|instr| match instr {
                Instruction::Mul(a, b) => Some(a * b),
                _ => None,
            })
            .collect(),
    ))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(parse_instruction)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_chars, inst)) =
        many_till(anychar, alt((parse_mul, parse_do, parse_dont)))(input)?;
    Ok((input, inst))
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, (a, b)) = delimited(
        char('('),
        separated_pair(parse_i64, char(','), parse_i64),
        char(')'),
    )(input)?;
    Ok((input, Instruction::Mul(a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = opt(space0)(input)?;
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = opt(space0)(input)?;
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
    Garbage,
}

#[cfg(test)]
mod tests_y2024 {
    use super::*;

    const INPUT: &str = include_str!("d03.txt");
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parse_instructions_works() {
        let (input, subject) = parse_instructions(TEST_INPUT).expect("parsing test input failed");
        assert_eq!(input, ")"); // last char is unprocessed
        assert_eq!(
            subject,
            vec![
                Instruction::Mul(2, 4),
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ]
        );
    }

    #[test]
    fn parse_instructions_with_modifiers() {
        let (input, subject) = parse_instructions(TEST_INPUT2).expect("parsing test input failed");
        assert_eq!(input, ")"); // last char is unprocessed
        assert_eq!(
            subject,
            vec![
                Instruction::Mul(2, 4),
                Instruction::Dont,
                Instruction::Mul(5, 5),
                Instruction::Mul(11, 8),
                Instruction::Mul(8, 5)
            ]
        );
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT).expect("parsing failed").1, 173419328);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT), Ok(42));
    }
}
