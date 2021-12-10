use std::{collections::HashMap, ops::Index};

use anyhow::{Context, Result};

/// Given a list of binary numbers, produces two new numbers of the same length:
/// one where each binary digit is the *most* common digit at the corresponding
/// index across all the input numbers, and another where each binary digit is
/// the *least* common digit at the corresponding index across all the input
/// numbers.
///
/// Then returns the product of the resulting two numbres as a decimal.
pub fn part1(input: &str) -> Result<String> {
    let lines = input.trim().split_whitespace();
    let mut length = 0;
    let mut ones_counts: HashMap<usize, i64> = HashMap::new();
    for line in lines {
        length += 1;
        for (i, ch) in line.chars().enumerate() {
            let count = ones_counts.entry(i).or_insert(0);
            if ch == '1' {
                *count += 1;
            }
        }
    }
    let (mut least_common, mut most_common) = (0, 0);
    for i in 0..ones_counts.len() {
        least_common <<= 1;
        most_common <<= 1;
        let ones_count = ones_counts
            .get(&i)
            .ok_or_else(|| anyhow::anyhow!("no count for {}", i))?;
        if ones_count > &(length / 2) {
            most_common |= 1;
        } else {
            least_common |= 1;
        }
    }
    Ok((most_common * least_common).to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let lines: Vec<Vec<char>> = input
        .trim()
        .split_whitespace()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut most_common = lines.to_vec();
    let mut i = 0;
    while most_common.len() > 1 {
        let ones: Vec<Vec<char>> = most_common
            .iter()
            .filter(|l| *l.index(i) == '1')
            .cloned()
            .collect();
        let zeroes: Vec<Vec<char>> = most_common
            .iter()
            .filter(|l| *l.index(i) == '0')
            .cloned()
            .collect();
        if ones.len() >= zeroes.len() {
            most_common = ones
        } else {
            most_common = zeroes
        }
        i += 1;
    }

    let mut least_common = lines.to_vec();
    let mut i = 0;
    while least_common.len() > 1 {
        let ones: Vec<Vec<char>> = least_common
            .iter()
            .filter(|l| *l.index(i) == '1')
            .cloned()
            .collect();
        let zeroes: Vec<Vec<char>> = least_common
            .iter()
            .filter(|l| *l.index(i) == '0')
            .cloned()
            .collect();
        if ones.len() >= zeroes.len() {
            least_common = zeroes
        } else {
            least_common = ones
        }
        i += 1;
    }

    let m: String = most_common.index(0).iter().collect();
    let l: String = least_common.index(0).iter().collect();

    Ok((binary_string_to_int(&m)? * binary_string_to_int(&l)?).to_string())
}

fn binary_string_to_int(s: &str) -> Result<isize> {
    isize::from_str_radix(s, 2).context("failed to parse binary string")
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "198");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "230");
}
