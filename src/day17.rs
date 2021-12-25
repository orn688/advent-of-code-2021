use anyhow::{anyhow, Result};
use regex::{Captures, Regex};

pub fn part1(input: &str) -> Result<String> {
    let mut region = parse_input(input)?;
    if (region.min_x..=region.max_x).contains(&0) {
        return Err(anyhow!(
            "maximal y velocity is infinite as the region sits on the y axis"
        ));
    } else if (region.min_y..=region.max_y).contains(&0) {
        return Err(anyhow!(
            "maximal y velocity is infinite as the region sits on the x axis"
        ));
    }

    // Maximal Y velocity is independent of whether initial X velocity is
    // positive or negative, so for simplicity let us assume that initial X
    // velocity is always positive.
    if region.min_x < 0 {
        region.min_x = region.max_x.abs();
        region.max_x = region.min_x.abs();
    }

    let max_y_vel = if region.min_y < 0 {
        // Since the probe's y velocity decreases by 1 each instant, it will
        // inevitably pass the start point again (0, 0) and at that time its
        // velocity will be -1 times its original y velocity, so it will next
        // move original_y_velocity+1 steps down. So to fall within the target
        // region, its max initial y velocity is the absolute value of the
        // region's bottom y value, minus 1.
        region.min_y.abs() - 1
    } else {
        // The probe will pass through the same set of points on the way up as
        // the way down, so if the region is entirely above the x axis then the
        // fastest initial y velocity will be to hit the highest edge of the
        // region in the first instant.
        region.max_y
    };

    let max_y_reached = sum_1_to(max_y_vel);

    Ok(max_y_reached.to_string())
}

fn sum_1_to(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
}

fn parse_input(input: &str) -> Result<Region> {
    let re_str = r"^target area: x=(?P<min_x>-?\d+)..(?P<max_x>-?\d+), y=(?P<min_y>-?\d+)..(?P<max_y>-d?\d+)$";
    let input_re = Regex::new(re_str).expect("failed to compile input regex");
    let caps = input_re
        .captures(input.trim())
        .ok_or(anyhow::anyhow!("input does not match regex"))?;
    let region = Region {
        min_x: cap_group_to_int(&caps, "min_x")?,
        max_x: cap_group_to_int(&caps, "max_x")?,
        min_y: cap_group_to_int(&caps, "min_y")?,
        max_y: cap_group_to_int(&caps, "max_y")?,
    };
    if region.min_x < region.max_x && region.min_y < region.max_y {
        Ok(region)
    } else {
        Err(anyhow!("invalid region: {:?}", region))
    }
}

fn cap_group_to_int(caps: &Captures, name: &str) -> Result<i32> {
    let group = caps
        .name(name)
        .ok_or_else(|| anyhow!("no group with name {}", name))?;
    match group.as_str().parse() {
        Ok(val) => Ok(val),
        Err(_) => Err(anyhow!("failed to parse group '{}' to int", group.as_str())),
    }
}

#[derive(Debug)]
struct Region {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "45");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "");
    }
}
