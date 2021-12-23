use std::collections::{
    hash_map::{Iter, Values},
    HashMap,
};

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
    let mut counts = Counter::new();
    template.chars().for_each(|c| counts.incr(c));

    let mut cache = Cache::new();

    let mut counts = template
        .chars()
        .tuple_windows()
        .map(|(c1, c2)| compute(c1, c2, iters, &rules, &mut cache))
        .reduce(|total, counts| {
            let mut res = total;
            res.merge(counts);
            res
        })
        .unwrap();

    template.chars().for_each(|c| counts.incr(c));

    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => Ok(max - min),
        MinMaxResult::OneElement(_) => Ok(0),
        MinMaxResult::NoElements => Err(anyhow::anyhow!("no elements")),
    }
}

type Cache = HashMap<(char, char, usize), Counter>;

/// Recursively expand the sequence (c1, c2) following the specified expansion
/// `rules` for `iters` iterations, and return a Counter representing the
/// character counts in the resulting expanded sequence.
fn compute(c1: char, c2: char, iters: usize, rules: &InsertionRules, cache: &mut Cache) -> Counter {
    if iters == 0 {
        return Counter::new();
    }
    let cache_key = (c1, c2, iters);
    if let Some(val) = cache.get(&cache_key) {
        return val.clone();
    }

    match rules.get(&(c1, c2)) {
        Some(&new) => {
            let mut res = Counter::new();
            res.incr(new);
            res.merge(compute(c1, new, iters - 1, rules, cache));
            res.merge(compute(new, c2, iters - 1, rules, cache));
            cache.insert(cache_key, res.clone());
            res
        }
        None => Counter::new(),
    }
}

#[derive(Clone)]
struct Counter(HashMap<char, usize>);

impl Counter {
    fn new() -> Counter {
        Counter { 0: HashMap::new() }
    }

    fn incr(&mut self, item: char) {
        self.incr_by(item, 1);
    }

    fn incr_by(&mut self, item: char, amount: usize) {
        let count = self.0.entry(item).or_insert(0);
        *count += amount;
    }

    fn iter(&self) -> Iter<char, usize> {
        self.0.iter()
    }

    fn values(&self) -> Values<char, usize> {
        self.0.values()
    }

    fn merge(&mut self, other: Counter) {
        for (&c, &amount) in other.iter() {
            self.incr_by(c, amount);
        }
    }
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
mod tests {
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
        assert_eq!(part2(TEST_INPUT).unwrap(), "2188189693529");
    }
}
