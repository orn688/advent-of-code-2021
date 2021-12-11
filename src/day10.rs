use std::collections::HashMap;

use anyhow::Result;

/// Checks for lines containing an invalid sequence of open/close parentheses,
/// e.g. "[{})", excluding lines that are incomplete but otherwise valid, e.g.
/// "[{}", and computes a score based on the first invalid character in each
/// line.
pub fn part1(input: &str) -> Result<String> {
    let mut close_scores = HashMap::new();
    close_scores.insert(')', 3);
    close_scores.insert(']', 57);
    close_scores.insert('}', 1197);
    close_scores.insert('>', 25137);

    let mut score = 0;
    for line in input.trim().lines() {
        let mut stack = String::new();
        for c in line.chars() {
            if close_scores.contains_key(&c) {
                let invalid = if let Some(c1) = stack.pop() {
                    c1 != c
                } else {
                    true
                };
                if invalid {
                    score += close_scores.get(&c).unwrap();
                }
            } else {
                stack.push(close_char(c));
            }
        }
    }

    Ok(score.to_string())
}

fn close_char(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid open char {}", open),
    }
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "26397");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
