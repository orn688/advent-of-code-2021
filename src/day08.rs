use std::collections::HashMap;

use anyhow::Result;

use lazy_static::lazy_static;

lazy_static! {
    // Length of each digit that's represented by a unique number of segments.
    static ref UNIQUE_DIGIT_LENGTHS: Vec<usize> = vec![2, 3, 4, 7];
    static ref DIGIT_SEGMENTS: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        map.insert("abcefg", 0);
        map.insert("cf", 1);
        map.insert("acdeg", 2);
        map.insert("acdfg", 3);
        map.insert("bcdf", 4);
        map.insert("abdfg", 5);
        map.insert("abdefg", 6);
        map.insert("acf", 7);
        map.insert("abcdefg", 8);
        map.insert("abcdfg", 9);
        map
    };
    #[derive(Debug)]
    static ref SEGMENT_LENGTH_SIGNATURES: HashMap<String, char> = {
        let digits = DIGIT_SEGMENTS.keys().copied().collect();
        segment_digit_lengths(digits)
    };
}

fn segment_digit_lengths(digits: Vec<&str>) -> HashMap<String, char> {
    let mut map: HashMap<char, HashMap<usize, usize>> = HashMap::new();
    for digit in digits {
        for c in digit.chars() {
            let counts = map.entry(c).or_default();
            let count = counts.entry(digit.len()).or_default();
            *count += 1;
        }
    }
    let mut inverted_map = HashMap::new();
    for (char, lengths) in map.iter() {
        let mut key_parts = lengths
            .iter()
            .map(|(length, count)| format!("{}:{}", length, count))
            .collect::<Vec<String>>();
        key_parts.sort();
        inverted_map.insert(key_parts.join(","), *char);
    }
    inverted_map
}

pub fn part1(input: &str) -> Result<String> {
    let lines = input.trim().lines();
    let count = lines
        .map(|line| line.split(" | ").last().unwrap())
        .map(|half| {
            half.split_whitespace()
                .filter(|digit| UNIQUE_DIGIT_LENGTHS.contains(&digit.len()))
                .count()
        })
        .sum::<usize>();
    Ok(count.to_string())
}

/// Takes advantange of the fact that for every line segment that makes up a part
/// of a digit, that line segment has a unique "signature". The signature is
/// defined by a counter that, for each number of per-digit segments, maps to the
/// number of digits that have that number of segments.
pub fn part2(input: &str) -> Result<String> {
    let lines = input.trim().lines();
    let mut sum = 0;
    for line in lines {
        let halves: Vec<_> = line.split(" | ").collect();
        let patterns: Vec<_> = halves[0].split_whitespace().collect();
        let mut translation: HashMap<char, char> = HashMap::new();

        for (signature, c) in segment_digit_lengths(patterns) {
            translation.insert(c, *SEGMENT_LENGTH_SIGNATURES.get(&signature).unwrap());
        }

        let mut line_sum = 0;
        for val in halves[1].split_whitespace() {
            let mut translated = val
                .chars()
                .map(|c| *translation.get(&c).unwrap())
                .collect::<Vec<char>>();
            translated.sort_unstable();
            let val = *DIGIT_SEGMENTS
                .get(translated.iter().collect::<String>().as_str())
                .unwrap();
            line_sum *= 10;
            line_sum += val;
        }
        sum += line_sum;
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "26");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "61229");
    }
}
