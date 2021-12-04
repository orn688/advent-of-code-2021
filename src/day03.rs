use std::collections::HashMap;

pub fn part1(input: &str) -> Result<String, String> {
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
        if ones_counts.get(&i).unwrap() > &(length / 2) {
            most_common |= 1;
        } else {
            least_common |= 1;
        }
    }
    Ok((most_common * least_common).to_string())
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
