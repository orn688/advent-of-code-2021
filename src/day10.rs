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

/// Checks for lines that have un-closed sequences of parentheses but are
/// otherwise valid, e.g. "[{}", and calculates a score for each such line based
/// on the characters that would need to be added to complete the line. Then
/// returns the median of all those scores.
pub fn part2(input: &str) -> Result<String> {
    let mut close_scores: HashMap<_, i64> = HashMap::new();
    close_scores.insert(')', 1);
    close_scores.insert(']', 2);
    close_scores.insert('}', 3);
    close_scores.insert('>', 4);

    let mut all_scores = vec![];
    for line in input.trim().lines() {
        // stack contains the minimum sequence of chars necessary to close all
        // sequences in the line (in reverse order).
        let mut stack = String::new();
        let mut line_invalid = false;
        for c in line.chars() {
            if close_scores.contains_key(&c) {
                let invalid = if let Some(c1) = stack.pop() {
                    c1 != c
                } else {
                    true
                };
                if invalid {
                    line_invalid = true;
                    break;
                }
            } else {
                stack.push(close_char(c));
            }
        }
        if line_invalid {
            continue;
        }
        let mut score = 0;
        for c in stack.chars().rev() {
            score *= 5;
            score += close_scores.get(&c).unwrap();
        }
        all_scores.push(score);
    }

    all_scores.sort_unstable();
    let mid_score = all_scores[all_scores.len() / 2];

    Ok(mid_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

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
        assert_eq!(part2(TEST_INPUT).unwrap(), "288957");
    }
}
