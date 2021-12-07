use std::collections::HashMap;

const VALUE_AFTER_BIRTH: i32 = 6;
const NEW_FISH_VALUE: i32 = 8;

pub fn part1(input: &str) -> Result<String, String> {
    Ok(population_after_days(input, 80).to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(population_after_days(input, 256).to_string())
}

fn population_after_days(input: &str, days: i32) -> usize {
    let fish = parse_input(input);
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for f in fish.into_iter() {
        incr_count(&mut counts, f, 1);
    }
    for _ in 0..days {
        let mut new_counts = HashMap::new();
        for (f, count) in counts.into_iter() {
            if f > 0 {
                incr_count(&mut new_counts, f - 1, count);
            } else {
                incr_count(&mut new_counts, NEW_FISH_VALUE, count);
                incr_count(&mut new_counts, VALUE_AFTER_BIRTH, count);
            }
        }
        counts = new_counts;
    }
    counts.values().sum()
}

fn incr_count(counter: &mut HashMap<i32, usize>, k: i32, amount: usize) {
    let val = counter.entry(k).or_insert(0);
    *val += amount;
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "5934");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "26984457539");
}
