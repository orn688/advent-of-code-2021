use std::collections::HashMap;

use anyhow::Result;
use itertools::{Itertools, MinMaxResult};

pub fn part1(input: &str) -> Result<String> {
    let (template, rules) = parse_input(input)?;
    Ok(counts_after_iterations(template, rules, 10)?.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let (template, rules) = parse_input(input)?;
    Ok(counts_after_iterations(template, rules, 40)?.to_string())
}

fn counts_after_iterations(template: String, rules: InsertionRules, iters: usize) -> Result<usize> {
    let mut counts: Counter = Counter::new();
    let mut prev = None;

    for c in template.chars() {
        incr_count(&mut counts, c);
        if let Some(prev) = prev {
            let mut stack = vec![(prev, c, iters)];
            while let Some((a, b, remaining)) = stack.pop() {
                if remaining == 0 {
                    continue;
                }
                if let Some(&new) = rules.get(&(a, b)) {
                    incr_count(&mut counts, new);
                    stack.push((a, new, remaining - 1));
                    stack.push((new, b, remaining - 1));
                }
            }
        }
        prev = Some(c);
    }

    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => Ok(max - min),
        MinMaxResult::OneElement(_) => Ok(0),
        MinMaxResult::NoElements => Err(anyhow::anyhow!("no elements")),
    }
}

type Counter = HashMap<char, usize>;

fn incr_count(counter: &mut Counter, item: char) {
    let count = counter.entry(item).or_insert(0);
    *count += 1;
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

    // TODO: enable this test after translating the optimize DP solution from
    // Python into Rust.
    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "2188189693529");
    }
}
