/// Counts the number of times a depth measurement increases from the previous
/// one.
pub fn part1(input: &str) -> Result<String, String> {
    let mut num_increasing = 0;
    let mut prev = -1;
    for depth in parse_input(input)? {
        if prev >= 0 && depth > prev {
            num_increasing += 1;
        }
        prev = depth
    }
    Ok(num_increasing.to_string())
}

/// Counts the number of times a depth measurement increases from one window of
/// three entries to the next.
pub fn part2(input: &str) -> Result<String, String> {
    let window_size = 3;
    let mut num_increasing = 0;
    let mut prev_sum = -1;
    let depths = parse_input(input)?;
    for window in depths.windows(window_size) {
        let sum = window.iter().sum();
        if prev_sum >= 0 && sum > prev_sum {
            num_increasing += 1;
        }
        prev_sum = sum;
    }
    Ok(num_increasing.to_string())
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    let numbers: Vec<i32> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    Ok(numbers)
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
199
200
208
210
200
207
240
269
260
263
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "7");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "5");
}
