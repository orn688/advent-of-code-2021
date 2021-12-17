use std::collections::HashMap;

use anyhow::Result;
use itertools::{Itertools, MinMaxResult};

pub fn part1(input: &str) -> Result<String> {
    let (mut template, rules) = parse_input(input)?;

    for _ in 0..10 {
        let mut prev = None;
        let mut new_template = String::new();
        for c in template.chars() {
            if let Some(prev) = prev {
                if let Some(&new) = rules.get(&(prev, c)) {
                    new_template.push(new)
                }
            }
            new_template.push(c);
            prev = Some(c);
        }
        template = new_template;
    }

    let counts = template.chars().counts();
    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => Ok((max - min).to_string()),
        MinMaxResult::OneElement(_) => Ok(0.to_string()),
        MinMaxResult::NoElements => Err(anyhow::anyhow!("no elements")),
    }
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
}

type InsertionRules = HashMap<(char, char), char>;

fn parse_input(input: &str) -> Result<(String, InsertionRules)> {
    let lines: Vec<_> = input.trim().lines().collect();
    let template = lines[0];

    let mapping = HashMap::from_iter(lines[2..lines.len()].iter().map(|line| {
        let chars: Vec<_> = line.chars().collect();
        ((chars[0], chars[1]), chars[6])
    }));
    Ok((template.to_string(), mapping))
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "1588");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "");
    }
}
