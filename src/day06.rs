const TOTAL_DAYS: i32 = 80;
const VALUE_AFTER_BIRTH: i32 = 6;
const NEW_FISH_VALUE: i32 = 8;

pub fn part1(input: &str) -> Result<String, String> {
    let mut fish = vec![];
    parse_input(input, &mut fish);
    for _ in 0..TOTAL_DAYS {
        let mut new_fish = vec![];
        for f in fish {
            if f == 0 {
                new_fish.push(VALUE_AFTER_BIRTH);
                new_fish.push(NEW_FISH_VALUE);
            } else {
                new_fish.push(f - 1);
            }
        }
        fish = new_fish;
    }
    Ok(fish.len().to_string())
}

pub fn part2(_: &str) -> Result<String, String> {
    Ok(String::new())
}

fn parse_input(input: &str, fish: &mut Vec<i32>) {
    fish.extend(
        input
            .trim()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .into_iter(),
    );
}

#[allow(dead_code)]
const TEST_INPUT: &str = "3,4,3,1,2";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "5934");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
