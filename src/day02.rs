use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let moves = parse_input(input)?;
    let mut horiz = 0;
    let mut depth = 0;
    for mv in moves {
        match mv.direction.as_str() {
            "forward" => horiz += mv.count,
            "down" => depth += mv.count,
            "up" => depth -= mv.count,
            _ => return Err(anyhow::anyhow!("Invalid direction: {}", mv.direction)),
        }
    }
    Ok((horiz * depth).to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let moves = parse_input(input)?;
    let (mut horiz, mut depth, mut aim) = (0, 0, 0);

    for mv in moves {
        match mv.direction.as_str() {
            "forward" => {
                horiz += mv.count;
                depth += aim * mv.count;
            }
            "down" => aim += mv.count,
            "up" => aim -= mv.count,
            _ => return Err(anyhow::anyhow!("Invalid direction: {}", mv.direction)),
        }
    }
    Ok((horiz * depth).to_string())
}

struct Move {
    direction: String,
    count: i32,
}

fn parse_input(input: &str) -> Result<Vec<Move>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            Ok(Move {
                direction: parts[0].into(),
                count: parts[1].parse()?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "150");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "900");
    }
}
