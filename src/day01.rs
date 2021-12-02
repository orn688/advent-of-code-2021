/// Counts the number of times a depth measurement increases from the previous
/// one.
pub fn part1(input: String) -> Result<String, String> {
    let mut num_increasing = 0;
    let mut prev = -1;
    for line in input.trim().lines() {
        let depth: i32 = line.parse().unwrap();
        if prev >= 0 && depth > prev {
            num_increasing += 1;
        }
        prev = depth
    }
    Ok(num_increasing.to_string())
}

#[test]
fn test_part1() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + "\n" + &b)
        .unwrap();
    assert_eq!(part1(input).unwrap(), "7");
}
