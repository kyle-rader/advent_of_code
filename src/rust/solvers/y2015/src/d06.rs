use std::vec;

#[allow(dead_code)]
fn part1(input: &str) -> anyhow::Result<u64> {
    let mut lights_on = vec![false; 1000 * 1000].into_boxed_slice();
    for line in input.lines() {
        let (action, start, end) = parse_line(line)?;
        for light in lights(start, end) {
            let index = (light.0 * 1000 + light.1) as usize;
            match action {
                Action::On => {
                    lights_on[index] = true;
                }
                Action::Off => {
                    lights_on[index] = false;
                }
                Action::Toggle => {
                    lights_on[index] = !lights_on[index];
                }
            }
        }
    }

    Ok(lights_on.iter().filter(|&&on| on).count() as u64)
}

#[allow(dead_code)]
fn part2(input: &str) -> anyhow::Result<i64> {
    let mut lights_brightness = vec![0 as i32; 1000 * 1000].into_boxed_slice();
    for line in input.lines() {
        let (action, start, end) = parse_line(line)?;
        for light in lights(start, end) {
            let index = (light.0 * 1000 + light.1) as usize;
            match action {
                Action::On => {
                    lights_brightness[index] += 1;
                }
                Action::Off => {
                    lights_brightness[index] = 0.max(lights_brightness[index] - 1);
                }
                Action::Toggle => {
                    lights_brightness[index] += 2;
                }
            }
        }
    }

    Ok(lights_brightness.iter().map(|&b| b as i64).sum())
}

enum Action {
    On,
    Off,
    Toggle,
}

type Instruction = (Action, Light, Light);

fn parse_line(line: &str) -> anyhow::Result<Instruction> {
    let mut words = line.split_whitespace();
    let first = words
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing first word"))?;
    match first {
        "turn" => parse_turn(&mut words),
        "toggle" => parse_toggle(&mut words),
        _ => Err(anyhow::anyhow!("unexpected first word: {}", first)),
    }
}

fn parse_turn<'a>(words: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Instruction> {
    let on_off = words
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing on/off"))?;

    let (start, end) = parse_start_end(words)?;

    match on_off {
        "on" => Ok((Action::On, start, end)),
        "off" => Ok((Action::Off, start, end)),
        _ => Err(anyhow::anyhow!("unexpected second word: {}", on_off)),
    }
}

fn parse_toggle<'a>(words: &mut impl Iterator<Item = &'a str>) -> anyhow::Result<Instruction> {
    let (start, end) = parse_start_end(words)?;
    Ok((Action::Toggle, start, end))
}

fn parse_start_end<'a>(
    words: &mut impl Iterator<Item = &'a str>,
) -> anyhow::Result<(Light, Light)> {
    let start = words
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing start"))?;
    let _ = words.next();
    let end = words.next().ok_or_else(|| anyhow::anyhow!("missing end"))?;
    Ok((parse_light(start)?, parse_light(end)?))
}

type Light = (i32, i32);

fn parse_light(input: &str) -> anyhow::Result<Light> {
    let mut parts = input.split(',');
    let x = parts.next().ok_or_else(|| anyhow::anyhow!("missing x"))?;
    let y = parts.next().ok_or_else(|| anyhow::anyhow!("missing y"))?;
    Ok((x.parse()?, y.parse()?))
}

fn lights(start: Light, end: Light) -> impl Iterator<Item = Light> {
    (start.0..=end.0).flat_map(move |x| (start.1..=end.1).map(move |y| (x, y)))
}

#[cfg(test)]
mod tests_y2015 {
    use super::*;

    const INPUT: &str = include_str!("d06.txt");

    #[test]
    fn test_parse_light() {
        assert_eq!(parse_light("0,0").unwrap(), (0, 0));
        assert_eq!(parse_light("1,2").unwrap(), (1, 2));
        assert!(parse_light("1").is_err());
    }

    #[test]
    fn test_lights() {
        let start = (0, 0);
        let end = (1, 1);
        let mut iter = lights(start, end);
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 0)));
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT).unwrap(), 569999);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT).unwrap(), 17836115);
    }
}
