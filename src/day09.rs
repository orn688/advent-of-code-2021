use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let grid = parse_input(input);
    let mut sum = 0;
    for (row, vals) in grid.grid.iter().enumerate() {
        for (col, val) in vals.iter().enumerate() {
            let neighbors = grid.neighbor_vals(row as i32, col as i32);
            if !neighbors.is_empty() && neighbors.iter().all(|&n| n > val) {
                sum += val + 1;
            }
        }
    }

    Ok(sum.to_string())
}

pub fn part2(_: &str) -> Result<String> {
    Ok(String::new())
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    Grid { grid }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn val(&self, row: i32, col: i32) -> Option<&i32> {
        self.grid.get(row as usize)?.get(col as usize)
    }

    fn neighbor_vals(&self, row: i32, col: i32) -> Vec<&i32> {
        let coords = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];
        coords
            .iter()
            .filter_map(|(r, c)| self.val(*r, *c))
            .collect()
    }
}

#[allow(dead_code)]
const TEST_INPUT: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), "15");
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT).unwrap(), "");
}
