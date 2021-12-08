pub fn part1(input: &str) -> Result<String, String> {
    let mut nums = parse_input(input);
    let med = median(&mut nums);
    let fuel: i32 = nums.iter().map(|x| (med - x).abs()).sum();
    Ok(fuel.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let nums = parse_input(input);
    let max = *nums.iter().max().unwrap();
    let min = *nums.iter().min().unwrap();

    let mut min_fuel: Option<i32> = None;
    for target in min..=max {
        let fuel = nums.iter().map(|&num| sum_1_to((target - num).abs())).sum();
        min_fuel = match min_fuel {
            None => Some(fuel),
            Some(current_min) => Some(current_min.min(fuel)),
        };
    }
    Ok(min_fuel.unwrap().to_string())
}

fn sum_1_to(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn median(nums: &mut Vec<i32>) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    if len % 2 == 1 {
        nums[len / 2]
    } else {
        (nums[len / 2 - 1] + nums[len / 2]) / 2
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "37");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "168");
}
